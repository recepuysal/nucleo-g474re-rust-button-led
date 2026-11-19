//! ******************************************************************************
//! @file    : scheduler.rs
//! @brief   : Application Main Real-Time Scheduler (400 Hz cooperative)
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! Same model as the C++ RT_SCHEDULER: one 400 Hz tick, slower threads by
//! divisor, each with its own phase offset so they never pile onto the same
//! tick. The tick comes from an embassy Ticker instead of a TIM6 interrupt
//! flag - there is no shared `volatile` variable to forget.
//!
//! Rules:
//!  - No thread blocks or awaits. Long jobs are step-by-step state machines.
//!  - Every tick is measured (common::mcu_timer); budget is 2.5 ms.
//!  - The watchdog is fed at the END of every tick, so a stuck thread resets
//!    the MCU.
//!
//! ******************************************************************************

use embassy_time::{Duration, Ticker};

use crate::app::indicator::Indicator;
use crate::bsp::Board;
use crate::common::mcu_timer::McuTimer;
use crate::drivers::button;



/// @brief Const Definitions
const TICK_RATE_HZ    : u64 = 400;
const COUNTER_LIMIT   : u32 = 400;      /* counter wraps once per second */

/*                          divisor  phase                                    */
const DIVISOR_200HZ   : u32 =   2;  const COMPARE_200HZ : u32 =  0;
const DIVISOR_100HZ   : u32 =   4;  const COMPARE_100HZ : u32 =  1;
const DIVISOR_50HZ    : u32 =   8;  const COMPARE_50HZ  : u32 =  3;
const DIVISOR_20HZ    : u32 =  20;  const COMPARE_20HZ  : u32 =  7;
const DIVISOR_10HZ    : u32 =  40;  const COMPARE_10HZ  : u32 = 11;
const DIVISOR_5HZ     : u32 =  80;  const COMPARE_5HZ   : u32 = 15;
const DIVISOR_2HZ     : u32 = 200;  const COMPARE_2HZ   : u32 = 23;
const DIVISOR_1HZ     : u32 = 400;  const COMPARE_1HZ   : u32 = 63;

/* Compile-time check: the button debounce time assumes it is sampled in thread100Hz. */
const _: () = assert!((TICK_RATE_HZ as u32 / DIVISOR_100HZ) == button::SAMPLE_RATE_HZ,
                      "button::scheduler() must run at button::SAMPLE_RATE_HZ");



/// @brief  Real-time scheduler. Owns the board and the applications.
pub struct RtScheduler
{
    board     : Board,
    indicator : Indicator,
    timer     : McuTimer,
    counter   : u32,
}



impl RtScheduler
{
    /// @brief   Constructor
    /// @param   board : owned board peripherals
    /// @return  RtScheduler
    pub fn new(board: Board) -> Self
    {
        Self
        {
            board,
            indicator : Indicator::new(),
            timer     : McuTimer::new(),
            counter   : 0,
        }
    }



    /// @brief   400 Hz (2.5 ms) Thread - fast sensors, motion
    fn thread_400hz(&mut self) { }

    /// @brief   200 Hz (5 ms) Thread - communication state machines
    fn thread_200hz(&mut self) { }

    /// @brief   100 Hz (10 ms) Thread - button sampling + indicator rule
    fn thread_100hz(&mut self)
    {
        self.board.button.scheduler();
        self.indicator.scheduler(&self.board.button, &mut self.board.led);
    }

    /// @brief   50 Hz (20 ms) Thread
    fn thread_50hz(&mut self) { }

    /// @brief   20 Hz (50 ms) Thread
    fn thread_20hz(&mut self) { }

    /// @brief   10 Hz (100 ms) Thread
    fn thread_10hz(&mut self) { }

    /// @brief   5 Hz (200 ms) Thread
    fn thread_5hz(&mut self) { }

    /// @brief   2 Hz (500 ms) Thread
    fn thread_2hz(&mut self) { }

    /// @brief   1 Hz (1000 ms) Thread - heartbeat log + load report
    fn thread_1hz(&mut self)
    {
        defmt::info!("$alive led={} presses={} raw={} {}",
                     self.board.led.is_on(),
                     self.board.button.press_events(),
                     self.board.button.read_raw(),
                     self.timer.report());
    }



    /* *************************  DO NOT MODIFY BELOW  ************************* */

    /// @brief   Scheduler main loop. Never returns.
    /// @param   void
    /// @return  never
    pub async fn run(&mut self) -> !
    {
        let mut ticker = Ticker::every(Duration::from_hz(TICK_RATE_HZ));

        loop
        {
            ticker.next().await;

            self.timer.start_tick();
            self.counter += 1;

            self.thread_400hz();
            if (self.counter % DIVISOR_200HZ) == COMPARE_200HZ { self.thread_200hz(); }
            if (self.counter % DIVISOR_100HZ) == COMPARE_100HZ { self.thread_100hz(); }
            if (self.counter % DIVISOR_50HZ)  == COMPARE_50HZ  { self.thread_50hz();  }
            if (self.counter % DIVISOR_20HZ)  == COMPARE_20HZ  { self.thread_20hz();  }
            if (self.counter % DIVISOR_10HZ)  == COMPARE_10HZ  { self.thread_10hz();  }
            if (self.counter % DIVISOR_5HZ)   == COMPARE_5HZ   { self.thread_5hz();   }
            if (self.counter % DIVISOR_2HZ)   == COMPARE_2HZ   { self.thread_2hz();   }
            if (self.counter % DIVISOR_1HZ)   == COMPARE_1HZ   { self.thread_1hz();   }

            if self.counter >= COUNTER_LIMIT
            {
                self.counter = 0;
                self.timer.scheduler();
            }
            else
            {
                /* inside the 1 s window */
            }

            self.board.watchdog.pet();
            self.timer.end_tick();
        }
    }
}

/********************************* END OF FILE *********************************/
