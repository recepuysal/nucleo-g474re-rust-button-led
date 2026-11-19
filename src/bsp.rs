//! ******************************************************************************
//! @file    : bsp.rs
//! @brief   : Board Support - Nucleo-G474RE (MB1367) wiring and clock tree
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! This is the ONLY file that knows pin numbers. A different board = a new bsp.
//!
//!   Signal      Pin    Nucleo part   Polarity
//!   ----------  -----  ------------  --------------------------------------------
//!   LD2 (green) PA5    LD2           active-high (HIGH = LED on)
//!   B1 (blue)   PC13   B1 USER       active-high on MB1367 (pressed = HIGH),
//!                                    internal pull-down keeps it LOW when idle.
//!                                    If your board reads inverted, change only
//!                                    BUTTON_ACTIVE_LEVEL below.
//!
//!   Clock: HSI16 / 4 * 85 / 2 = 170 MHz SYSCLK (PLL R), boost mode on.
//!   Watchdog: IWDG, WATCHDOG_TIMEOUT_US, fed at the end of every scheduler tick.
//!
//! ******************************************************************************

use embassy_stm32::Config;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::IWDG;
use embassy_stm32::rcc::{Pll, PllMul, PllPreDiv, PllRDiv, PllSource, Sysclk};
use embassy_stm32::wdg::IndependentWatchdog;

use crate::drivers::button::Button;
use crate::drivers::led::Led;



/// @brief Const Definitions
const LED_ACTIVE_LEVEL:    Level = Level::High;     /* LD2: HIGH = on                 */
const BUTTON_ACTIVE_LEVEL: Level = Level::High;     /* B1 on MB1367: pressed = HIGH   */
const WATCHDOG_TIMEOUT_US: u32   = 500_000;         /* 200 missed ticks @ 400 Hz      */



/// @brief  Everything the application owns. Each field has exactly one owner.
pub struct Board
{
    pub led      : Led<'static>,
    pub button   : Button<'static>,
    pub watchdog : IndependentWatchdog<'static, IWDG>,
}



/// @brief   Initialize clocks and peripherals, build the board objects.
///          All outputs start in their SAFE state (LED off).
/// @param   void
/// @return  Board : owned peripherals for the scheduler
pub fn initialize() -> Board
{
    let mut config = Config::default();

    config.rcc.pll = Some(Pll
    {
        source : PllSource::HSI,        /* 16 MHz              */
        prediv : PllPreDiv::DIV4,       /* 4 MHz VCO input     */
        mul    : PllMul::MUL85,         /* 340 MHz VCO         */
        divp   : None,
        divq   : None,
        divr   : Some(PllRDiv::DIV2),   /* 170 MHz SYSCLK      */
    });
    config.rcc.sys   = Sysclk::PLL1_R;
    config.rcc.boost = true;            /* required above 150 MHz */

    let p = embassy_stm32::init(config);

    let led    = Led::new(Output::new(p.PA5, Level::Low, Speed::Low), LED_ACTIVE_LEVEL);
    let button = Button::new(Input::new(p.PC13, Pull::Down), BUTTON_ACTIVE_LEVEL);

    let mut watchdog = IndependentWatchdog::new(p.IWDG, WATCHDOG_TIMEOUT_US);
    watchdog.unleash();

    Board
    {
        led,
        button,
        watchdog,
    }
}

/********************************* END OF FILE *********************************/
