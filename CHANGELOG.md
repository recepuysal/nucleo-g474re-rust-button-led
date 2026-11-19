# Nucleo-G474RE Rust Button LED - Changelog

### 19.11.2026 @ Recep - 0.1.0.1:

### Fixed
-

### Changed
-

### Added
1. Project created for Nucleo-G474RE (Rust, Embassy 0.6 / executor 0.10)
1. bsp: HSI16 -> PLL 170 MHz, boost mode, LD2 (PA5) output, B1 (PC13) input with pull-down, IWDG 500 ms
1. 400 Hz cooperative scheduler with phase-offset threads (400 ... 1 Hz), watchdog fed every tick
1. MCU load / max tick time / budget overrun measurement, reported once per second
1. LED driver with configurable active level
1. Button driver with 3-sample (30 ms) debounce and press counter
1. Indicator application: LED follows the debounced button
1. Compile-time check that the button is sampled at its expected rate
1. defmt RTT logging, probe-rs runner

- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -
