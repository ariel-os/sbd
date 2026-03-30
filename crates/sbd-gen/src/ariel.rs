//! Ariel OS board support crate generation

// Conventions:
// - functions rendering a whole file are named render_<file_name>_<ext>, e.g., `render_target_rs`
// - functions rendering a part of a file are named render_<somename>, e.g., `render_target_rs_init_body`
//
use std::{collections::HashSet, fmt::Write as _};

use anyhow::{Context as _, Result, anyhow};
use camino::Utf8PathBuf;

use crate::{
    filemap::{FileMap, Mode, parse_mode},
    krate::{Crate, DependencyFull},
    laze::{LazeContext, LazeFile},
    parse_sbd_files,
    resources::Resources,
};

use sbd_gen_schema::{PinLevel, Quirk, SbdFile, SetPinOp, Target, common::StringOrVecString};

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "generate-ariel")]
/// generate Ariel OS specific files
pub struct GenerateArielArgs {
    /// the name of the directory containing board descriptions
    #[argh(positional)]
    sbd_dir: String,

    /// operation mode: create|check|update
    #[argh(option, short = 'm', from_str_fn(parse_mode))]
    mode: Option<Mode>,

    /// ariel os boards crate output folder
    #[argh(
        option,
        short = 'o',
        default = "Utf8PathBuf::from(\"ariel-os-boards\")"
    )]
    output: Utf8PathBuf,
}

