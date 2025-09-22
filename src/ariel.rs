//! Ariel OS board support crate generation

// Conventions:
// - functions rendering a whole file are named render_<file_name>_<ext>, e.g., `render_board_rs`
// - functions rendering a part of a file are named render_<somename>, e.g., `render_board_rs_init_body`
//
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Write as _,
    str::FromStr,
};

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

    /// overwrite existing files
    #[argh(option, short = 'm', from_str_fn(parse_mode))]
    mode: Option<Mode>,

    /// ariel os boards crate output folder
    #[argh(option, short = 'o', default = "String::from(\"ariel-os-boards\")")]
    output: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Mode {
    #[default]
    Create,
    Overwrite,
    Check,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Mode::Create),
            "overwrite" => Ok(Mode::Overwrite),
            "check" => Ok(Mode::Check),
            _ => Err(format!("Invalid mode: {s}")),
        }
    }
}

fn parse_mode(s: &str) -> Result<Mode, String> {
    Mode::from_str(s)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Ariel {
    pub chips: Option<Vec<String>>,
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

pub fn generate(args: &GenerateArielArgs) -> Result<()> {
    let sbd_file = parse_sbd_files(args.sbd_dir.as_str())?;
    let mode = args.mode.unwrap_or_default();

    // Finally, render the ariel crate.
    render_ariel_board_crate(&sbd_file, args.output.as_str().into(), mode)?;

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn render_ariel_board_crate(sbd: &SbdFile, out: &Utf8Path, mode: Mode) -> Result<()> {
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

    // filter boards with unknown chips
    let boards = sbd
        .boards
        .iter()
        .flatten()
        .filter(|board| {
            if chips.contains(&board.chip) {
                true
            } else {
                println!(
                    "warning: skipping board {}, unknown chip {}",
                    board.name, board.chip
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
            .insert("build.rs".into(), render_build_rs(&boards));

        for board in &boards {
            let board_rs = render_board_rs(board);
            board_crate
                .files
                .insert(format!("src/{}.rs", board.name).into(), board_rs);
        }

        let mut lib_rs = String::new();
        lib_rs.push_str("// @generated\n\n#![no_std]\n\n");

        lib_rs.push_str(&render_boards_dispatch(&boards));

        board_crate.files.insert("src/lib.rs".into(), lib_rs);
    }

    // laze file
    {
        let mut laze_file = LazeFile::new();
        let mut laze_builders = Vec::new();
        for board in boards {
            let mut board_builder = LazeContext::new(&board.name);
            board_builder.parent = Some(board.chip.clone());
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

        let mut laze_file_str = String::from("# yamllint disable-file\n\n");
        laze_file_str.push_str(&laze_file.to_string().unwrap());

        // add to crate
        board_crate.files.insert("laze.yml".into(), laze_file_str);
    }

    board_crate.write_to_directory(out, mode == Mode::Overwrite)?;
    Ok(())
}

fn render_boards_dispatch(boards: &[Board]) -> String {
    let mut s = String::new();

    s.push_str("cfg_if::cfg_if! {\n   ");
    for board in boards {
        let board_name = &board.name;
        let _ = writeln!(s, " if #[cfg(context = \"{board_name}\")] {{");
        let _ = writeln!(s, "        include!(\"{board_name}.rs\");");
        s.push_str("    } else");
    }
    s.push_str(" {\n");
    s.push_str("        // TODO\n");
    s.push_str("    }\n");

    s.push_str("}\n");

    s
}

fn render_board_rs(board: &Board) -> String {
    let pins = render_pins(board);

    let mut init_body = String::new();
    handle_quirks(board, &mut init_body);

    let board_rs = format!(
        "// @generated\n\n{pins}\n#[allow(unused_variables)]\npub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {{\n{init_body}}}\n"
    );

    board_rs
}

pub fn render_build_rs(boards: &[Board]) -> String {
    let mut build_rs = String::new();

    build_rs.push_str("// @generated\n");
    build_rs.push_str("pub fn main() {\n");

    for board in boards {
        let _ = writeln!(
            build_rs,
            "    println!(\"cargo::rustc-check-cfg=cfg(context, values(\\\"{}\\\"))\");",
            board.name
        );
    }

    build_rs.push_str("}\n");

    build_rs
}

fn handle_quirks(board: &Board, init_body: &mut String) {
    for quirk in &board.quirks {
        match quirk {
            crate::sbd::Quirk::SetPin(set_pin_op) => {
                handle_set_bin_op(set_pin_op, init_body);
            }
        }
    }
}

fn handle_set_bin_op(set_pin_op: &crate::sbd::SetPinOp, init_body: &mut String) {
    let mut code = String::new();
    code.push_str("{\n");
    if let Some(description) = &set_pin_op.description {
        let _ = writeln!(code, "    // {description}");
    }

    let _ = writeln!(
        code,
        "    let pin = peripherals.{}.take().unwrap();",
        set_pin_op.pin
    );

    let _ = writeln!(
        code,
        "    let output = ariel_os_hal::gpio::Output::new(pin, {});",
        match set_pin_op.level {
            crate::sbd::PinLevel::High => "ariel_os_embassy_common::gpio::Level::High",
            crate::sbd::PinLevel::Low => "ariel_os_embassy_common::gpio::Level::Low",
        }
    );

    code.push_str("    core::mem::forget(output);\n");
    code.push_str("}\n");

    init_body.push_str(&code);
}

fn render_pins(board: &Board) -> String {
    let mut pins = String::new();

    pins.push_str("pub mod pins {\n");

    if board.has_leds() || board.has_buttons() {
        pins.push_str("use ariel_os_hal::hal::peripherals;\n\n");
        if let Some(leds) = board.leds.as_ref() {
            pins.push_str(&render_led_pins(&board.name, leds));
        }
        if let Some(buttons) = board.buttons.as_ref() {
            pins.push_str(&render_button_pins(&board.name, buttons));
        }
    }

    pins.push_str("}\n");

    pins
}

fn render_led_pins(board: &str, leds: &[Led]) -> String {
    let mut leds_rs = String::new();

    let _ = writeln!(leds_rs, "#[cfg(context = \"{board}\")]");
    leds_rs.push_str("ariel_os_hal::define_peripherals!(LedPeripherals {\n");

    for led in leds {
        let _ = writeln!(leds_rs, "{}: {},", led.name, led.pin);
    }

    leds_rs.push_str("});\n");

    leds_rs
}

fn render_button_pins(board: &str, buttons: &[Button]) -> String {
    let mut buttons_rs = String::new();

    let _ = writeln!(buttons_rs, "#[cfg(context = \"{board}\")]");
    buttons_rs.push_str("ariel_os_hal::define_peripherals!(ButtonPeripherals {\n");

    for button in buttons {
        let _ = writeln!(buttons_rs, "{}: {},", button.name, button.pin);
    }

    buttons_rs.push_str("});\n");

    buttons_rs
}
