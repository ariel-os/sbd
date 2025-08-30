use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use anyhow::{Result, anyhow};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::{
    parse_sbd_files,
    sbd::{Board, Button, Led, SbdFile},
    utils::write_all,
};

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "generate-riot")]
/// generate RIOT OS specific files
pub struct GenerateRiotArgs {
    /// the name of the directory containing board descriptions
    #[argh(positional)]
    sbd_dir: String,

    /// riot os external boards output dir
    #[argh(option, short = 'o', default = "String::from(\"riot-os-boards\")")]
    output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Riot {
    pub socs: BTreeMap<String, RiotSocMapEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotSocMapEntry {
    cpu: String,
    cpu_model: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    quirks: BTreeMap<String, RiotQirkEntry>,
    peripherals: Option<RiotSocPeripherals>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotBoardExt {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub flags: BTreeSet<String>,
    // #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    // pub global_env: BTreeMap<String, StringOrVecString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotQirkEntry {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotSocPeripherals {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    uarts: BTreeMap<String, RiotSocUartPeripheral>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotSocUartPeripheral {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    config: BTreeMap<String, String>,
    isr: Option<String>,
}

struct RiotBoard {
    pub name: String,
    pub files: HashMap<Utf8PathBuf, String>,
}
impl RiotBoard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            files: HashMap::new(),
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

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if self.pragma_once {
            s.push_str("#pragma once\n");
        }
        if !self.includes.is_empty() {
            s.push('\n');
            for include in &self.includes {
                s.push_str(&format!("#include {}\n", include));
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

pub fn generate(args: GenerateRiotArgs) -> Result<()> {
    let sbd_file = parse_sbd_files(args.sbd_dir.as_str())?;

    render_riot_boards_dir(&sbd_file, args.output.as_str().into())?;

    Ok(())
}

pub fn render_riot_boards_dir(sbd: &SbdFile, out: &Utf8Path) -> Result<()> {
    let socs: HashSet<String> =
        HashSet::from_iter(sbd.riot.clone().unwrap_or_default().socs.keys().cloned());

    if socs.is_empty() {
        println!("warning: No supported SoCs defined for RIOT OS");
    }

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
        println!("warning: No boards defined for Riot OS");
    }

    let mut riot_boards = Vec::new();

    for board in boards {
        riot_boards.push(generate_riot_board(sbd, &board)?);
    }

    for board in riot_boards {
        let board_dir = out.join(&board.name);

        write_all(&board_dir, board.files.iter())?;
    }

    Ok(())
}

fn generate_riot_board(sbd: &SbdFile, board: &Board) -> Result<RiotBoard> {
    let mut riot_board = RiotBoard::new(&board.name);

    let mut periph_conf_h = CFile::new_header();
    let mut board_h = CFile::new_header();

    let mut makefile = String::new();
    let mut makefile_dep = String::new();
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
    let riot_soc = sbd.riot.as_ref().unwrap().socs.get(&board.soc).unwrap();
    makefile_features.push_str(&format!("CPU = {}\n", riot_soc.cpu));
    makefile_features.push_str(&format!("CPU_MODEL = {}\n", riot_soc.cpu_model));

    // handle file quirks
    let quirk_file_map = [
        ("periph_conf.h", &mut periph_conf_h),
        ("board.h", &mut board_h),
    ];

    for (filename, file_obj) in quirk_file_map {
        if let Some(quirk) = riot_soc.quirks.get(filename) {
            for snip in &quirk.body {
                file_obj.content_snips.push(snip.to_string());
            }
        }
    }

    let mut uarts = Vec::new();

    // Debugger
    if let Some(debugger) = &board.debugger {
        makefile_include.push_str(&format!("PROGRAMMER ?= {}\n", debugger._type));
        if let Some(uart) = &debugger.uart {
            uarts.push(uart);
        }
    }

    // UARTs
    //let mut uart_isrs = Vec::new();
    uarts.extend(board.uarts.iter().flatten());
    let mut uart_peripherals = riot_soc.peripherals.as_ref().unwrap().uarts.clone();
    let mut uarts_configured = Vec::new();
    for uart in uarts {
        // get the map key of the peripheral that works for this UART's pins
        let peripheral_key = uart_peripherals
            .iter()
            .find_map(|(key, peripheral)| {
                // TODO: actually do filter/select by possible pins
                Some(key)
            })
            .map(|key| key.to_string())
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
                uart.name.as_ref().map_or_else(|| "unnamed", |s| &s)
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
                for (k, v) in uart_cfg.iter() {
                    s.push_str(&format!("        .{} = {},\n", k, v));
                }
                s.push_str("    },\n");
            }
            s.push_str("};\n\n");
        }

        for (n, (_, isr)) in uarts_configured.into_iter().enumerate() {
            if let Some(isr) = isr {
                s.push_str(&format!("#define UART_{n}_ISR          ({isr})\n"));
            }
        }

        s.push_str("#define UART_NUMOF          ARRAY_SIZE(uart_config)\n\n");

        periph_conf_h.content_snips.push(s);

        features.insert("periph_uart".into());
    }

    // finishing
    if !features.is_empty() {
        for feature in features {
            makefile_features.push_str(&format!("FEATURES_PROVIDED += {}\n", feature));
        }
    }

    makefile.push_str("\ninclude $(RIOTBASE)/Makefile.base\n");

    riot_board
        .files
        .insert("include/periph_conf.h".into(), periph_conf_h.to_string());
    riot_board
        .files
        .insert("include/board.h".into(), board_h.to_string());
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
        .ok_or_else(|| anyhow!("error parsing GPIO name: {}", gpio_name))?;

    Ok(format!("GPIO_PIN({}, {})", port, pin))
}
