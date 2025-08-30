use std::collections::{BTreeMap, BTreeSet, HashSet};

use anyhow::Result;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use crate::{
    krate::{Crate, DependencyFull},
    laze::{LazeContext, LazeFile, StringOrVecString},
    parse_sbd_files,
    sbd::{Board, Button, Led, SbdFile},
};

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "generate-ariel")]
/// generate Ariel OS specific files
pub struct GenerateArielArgs {
    /// the name of the directory containing board descriptions
    #[argh(positional)]
    sbd_dir: String,

    /// ariel os boards crate output folder
    #[argh(option, short = 'o', default = "String::from(\"ariel-os-boards\")")]
    output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Ariel {
    pub socs: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ArielBoardExt {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub flags: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub global_env: BTreeMap<String, StringOrVecString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swi: Option<String>,
}

pub fn generate(args: GenerateArielArgs) -> Result<()> {
    let sbd_file = parse_sbd_files(args.sbd_dir.as_str())?;

    // Finally, render the ariel crate.
    render_ariel_board_crate(&sbd_file, args.output.as_str().into())?;

    Ok(())
}

pub fn render_ariel_board_crate(sbd: &SbdFile, out: &Utf8Path) -> Result<()> {
    let mut board_crate = Crate::new("ariel-os-boards");

    let socs: HashSet<String> = HashSet::from_iter(
        sbd.ariel
            .clone()
            .unwrap_or_default()
            .socs
            .iter()
            .flatten()
            .cloned(),
    );

    if socs.is_empty() {
        println!("warning: No SoCs defined for Ariel OS");
    }

    let mut init_body = String::new();

    // filter boards with unknown SoCs
    let boards = sbd
        .boards
        .iter()
        .flatten()
        .filter(|board| {
            if socs.contains(&board.soc) {
                true
            } else {
                println!(
                    "warning: skipping board {}, unknown SoC {}",
                    board.name, board.soc
                );
                false
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    if boards.is_empty() {
        println!("warning: No boards defined for Ariel OS");
    }

    // crate
    {
        board_crate.manifest.package.edition = Some(crate::krate::Edition::workspace());
        board_crate.manifest.package.rust_version = Some(crate::krate::RustVersion::workspace());

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

        board_crate
            .files
            .insert("src/pins.rs".into(), render_pins_rs(&boards));

        board_crate
            .files
            .insert("build.rs".into(), render_build_rs(&boards));

        handle_crate_quirks(&boards, &mut init_body);

        let lib_rs = format!(
            "#![no_std]\n\npub mod pins;\npub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {{\n{init_body}}}\n"
        );

        board_crate.files.insert("src/lib.rs".into(), lib_rs);
    }

    // laze file
    {
        let mut laze_file = LazeFile::new();
        let mut laze_builders = Vec::new();
        for board in boards {
            let mut board_builder = LazeContext::new(&board.name);
            board_builder.parent = Some(board.soc.clone());
            board_builder.provides.extend(board.flags.clone());
            board_builder.provides.extend(board.ariel.flags.clone());

            if board.has_leds() {
                board_builder.provides.insert("has_leds".into());
            }
            if board.has_buttons() {
                board_builder.provides.insert("has_buttons".into());
            }

            if let Some(swi) = board.ariel.swi {
                board_builder.provides.insert("has_swi".into());

                let config_swi = format!("CONFIG_SWI={swi}");

                board_builder
                    .env
                    .entry("CARGO_ENV".into())
                    .or_insert_with(|| StringOrVecString::VecString(Vec::new()))
                    .push(config_swi);
            }

            // copy over Ariel's global environment
            board_builder.env.extend(board.ariel.global_env.clone());

            laze_builders.push(board_builder);
        }
        laze_file.builders = Some(laze_builders);

        // add to crate
        board_crate
            .files
            .insert("laze.yml".into(), laze_file.to_string().unwrap());
    }

    board_crate.write_to_directory(out)?;
    Ok(())
}

pub fn render_build_rs(boards: &[Board]) -> String {
    let mut build_rs = String::new();

    build_rs.push_str("pub fn main() {\n");

    for board in boards {
        build_rs.push_str(&format!(
            "    println!(\"cargo::rustc-check-cfg=cfg(context, values(\\\"{}\\\"))\");\n",
            board.name
        ))
    }

    build_rs.push_str("}\n");

    build_rs
}

fn handle_crate_quirks(boards: &[Board], init_body: &mut String) {
    for board in boards {
        for quirk in &board.quirks {
            match quirk {
                crate::sbd::Quirk::SetPin(set_pin_op) => {
                    handle_set_bin_op(&board.name, set_pin_op, init_body);
                }
            }
        }
    }
}

fn handle_set_bin_op(board_name: &str, set_pin_op: &crate::sbd::SetPinOp, init_body: &mut String) {
    let mut code = String::new();
    code.push_str(&format!("#[cfg(context = \"{board_name}\")]\n"));
    code.push_str("{\n");
    if let Some(description) = &set_pin_op.description {
        code.push_str(&format!("    // {}\n", description));
    }
    code.push_str(&format!(
        "    let pin = peripherals.{}.take().unwrap();\n",
        set_pin_op.pin
    ));

    code.push_str(&format!(
        "    let output = ariel_os_hal::gpio::Output::new(pin, {});\n",
        match set_pin_op.level {
            crate::sbd::PinLevel::High => "ariel_os_embassy_common::gpio::Level::High",
            crate::sbd::PinLevel::Low => "ariel_os_embassy_common::gpio::Level::Low",
        }
    ));
    code.push_str("    core::mem::forget(output);\n");
    code.push_str("}\n");

    init_body.push_str(&code);
}

fn render_pins_rs(boards: &[Board]) -> String {
    let mut pins_rs = String::new();
    pins_rs.push_str("use ariel_os_hal::hal::peripherals;\n\n");
    for board in boards {
        if let Some(leds) = board.leds.as_ref() {
            pins_rs.push_str(&render_led_pins(&board.name, leds));
        }
        if let Some(buttons) = board.buttons.as_ref() {
            pins_rs.push_str(&render_button_pins(&board.name, buttons));
        }
    }

    pins_rs
}

fn render_led_pins(board: &str, leds: &[Led]) -> String {
    let mut leds_rs = String::new();

    leds_rs.push_str(&format!("#[cfg(context = \"{board}\")]\n"));
    leds_rs.push_str("ariel_os_hal::define_peripherals!(LedPeripherals {\n");

    for led in leds {
        leds_rs.push_str(&format!("{}: {},\n", led.name, led.pin));
    }

    leds_rs.push_str("});\n");

    leds_rs
}

fn render_button_pins(board: &str, buttons: &[Button]) -> String {
    let mut buttons_rs = String::new();

    buttons_rs.push_str(&format!("#[cfg(context = \"{board}\")]\n"));
    buttons_rs.push_str("ariel_os_hal::define_peripherals!(ButtonPeripherals {\n");

    for button in buttons {
        buttons_rs.push_str(&format!("{}: {},\n", button.name, button.pin));
    }

    buttons_rs.push_str("});\n");

    buttons_rs
}
