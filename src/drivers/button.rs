//! ******************************************************************************
//! @file    : button.rs
//! @brief   : Debounced push-button driver (polled, no interrupt)
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! Call scheduler() periodically at SAMPLE_RATE_HZ (scheduler thread100Hz).
//!
//! One reading never makes a decision: the debounced state changes only after
//! DEBOUNCE_SAMPLES consecutive raw samples disagree with it
//! (3 samples @ 100 Hz = 30 ms). A single matching sample resets the counter.
//!
//! Live raw level and debounced state are kept apart: decisions use
//! is_pressed() only, read_raw() is for diagnostics.
//!
//! ******************************************************************************

use embassy_stm32::gpio::{Input, Level};



/// @brief Const Definitions
pub const SAMPLE_RATE_HZ   : u32 = 100;   /* expected scheduler() call rate     */
const     DEBOUNCE_SAMPLES : u8  = 3;     /* consecutive samples -> 30 ms       */



/// @brief  Button driver
pub struct Button<'d>
{
    input         : Input<'d>,
    active_level  : Level,
    is_pressed    : bool,       /* debounced state - the only decision value */
    sample_count  : u8,         /* consecutive samples disagreeing with it   */
    press_events  : u32,        /* released -> pressed transitions           */
}



impl<'d> Button<'d>
{
    /// @brief   Constructor. Starts in the released state.
    /// @param   input        : owned GPIO input (pull configured by bsp)
    /// @param   active_level : pin level while the button is pressed
    /// @return  Button
    pub fn new(input: Input<'d>, active_level: Level) -> Self
    {
        Self
        {
            input,
            active_level,
            is_pressed   : false,
            sample_count : 0,
            press_events : 0,
        }
    }



    /// @brief   Periodic sample + debounce. Call at SAMPLE_RATE_HZ.
    /// @param   void
    /// @return  void
    pub fn scheduler(&mut self)
    {
        let raw_pressed = self.read_raw();

        if raw_pressed != self.is_pressed
        {
            self.sample_count += 1;

            if self.sample_count >= DEBOUNCE_SAMPLES
            {
                self.is_pressed   = raw_pressed;
                self.sample_count = 0;

                if raw_pressed
                {
                    self.press_events = self.press_events.wrapping_add(1);
                }
                else
                {
                    /* release edge: nothing to count */
                }
            }
            else
            {
                /* not stable long enough yet */
            }
        }
        else
        {
            self.sample_count = 0;   /* agrees with debounced state -> reset */
        }
    }



    /// @brief   Debounced state - use this for decisions.
    /// @param   void
    /// @return  bool : true = pressed
    pub fn is_pressed(&self) -> bool
    {
        self.is_pressed
    }



    /// @brief   Number of debounced presses since power-up (wraps).
    /// @param   void
    /// @return  u32 : press count
    pub fn press_events(&self) -> u32
    {
        self.press_events
    }



    /// @brief   Live, un-debounced pin state (diagnostics only).
    /// @param   void
    /// @return  bool : true = pin at active level
    pub fn read_raw(&self) -> bool
    {
        self.input.get_level() == self.active_level
    }
}

/********************************* END OF FILE *********************************/
