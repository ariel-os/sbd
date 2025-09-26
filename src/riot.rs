use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;

use anyhow::{Result, anyhow};
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use crate::filemap::{Mode, parse_mode};
use crate::{
    filemap::FileMap,
    parse_sbd_files,
    sbd::{Board, SbdFile},
};

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "generate-riot")]
/// generate RIOT OS specific files
pub struct GenerateRiotArgs {
    /// the name of the directory containing board descriptions
    #[argh(positional)]
    sbd_dir: String,

    /// operation mode: create|check|update
    #[argh(option, short = 'm', from_str_fn(parse_mode))]
    mode: Option<Mode>,

    /// riot os external boards output dir
    #[argh(option, short = 'o', default = "Utf8PathBuf::from(\"riot-os-boards\")")]
    output: Utf8PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Riot {
    pub chips: BTreeMap<String, RiotChipMapEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotChipMapEntry {
    cpu: String,
    cpu_model: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    quirks: BTreeMap<String, RiotQirkEntry>,
    peripherals: Option<RiotChipPeripherals>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotBoardExt {
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotQirkEntry {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotChipPeripherals {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    uarts: BTreeMap<String, RiotChipUartPeripheral>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotChipUartPeripheral {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    config: BTreeMap<String, String>,
    isr: Option<String>,
}

struct RiotBoard {
    pub name: String,
    pub files: FileMap,
}
impl RiotBoard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            files: FileMap::new(),
        }
    }
}

#[derive(Default)]
struct CFile {
    pub pragma_once: bool,
    pub cplusplus_guards: bool,
    pub includes: Vec<String>,
    pub content_snips: Vec<String>,
}

impl CFile {
    #[expect(dead_code, reason = "currently unused, unmark when it is")]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new_header() -> Self {
        Self {
            pragma_once: true,
            cplusplus_guards: true,
            ..Default::default()
        }
    }

    pub fn render(&self) -> String {
        let mut s = String::new();
        if self.pragma_once {
            s.push_str("#pragma once\n");
        }
        if !self.includes.is_empty() {
            s.push('\n');
            for include in &self.includes {
                let _ = writeln!(s, "#include {include}");
            }
        }
        if self.cplusplus_guards {
            s.push_str("\n#ifdef __cplusplus\nextern \"C\" {\n#endif\n\n");
        }
        for snip in &self.content_snips {
            s.push_str(snip);
        }
        if self.cplusplus_guards {
            s.push_str("#ifdef __cplusplus\n}\n#endif\n");
        }
        s
    }
}

pub fn generate(args: &GenerateRiotArgs) -> Result<()> {
    let sbd_file = parse_sbd_files(args.sbd_dir.as_str())?;
    let mode = args.mode.unwrap_or_default();

    let boards_dir = render_riot_boards_dir(&sbd_file)?;

    mode.apply(&args.output, &boards_dir)?;

    Ok(())
}

pub fn render_riot_boards_dir(sbd: &SbdFile) -> Result<FileMap> {
    let chips: HashSet<String> = sbd
        .riot
        .clone()
        .unwrap_or_default()
        .chips
        .keys()
        .cloned()
        .collect::<HashSet<_>>();

    if chips.is_empty() {
        println!("warning: No supported chips defined for RIOT OS");
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
        println!("warning: No boards defined for Riot OS");
    }

    let mut riot_boards = Vec::new();

    for board in boards {
        riot_boards.push(generate_riot_board(sbd, &board)?);
    }

    let mut riot_boards_dir = FileMap::new();
    for board in riot_boards {
        riot_boards_dir.extend_subdir(&Utf8PathBuf::from(board.name), board.files);
    }

    Ok(riot_boards_dir)
}

#[allow(clippy::too_many_lines)]
fn generate_riot_board(sbd: &SbdFile, board: &Board) -> Result<RiotBoard> {
    let mut riot_board = RiotBoard::new(&board.name);

    let mut periph_conf_h = CFile::new_header();
    let mut board_h = CFile::new_header();

    let mut makefile = String::new();
    let makefile_dep = String::new();
    let mut makefile_features = String::new();
    let mut makefile_include = String::new();

    let mut features = HashSet::<String>::new();

    // generate base headers
    periph_conf_h.includes.push("\"kernel_defines.h\"".into());
    periph_conf_h.includes.push("\"periph_cpu.h\"".into());

    board_h.includes.push("\"cpu.h\"".into());

    // populate makefiles
    makefile.push_str("MODULE = board");

    // both unwraps should always succeed (filtered in caller)
    let riot_chip = sbd.riot.as_ref().unwrap().chips.get(&board.chip).unwrap();
    let _ = writeln!(makefile_features, "CPU = {}", riot_chip.cpu);
    let _ = writeln!(makefile_features, "CPU_MODEL = {}", riot_chip.cpu_model);

    // handle file quirks
    let quirk_file_map = [
        ("periph_conf.h", &mut periph_conf_h),
        ("board.h", &mut board_h),
    ];

    for (filename, file_obj) in quirk_file_map {
        if let Some(quirk) = riot_chip.quirks.get(filename) {
            for snip in &quirk.body {
                file_obj.content_snips.push(snip.to_string());
            }
        }
    }

    let mut uarts = Vec::new();

    // Debugger
    if let Some(debugger) = &board.debugger {
        let _ = writeln!(makefile_include, "PROGRAMMER ?= {}", debugger.type_);
        if let Some(uart) = &debugger.uart {
            uarts.push(uart);
        }
    }

    // UARTs
    //let mut uart_isrs = Vec::new();
    uarts.extend(board.uarts.iter().flatten());
    let mut uart_peripherals = riot_chip.peripherals.as_ref().unwrap().uarts.clone();
    let mut uarts_configured = Vec::new();
    for uart in uarts {
        // get the map key of the peripheral that works for this UART's pins
        #[expect(clippy::unnecessary_find_map)]
        let peripheral_key = uart_peripherals
            .iter()
            .find_map(|(key, _peripheral)| {
                // TODO: actually do filter/select by possible pins
                Some(key)
            })
            .map(std::string::ToString::to_string)
            .clone();

        if let Some(key) = peripheral_key {
            let uart_peripheral = uart_peripherals.remove(&key).unwrap();
            let mut uart_cfg = uart_peripheral.config.clone();
            let rx_pin = name2riot_pin(&uart.rx_pin)?;
            let tx_pin = name2riot_pin(&uart.tx_pin)?;
            uart_cfg.insert("rx_pin".into(), rx_pin);
            uart_cfg.insert("tx_pin".into(), tx_pin);

            uarts_configured.push((uart_cfg, uart_peripheral.isr));
        } else {
            println!(
                "warning: {}: no peripheral found for UART {}",
                board.name,
                uart.name.as_ref().map_or_else(|| "unnamed", |s| s)
            );
        }
    }

    if !uarts_configured.is_empty() {
        let mut s = String::new();

        s.push('\n');

        // generate cfg struct
        {
            s.push_str("static const uart_conf_t uart_config[] = {\n");
            for (uart_cfg, _) in &uarts_configured {
                s.push_str("    {\n");
                for (k, v) in uart_cfg {
                    let _ = writeln!(s, "        .{k} = {v},");
                }
                s.push_str("    },\n");
            }
            s.push_str("};\n\n");
        }

        for (n, (_, isr)) in uarts_configured.into_iter().enumerate() {
            if let Some(isr) = isr {
                let _ = writeln!(s, "#define UART_{n}_ISR          ({isr})");
            }
        }

        s.push_str("#define UART_NUMOF          ARRAY_SIZE(uart_config)\n\n");

        periph_conf_h.content_snips.push(s);

        features.insert("periph_uart".into());
    }

    // finishing
    if !features.is_empty() {
        for feature in features {
            let _ = writeln!(makefile_features, "FEATURES_PROVIDED += {feature}");
        }
    }

    makefile.push_str("\ninclude $(RIOTBASE)/Makefile.base\n");

    riot_board
        .files
        .insert("include/periph_conf.h".into(), periph_conf_h.render());
    riot_board
        .files
        .insert("include/board.h".into(), board_h.render());
    riot_board.files.insert("Makefile".into(), makefile);
    riot_board.files.insert("Makefile.dep".into(), makefile_dep);
    riot_board
        .files
        .insert("Makefile.features".into(), makefile_features);
    riot_board
        .files
        .insert("Makefile.include".into(), makefile_include);

    Ok(riot_board)
}

fn name2riot_pin(gpio_name: &str) -> Result<String> {
    let (port, pin) = crate::pin2tuple::parse_gpio_name(gpio_name)
        .ok_or_else(|| anyhow!("error parsing GPIO name: {gpio_name}"))?;

    Ok(format!("GPIO_PIN({port}, {pin})"))
}
