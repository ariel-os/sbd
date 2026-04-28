# SBD File Format

SBD (Structured Board Description) files describe board hardware in YAML for
use with `sbd-gen`, which generates OS-specific board support code.

> **Note**: This document was generated using Claude Opus 4.6 and reviewed for
> obvious errors. It is not authoritative documentation — we don't have that
> yet. Until we do, this serves as a rough, LLM-generated overview that is at
> least not blatantly wrong. If you'd like to help document this properly,
> please contribute to the rustdoc comments in
> [`sbd-gen-schema`](https://docs.rs/sbd-gen-schema) rather than editing this
> file, as it will be LLM-regenerated as the source code evolves.

## Multi-File Merging

All `.yaml` files in a directory are collected recursively, sorted
alphabetically, and deep-merged into a single document before parsing. This
allows splitting board descriptions across files — e.g., one file for chip lists,
separate files for the targets. All conflicts are resolved silently in favor of the later
(alphabetically) file. The only case where data is combined rather than replaced
is when both sides are maps — then recursion goes deeper. Everything else
(scalars, arrays, type mismatches) is a quiet last-writer-wins.

Unknown fields are rejected as a hard error.

## Version

The version field is exceptional in that it is checked before merging.
It expresses a semver requirement on the `sbd-gen` tool that processes it, following the [usual Rust rules](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#version-requirement-syntax).

## Top-Level Keys

| Key           | Required | Type                                      | Description                                      |
|---------------|----------|-------------------------------------------|--------------------------------------------------|
| `version`     | no       | string                                    | Schema version (semver).                         |
| `description` | no       | string                                    | Human-readable description (currently unused).   |
| `include`     | no       | list of strings                           | Currently unused.                                |
| `targets`     | no       | map of string → [Target](#target-fields)  | Map of target name to target definition.         |
| `ariel`       | no       | [Ariel](#ariel-configuration)             | Ariel OS global configuration.                   |
| `riot`        | no       | [RIOT](#riot-configuration)               | RIOT OS global configuration.                    |

Typical use has boards separated from the OS-specific globals. A file like this
might be provided with Ariel OS:

```yaml
version: 0.2.0
ariel:
  chips:
    - nrf52840
    - nrf5340-app
```

while an application could bring its own boards:

```yaml
version: 0.2.0
targets:
  my-board-rev-a:
    chip: nrf52840
    leds: ...
  my-board-rev-b:
    chip: nrf5340-app
    leds: ...
```

## Targets

The `targets` key is a map. Each key becomes the target name.

| Field       | Required | Description                                              |
|-------------|----------|----------------------------------------------------------|
| `chip`      | **yes**  | Chip/SoC identifier. Must appear in the relevant OS chip list to be processed. |
| `description` | no     | Human-readable description (currently unused).            |
| `include`   | no       | Currently unused.                                        |
| `flags`     | no       | Set of capability flags (e.g., `has_usb_device_port`).   |
| `quirks`    | no       | List of hardware quirk operations. See [Quirks](#quirks).|
| `debugger`  | no       | Debugger configuration. See [Debugger](#debugger).       |
| `leds`      | no       | Map of LED name to definition. See [LEDs](#leds).        |
| `buttons`   | no       | Map of button name to definition. See [Buttons](#buttons).|
| `uarts`     | no       | Map of UART name to definition. See [UARTs](#uarts).     |
| `ariel`     | no       | Ariel OS per-target configuration. See [Ariel OS Per-Target](#ariel-os-per-target). |
| `riot`      | no       | RIOT OS per-target configuration (currently unused).     |

```yaml
targets:
  nrf52840dk:
    chip: nrf52840
    flags:
      - has_usb_device_port
    leds:
      led0:
        color: green
        pin: P0_13
        active: low
      led1:
        color: green
        pin: P0_14
        active: low
    buttons:
      button0:
        pin: P0_11
        active: low
      button1:
        pin: P0_12
        active: low
```

A target can be minimal — only `chip` is required:

```yaml
targets:
  nrf5340dk-net:
    chip: nrf5340-net
```

Multiple targets can be defined in a single file:

```yaml
targets:
  nordic-thingy-91-x-nrf9151:
    chip: nrf9151
  nordic-thingy-91-x-nrf5340-app:
    chip: nrf5340-app
  nordic-thingy-91-x-nrf5340-net:
    chip: nrf5340-net
```

### Flags

Flags are a set of strings declaring board capabilities. They are passed
verbatim by the Ariel OS generator into the `provides:` field of the resulting
laze builder. The Ariel OS build system is authoritative on their meaning.

Common values:

- `has_ble_cyw43`
- `has_ethernet_stm32`
- `has_nrf_ble`
- `has_usb_device_port`
- `has_wifi_cyw43`

## LEDs

Map of LED name to definition.

| Field    | Required | Description                                      |
|----------|----------|--------------------------------------------------|
| `pin`    | **yes**  | GPIO pin name (HAL-specific, see [Pin Naming](#pin-naming)). |
| `color`  | no       | LED color (e.g., `red`, `green`, `blue`, `white`, `yellow`, `orange`). |
| `active` | no       | Active level: `high` or `low`.                   |
| `aliases`| no       | List of alternative names (e.g., schematic labels). Currently informative only. |

```yaml
leds:
  led0:
    color: green
    pin: P0_13
    active: low
    aliases:
      - main_led
  led1:
    color: red
    pin: PA5
```

## Buttons

Map of button name to definition.

| Field    | Required | Description                                      |
|----------|----------|--------------------------------------------------|
| `pin`    | **yes**  | GPIO pin name (HAL-specific, see [Pin Naming](#pin-naming)). |
| `active` | no       | Active level: `high` or `low`.                   |
| `aliases`| no       | List of alternative names. Currently informative only. |

```yaml
buttons:
  button0:
    pin: P0_11
    active: low
  button1:
    pin: P0_12
    active: low
    aliases:
      - user_btn
```

## UARTs

Map of UART name to definition.

| Field     | Required | Description            |
|-----------|----------|------------------------|
| `rx_pin`  | **yes**  | Receive pin.           |
| `tx_pin`  | **yes**  | Transmit pin.          |
| `cts_pin` | no       | Clear-to-send pin.     |
| `rts_pin` | no       | Ready-to-send pin.     |

```yaml
uarts:
  uart0:
    rx_pin: P0_08
    tx_pin: P0_06
    cts_pin: P0_07
    rts_pin: P0_05
```

## Debugger

| Field  | Required | Description                                          |
|--------|----------|------------------------------------------------------|
| `type` | **yes**  | Debugger type (e.g., `jlink`, `openocd`).            |
| `uart` | no       | UART connection used by the debugger (same fields as a UART entry, without a name). |

```yaml
debugger:
  type: jlink
  uart:
    rx_pin: P0_08
    tx_pin: P0_06
```

## Quirks

A list of hardware quirk operations applied at initialization time. Each entry
has a `type` field that selects the operation, and more fields per type.

### `set_pin`

Sets a GPIO pin to a fixed level at init time.

| Field         | Required | Description                             |
|---------------|----------|-----------------------------------------|
| `type`        | **yes**  | Must be `set_pin`.                      |
| `pin`         | **yes**  | GPIO pin name.                          |
| `level`       | **yes**  | Pin level: `high` or `low`.             |
| `description` | no       | Explanation of what setting that pin effects.|

```yaml
quirks:
  - type: set_pin
    description: turn off buzzer
    pin: GPIO15
    level: low
```

## Pin Naming

Pin names must match the naming convention of the target HAL used by the
generator. Examples from different chip families:

| Chip Family | Example Pins             |
|-------------|--------------------------|
| nRF         | `P0_13`, `P1_12`         |
| STM32       | `PA5`, `PH10`, `PC13`    |
| ESP32       | `GPIO15`, `GPIO2`        |
| RP2040      | `PIN_25`, `PIN_2`        |

## Ariel OS Configuration

### Global

The top-level `ariel` key holds global Ariel OS configuration.

| Field  | Required | Description                                          |
|--------|----------|------------------------------------------------------|
| `chips`| no       | List of chip names supported by Ariel OS. Targets whose chip is not in this list are skipped during Ariel code generation. |

```yaml
ariel:
  chips:
    - nrf52840
    - nrf5340-app
    - esp32-c6-mini-1
    - stm32u585ai
```

### Per-Target

Each target may have an `ariel` section with Ariel OS-specific settings.

| Field        | Required | Description                                      |
|--------------|----------|--------------------------------------------------|
| `flags`      | no       | Set of Ariel-specific capability flags. Used verbatim in the laze builder `provides:`. |
| `global_env` | no       | Map of laze environment variable name to value (single string or list of strings). Copied verbatim into the laze builder `global_env:`. |
| `swi`        | no       | Software interrupt name.                         |

```yaml
targets:
  arduino-uno-q:
    chip: stm32u585ai
    ariel:
      flags:
        - has_swi
      global_env:
        CARGO_ENV:
          - CONFIG_VBUS_DETECTION=false
          - CONFIG_SWI=USART2
      swi: USART2
```

## RIOT OS Configuration

### Global

The top-level `riot` key holds global RIOT OS configuration.

| Field  | Required | Description                                              |
|--------|----------|----------------------------------------------------------|
| `chips`| **yes**  | Map of chip name to chip configuration. Targets whose chip is not in this map are skipped during RIOT code generation. |

Each chip entry:

| Field         | Required | Description                                      |
|---------------|----------|--------------------------------------------------|
| `cpu`         | **yes**  | RIOT CPU identifier.                             |
| `cpu_model`   | **yes**  | RIOT CPU model identifier.                       |
| `quirks`      | no       | Map of output filename to content. Used to inject raw C snippets into generated header files. |
| `peripherals` | no       | Chip peripheral definitions for code generation. |

Quirk entries contain a `body` field with a list of C code strings:

```yaml
riot:
  chips:
    nrf52840:
      cpu: nrf52
      cpu_model: nrf52840xxaa
      quirks:
        periph_conf.h:
          body:
            - "#define SOME_DEFINE 1\n"
        board.h:
          body:
            - "/* board init */\n"
```

#### RIOT Chip Peripherals

Currently supports UART peripherals.

Each UART entry is a map of peripheral name to configuration:

| Field    | Required | Description                                |
|----------|----------|--------------------------------------------|
| `config` | no       | Map of C struct field names to values.     |
| `isr`    | no       | ISR handler name for this UART.            |

```yaml
riot:
  chips:
    nrf52840:
      cpu: nrf52
      cpu_model: nrf52840xxaa
      peripherals:
        uarts:
          UARTE0:
            config:
              dev: NRF_UARTE0
              irqn: UARTE0_UART0_IRQn
            isr: isr_uart0
```

### Per-Target

Each target may have a `riot` section. This is currently reserved for future
use and accepts no fields.

## Full Example

A complete example combining multiple features:

```yaml
version: 0.2.0
description: Example board definitions

targets:
  nrf52840dk:
    chip: nrf52840
    flags:
      - has_usb_device_port
    debugger:
      type: jlink
      uart:
        rx_pin: P0_08
        tx_pin: P0_06
    leds:
      led0:
        color: green
        pin: P0_13
        active: low
      led1:
        color: green
        pin: P0_14
        active: low
      led2:
        color: green
        pin: P0_15
        active: low
      led3:
        color: green
        pin: P0_16
    buttons:
      button0:
        pin: P0_11
        active: low
      button1:
        pin: P0_12
        active: low
      button2:
        pin: P0_24
        active: low
      button3:
        pin: P0_25
        active: low
    uarts:
      uart0:
        rx_pin: P0_08
        tx_pin: P0_06
        cts_pin: P0_07
        rts_pin: P0_05

  ulanzi-tc001:
    chip: esp32-s3-mini-1-n4r2
    quirks:
      - type: set_pin
        description: turn off buzzer
        pin: GPIO15
        level: low

  arduino-uno-q:
    chip: stm32u585ai
    ariel:
      flags:
        - has_swi
      global_env:
        CARGO_ENV:
          - CONFIG_VBUS_DETECTION=false
          - CONFIG_SWI=USART2
      swi: USART2
    leds:
      led0:
        color: red
        pin: PH10
      led1:
        color: green
        pin: PH11
      led2:
        color: blue
        pin: PH12

ariel:
  chips:
    - nrf52840
    - esp32-s3-mini-1-n4r2
    - stm32u585ai
```
