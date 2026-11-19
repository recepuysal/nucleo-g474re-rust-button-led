//! ******************************************************************************
//! @file    : indicator.rs
//! @brief   : Application rule: LED follows the debounced button
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! The single decision point of the application. Call scheduler() after the
//! button has been sampled in the same tick (scheduler thread100Hz).
//!
//! Rule: button pressed -> LED on, released -> LED off.
//! The LED is written only on a change, so logging stays event-based.
//!
//! ******************************************************************************

use crate::drivers::button::Button;
use crate::drivers::led::Led;



/// @brief  Indicator application
pub struct Indicator
{
    last_state : bool,
}



impl Indicator
{
    /// @brief   Constructor
    /// @param   void
    /// @return  Indicator
    pub const fn new() -> Self
    {
        Self
        {
            last_state : false,
        }
    }



    /// @brief   Periodic rule evaluation.
    /// @param   button : read-only access - the indicator never drives the button
    /// @param   led    : exclusive access for this call only
    /// @return  void
    pub fn scheduler(&mut self, button: &Button<'_>, led: &mut Led<'_>)
    {
        let state = button.is_pressed();

        if state != self.last_state
        {
            led.set(state);
            self.last_state = state;
            defmt::info!("button {} -> LED {}", if state { "pressed" } else { "released" }, if state { "on" } else { "off" });
        }
        else
        {
            /* no change: leave the LED alone */
        }
    }
}

/********************************* END OF FILE *********************************/