pub fn generate(args: &GenerateArielArgs) -> Result<()> {
    let sbd_file = parse_sbd_files(args.sbd_dir.as_str())?;
    let mode = args.mode.unwrap_or_default();

    // Render the ariel crate.
    let krate = render_ariel_board_crate(&sbd_file)?;

    mode.apply(&args.output, &krate)?;

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn render_ariel_board_crate(sbd: &SbdFile) -> Result<FileMap> {
    let mut board_crate = Crate::new("ariel-os-boards");

    let chips: HashSet<String> = sbd
        .ariel
        .clone()
        .unwrap_or_default()
        .chips
        .iter()
        .flatten()
        .cloned()
        .collect();

    if chips.is_empty() {
        println!("warning: No chips defined for Ariel OS");
    }

    // filter targets with unknown chips
    let targets = sbd
        .targets
        .iter()
        .flatten()
        .filter(|target| {
            if chips.contains(&target.chip) {
                true
            } else {
                println!(
                    "warning: skipping target {}, unknown chip {}",
                    target.name, target.chip
                );
                false
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    if targets.is_empty() {
        println!("warning: No targets defined for Ariel OS");
    }

    // crate
    {
        use crate::krate::workspace;
        board_crate.manifest.package.edition = workspace();
        board_crate.manifest.package.license = workspace();
        board_crate.manifest.package.rust_version = workspace();

        // features
        board_crate
            .manifest
            .features
            .insert("no-boards".into(), Vec::new());

        // deps
        board_crate.manifest.dependencies.insert(
            "ariel-os-hal".into(),
            crate::krate::Dependency::Full(DependencyFull {
                workspace: Some(true),
                ..Default::default()
            }),
        );

        board_crate.manifest.dependencies.insert(
            "ariel-os-embassy-common".into(),
            crate::krate::Dependency::Full(DependencyFull {
                workspace: Some(true),
                ..Default::default()
            }),
        );

        board_crate.manifest.dependencies.insert(
            "cfg-if".into(),
            crate::krate::Dependency::Full(DependencyFull {
                workspace: Some(true),
                ..Default::default()
            }),
        );

        board_crate
            .files
            .insert("build.rs".into(), render_build_rs(&targets));

        for target in &targets {
            let target_rs = render_target_rs(target)
                .with_context(|| anyhow!("cannot render {}", target.name))?;
            board_crate
                .files
                .insert(format!("src/{}.rs", target.name).into(), target_rs);
        }

        let mut lib_rs = String::new();
        lib_rs.push_str("// @generated\n\n#![no_std]\n\n");

        lib_rs.push_str(&render_targets_dispatch(&targets));

        board_crate.files.insert("src/lib.rs".into(), lib_rs);
    }

    // laze file
    {
        let mut laze_file = LazeFile::new();
        let mut laze_builders = Vec::new();
        for target in targets {
            let mut target_builder = LazeContext::new(&target.name);
            target_builder.parent = Some(target.chip.clone());
            target_builder.provides.extend(target.flags.clone());
            target_builder.provides.extend(target.ariel.flags.clone());

            if target.has_leds() {
                target_builder.provides.insert("has_leds".into());
            }
            if target.has_buttons() {
                target_builder.provides.insert("has_buttons".into());
            }
            if target.has_host_facing_uart() {
                target_builder
                    .provides
                    .insert("has_host_facing_uart".into());
            }

            if let Some(swi) = target.ariel.swi {
                target_builder.provides.insert("has_swi".into());

                let config_swi = format!("CONFIG_SWI={swi}");

                target_builder
                    .env
                    .entry("CARGO_ENV".into())
                    .or_insert_with(|| StringOrVecString::VecString(Vec::new()))
                    .push(config_swi);
            }

            // copy over Ariel's global environment
            target_builder.env.extend(target.ariel.global_env.clone());

            laze_builders.push(target_builder);
        }
        laze_file.builders = Some(laze_builders);

        let mut laze_file_str = String::from("# yamllint disable-file\n\n");
        laze_file_str.push_str(&laze_file.to_string().unwrap());

        // add to crate
        board_crate.files.insert("laze.yml".into(), laze_file_str);
    }

    Ok(board_crate.render())
}

fn render_targets_dispatch(targets: &[Target]) -> String {
    let mut s = String::new();

    s.push_str("cfg_if::cfg_if! {\n");
    for target in targets {
        let target_name = &target.name;
        let _ = writeln!(s, "if #[cfg(context = \"{target_name}\")] {{");
        let _ = writeln!(s, "    include!(\"{target_name}.rs\");");
        s.push_str("} else ");
    }
    s.push_str("{\n");
    s.push_str("    // TODO: handle unexpected context\n");
    s.push_str("}\n");

    s.push_str("}\n");

    s
}

struct RenderTarget<'a> {
    target: &'a Target,
    resources: Resources<'a>,
}

impl<'a> RenderTarget<'a> {
    pub fn new(target: &'a Target) -> Self {
        let resources = Resources::new(target);
        Self { target, resources }
    }

    pub fn render_pins(&mut self) -> Result<String> {
        let mut pins = String::new();

        pins.push_str("pub mod pins {\n");
        let target = self.target;

        if target.has_leds() || target.has_buttons() || target.has_uarts() {
            pins.push_str("use ariel_os_hal::hal::peripherals;\n\n");
            if target.has_leds() {
                pins.push_str(&self.render_led_pins()?);
            }
            if target.has_buttons() {
                pins.push_str(&self.render_button_pins()?);
            }
            if target.uarts.is_some() {
                pins.push_str(&self.render_uarts()?);
            }
        }

        pins.push_str("}\n");

        Ok(pins)
    }

    fn render_led_pins(&mut self) -> Result<String> {
        let leds = &self.target.leds;
        let mut leds_rs = String::new();

        leds_rs.push_str("ariel_os_hal::define_peripherals!(LedPeripherals {\n");

        for (n, led) in leds.iter().enumerate() {
            let name = format!("led{n}");
            self.resources.claim(&led.pin, &name)?;
            let _ = writeln!(leds_rs, "{}: {},", name, led.pin);
        }

        leds_rs.push_str("});\n");

        Ok(leds_rs)
    }

    fn render_button_pins(&mut self) -> Result<String> {
        let buttons = &self.target.buttons;
        let mut buttons_rs = String::new();

        buttons_rs.push_str("ariel_os_hal::define_peripherals!(ButtonPeripherals {\n");

        for (n, button) in buttons.iter().enumerate() {
            let name = format!("button{n}");
            self.resources.claim(&button.pin, &name)?;
            let _ = writeln!(buttons_rs, "{}: {},", name, button.pin);
        }

        buttons_rs.push_str("});\n");

        Ok(buttons_rs)
    }

    fn render_uarts(&mut self) -> Result<String> {
        let uarts = self.target.uarts.as_ref().unwrap();
        let mut code = String::new();

        code.push_str("ariel_os_hal::define_uarts![\n");

        for (uart_number, uart) in uarts.iter().enumerate() {
            let name = uart.name.as_ref().map_or_else(
                || format!("_unnamed_uart_{uart_number}").into(),
                std::borrow::Cow::from,
            );

            {
                // claim this UART's resources
                // TODO: "by" could be more specific ("claimed by uart FOO as rx_pin" vs "claimed
                // by uart FOO")
                self.resources.claim(&uart.rx_pin, &name)?;
                self.resources.claim(&uart.tx_pin, &name)?;

                if let Some(ref cts_pin) = uart.cts_pin {
                    self.resources.claim(cts_pin, &name)?;
                }
                if let Some(ref rts_pin) = uart.rts_pin {
                    self.resources.claim(rts_pin, &name)?;
                }

                // Note: We claim uart "device" later, after actually figuring out which one to use.
            }

            let Some(device) = uart.possible_peripherals.first() else {
                eprintln!(
                    "warning: No peripheral defined for UART, making it unusable in Ariel output."
                );
                eprintln!("Affected UART: {uart:?}");
                continue;
            };
            if uart.possible_peripherals.len() > 1 {
                eprintln!(
                    "warning: Multiple hardware devices are available, but Ariel OS does not process any but the first."
                );
                eprintln!("Affected UART: {uart:?}");
            }

            // claiming uart "device" here
            self.resources.claim(device, &name)?;

            // Deferring to a macro so that any actual logic in there is handled in the OS where it
            // belongs; this merely processes the data into a format usable there.
            writeln!(
                code,
                "{{ name: {}, device: {}, tx: {}, rx: {}, host_facing: {} }},",
                name, device, uart.tx_pin, uart.rx_pin, uart.host_facing
            )
            .unwrap();
        }

        code.push_str("];\n");

        Ok(code)
    }
}

fn render_target_rs(target: &Target) -> Result<String> {
    let mut render_target = RenderTarget::new(target);
    let pins = render_target.render_pins()?;

    let mut init_body = String::new();
    handle_quirks(target, &mut init_body);

    let target_rs = format!(
        "// @generated\n\n{pins}\n#[allow(unused_variables)]\npub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {{\n{init_body}}}\n"
    );

    Ok(target_rs)
}

pub fn render_build_rs(targets: &[Target]) -> String {
    let mut build_rs = String::new();

    build_rs.push_str("// @generated\n");
    build_rs.push_str("pub fn main() {\n");

    for target in targets {
        let _ = writeln!(
            build_rs,
            "println!(\"cargo::rustc-check-cfg=cfg(context, values(\\\"{}\\\"))\");",
            target.name
        );
    }

    build_rs.push_str("}\n");

    build_rs
}

fn handle_quirks(target: &Target, init_body: &mut String) {
    for quirk in &target.quirks {
        match quirk {
            Quirk::SetPin(set_pin_op) => {
                handle_set_bin_op(set_pin_op, init_body);
            }
        }
    }
}

fn handle_set_bin_op(set_pin_op: &SetPinOp, init_body: &mut String) {
    let mut code = String::new();
    code.push_str("{\n");
    if let Some(description) = &set_pin_op.description {
        let _ = writeln!(code, "// {description}");
    }

    let _ = writeln!(
        code,
        "let pin = peripherals.{}.take().unwrap();",
        set_pin_op.pin
    );

    let _ = writeln!(
        code,
        "let output = ariel_os_hal::gpio::Output::new(pin, {});",
        match set_pin_op.level {
            PinLevel::High => "ariel_os_embassy_common::gpio::Level::High",
            PinLevel::Low => "ariel_os_embassy_common::gpio::Level::Low",
        }
    );

    code.push_str("    core::mem::forget(output);\n");
    code.push_str("}\n");

    init_body.push_str(&code);
}

#[cfg(test)]
#[must_use]
pub fn test_default_target() -> Target {
    Target {
        name: "test-target".to_string(),
        ariel: sbd_gen_schema::ariel::ArielTargetExt::default(),
        buttons: None,
        chip: "test-chip".to_string(),
        debugger: None,
        description: None,
        leds: None,
        flags: std::collections::BTreeSet::default(),
        include: None,
        uarts: None,
        quirks: vec![],
        riot: sbd_gen_schema::riot::RiotTargetExt::default(),
    }
}

#[test]
fn test_render_uarts() {
    use sbd_gen_schema::Uart;
    let uarts = Some(vec![
        Uart {
            name: Some("CON0".to_string()),
            rx_pin: "PA08".to_owned(),
            tx_pin: "PC99".to_owned(),
            cts_pin: None,
            rts_pin: None,
            possible_peripherals: vec!["UART2".to_owned(), "LEUART0".to_owned()],
            host_facing: false,
        },
        Uart {
            name: Some("VCOM".to_string()),
            rx_pin: "P0_04".to_owned(),
            tx_pin: "P1_23".to_owned(),
            cts_pin: Some("P7.89".to_owned()),
            rts_pin: Some("D5".to_owned()),
            possible_peripherals: vec!["UART1".to_owned(), "LEUART0".to_owned()],
            host_facing: true,
        },
    ]);

    let target = Target {
        uarts,
        ..test_default_target()
    };

    let mut render_target = RenderTarget::new(&target);

    let rendered = render_target.render_uarts().unwrap();
    assert_eq!(
        rendered,
        "ariel_os_hal::define_uarts![
{ name: CON0, device: UART2, tx: PC99, rx: PA08, host_facing: false },
{ name: VCOM, device: UART1, tx: P1_23, rx: P0_04, host_facing: true },
];
"
    );
}
