//! ******************************************************************************
//! @file    : led.rs
//! @brief   : Single LED driver with configurable active level
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! The driver owns its output pin. Because Rust allows only one owner, nothing
//! else in the firmware can drive this pin - the "only the actuator's own class
//! writes the actuator" rule is enforced by the compiler.
//!
//! Callers talk in "on/off", never in pin levels: polarity lives here.
//!
//! ******************************************************************************

use embassy_stm32::gpio::{Level, Output};



/// @brief  LED driver
pub struct Led<'d>
{
    output       : Output<'d>,
    active_level : Level,
    is_on        : bool,
}



impl<'d> Led<'d>
{
    /// @brief   Constructor. Forces the LED OFF regardless of the pin's initial level.
    /// @param   output       : owned GPIO output
    /// @param   active_level : pin level that turns the LED on
    /// @return  Led
    pub fn new(output: Output<'d>, active_level: Level) -> Self
    {
        let mut led = Self
        {
            output,
            active_level,
            is_on : false,
        };
        led.set(false);
        led
    }



    /// @brief   Switch the LED on or off.
    /// @param   state : true = on, false = off
    /// @return  void
    pub fn set(&mut self, state: bool)
    {
        let level = if state { self.active_level } else { invert(self.active_level) };

        self.output.set_level(level);
        self.is_on = state;
    }



    /// @brief   Current commanded state.
    /// @param   void
    /// @return  bool : true = on
    pub fn is_on(&self) -> bool
    {
        self.is_on
    }
}



/// @brief   Opposite logic level.
/// @param   level : Level::High or Level::Low
/// @return  Level : the other one
fn invert(level: Level) -> Level
{
    match level
    {
        Level::High => Level::Low,
        Level::Low  => Level::High,
    }
}

/********************************* END OF FILE *********************************/
