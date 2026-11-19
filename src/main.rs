//! ******************************************************************************
//! @file    : main.rs
//! @brief   : Entry point - bring the board up, then hand over to the scheduler
//! @author  : Recep Uysal
//! @date    : 19.11.2026
//! @version : 0.1.0
//! ******************************************************************************
//!
//! *****************************   USAGE INFORMATION   *************************
//!
//! Layers (dependencies only point downwards):
//!
//!   main.rs       entry, nothing else
//!   scheduler.rs  400 Hz cooperative scheduler, thread400Hz ... thread1Hz
//!   app/          application logic (indicator: button -> LED)
//!   drivers/      device drivers (led, button) - board independent
//!   bsp.rs        board support: the ONLY place that knows pin numbers / clocks
//!   common/       version, MCU load measurement
//!
//! ******************************************************************************

#![no_std]
#![no_main]

mod app;
mod bsp;
mod common;
mod drivers;
mod scheduler;

use defmt_rtt as _;   // defmt transport: RTT over the on-board ST-LINK
use panic_probe as _; // panic -> defmt message + probe exit

use embassy_executor::Spawner;

use crate::common::version::VERSION;
use crate::scheduler::RtScheduler;



/// @brief   Entry point. Initializes the board, then runs the scheduler forever.
/// @param   _spawner : unused - the whole application runs in one scheduler task
/// @return  never
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> !
{
    defmt::info!("nucleo-g474re-button-led v{}.{}.{}.{}",
                 VERSION.major, VERSION.minor, VERSION.patch, VERSION.suffix);

    let board = bsp::initialize();

    let mut scheduler = RtScheduler::new(board);
    scheduler.run().await
}

/********************************* END OF FILE *********************************/
