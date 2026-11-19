//! ******************************************************************************
//! @file    : mcu_timer.rs
//! @brief   : Thread execution time and MCU load measurement
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! The scheduler calls start_tick() / end_tick() around the work of every tick.
//! Busy time is accumulated; once per second (scheduler()) it becomes the MCU
//! load in percent and the longest tick of that second is latched.
//!
//! Load = busy time / wall time. With 400 Hz ticks the budget per tick is
//! TICK_BUDGET_US; a tick longer than that means the next tick is late.
//!
//! ******************************************************************************

use embassy_time::Instant;



/// @brief Const Definitions
pub const TICK_BUDGET_US : u64 = 2_500;        /* 1 / 400 Hz                  */
const     WINDOW_US      : u64 = 1_000_000;    /* load is evaluated per 1 s   */
const     PERCENT_SCALE  : u64 = 10_000;       /* load in 0.01 % units        */



/// @brief  Latched statistics of the last 1 s window
#[derive(Clone, Copy, defmt::Format)]
pub struct LoadReport
{
    pub load_centi_percent : u32,   /* 1234 = 12.34 %                */
    pub max_tick_us        : u32,   /* longest tick in the window    */
    pub overruns           : u32,   /* ticks longer than the budget  */
}



/// @brief  MCU timer
pub struct McuTimer
{
    window_start  : Instant,
    tick_start    : Instant,
    busy_us       : u64,
    max_tick_us   : u64,
    overruns      : u32,
    report        : LoadReport,
}



impl McuTimer
{
    /// @brief   Constructor
    /// @param   void
    /// @return  McuTimer
    pub fn new() -> Self
    {
        let now = Instant::now();

        Self
        {
            window_start : now,
            tick_start   : now,
            busy_us      : 0,
            max_tick_us  : 0,
            overruns     : 0,
            report       : LoadReport { load_centi_percent : 0, max_tick_us : 0, overruns : 0 },
        }
    }



    /// @brief   Mark the start of a tick's work.
    /// @param   void
    /// @return  void
    pub fn start_tick(&mut self)
    {
        self.tick_start = Instant::now();
    }



    /// @brief   Mark the end of a tick's work, accumulate busy time.
    /// @param   void
    /// @return  void
    pub fn end_tick(&mut self)
    {
        let elapsed_us = self.tick_start.elapsed().as_micros();

        self.busy_us += elapsed_us;

        if elapsed_us > self.max_tick_us
        {
            self.max_tick_us = elapsed_us;
        }
        else
        {
            /* not a new maximum */
        }

        if elapsed_us > TICK_BUDGET_US
        {
            self.overruns = self.overruns.saturating_add(1);
        }
        else
        {
            /* within budget */
        }
    }



    /// @brief   Close the 1 s window: compute load, latch report, reset counters.
    ///          Call once per second (scheduler thread1Hz).
    /// @param   void
    /// @return  void
    pub fn scheduler(&mut self)
    {
        let window_us = self.window_start.elapsed().as_micros();

        /* window_us is ~1 s here; the guard only protects the division */
        let divisor = if window_us != 0 { window_us } else { WINDOW_US };

        self.report = LoadReport
        {
            load_centi_percent : ((self.busy_us * PERCENT_SCALE) / divisor) as u32,
            max_tick_us        : self.max_tick_us as u32,
            overruns           : self.overruns,
        };

        self.window_start = Instant::now();
        self.busy_us      = 0;
        self.max_tick_us  = 0;
        self.overruns     = 0;
    }



    /// @brief   Statistics of the last completed window.
    /// @param   void
    /// @return  LoadReport
    pub fn report(&self) -> LoadReport
    {
        self.report
    }
}

/********************************* END OF FILE *********************************/
