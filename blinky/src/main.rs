#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, log::*, exit};
use ariel_os::thread::{ThreadId, sync::Event};
use ariel_os::time::Timer;
use ariel_os::gpio::{DriveStrength, Input, Output, Level, Pull, Speed};


mod pins;

static EVENT: Event = Event::new();

fn waiter() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    let my_prio = ariel_os::thread::get_priority(my_id).unwrap();
    let core = ariel_os::thread::core_id();
    info!("[{:?}@{:?}-{:?}] Waiting for event...", my_id, my_prio, core);
    EVENT.wait();
    info!("[{:?}@{:?}{:?}] Done.", my_id, my_prio, core);

    if my_id == ThreadId::new(0) {
        info!(
            "[{:?}@{:?}-{:?}] All five threads should have reported \"Done.\". exiting.",
            my_id, my_prio, core
        );
        //ariel_os::debug::exit(ExitCode::SUCCESS);
    }
    info!("Final of the thread {:?}{:?}{:?}", my_id,my_prio, core);
}

#[ariel_os::thread(autostart, priority = 1)]
fn thread0() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    let my_prio = ariel_os::thread::get_priority(my_id).unwrap();
    let core = ariel_os::thread::core_id();
    info!("[{:?}@{:?}-{:?}] Setting event...", my_id, my_prio, core);
    EVENT.set();
    info!("[{:?}@{:?}-{:?}] Event set.", my_id, my_prio, core);

    waiter();

}

#[ariel_os::thread(autostart, priority = 2)]
fn thread1() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 3)]
fn thread2() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 4)]
fn thread3() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 2)]
fn thread4() {

    waiter();
}

#[ariel_os::task(autostart)]
async fn main_task() {
    let core = ariel_os::thread::core_id();
    loop{
        info!("Main task started.");
        Timer::after_millis(1000).await;
        info!("main tick.{:?}", core);
    }
}

#[ariel_os::task(autostart, peripherals)]
async fn blinky_task(peripherals: pins::OutputPeripherals) {
    // Initialize pin_0 as output low
    let mut _led_0 = Output::new(peripherals.pin_2, Level::Low);
    let core = ariel_os::thread::core_id();




    info!("Blinky task started.");
    loop{
        _led_0.set_level(Level::High);
        Timer::after_millis(500).await;       
        _led_0.set_level(Level::Low);
        Timer::after_millis(500).await;
        info!("Blinky tick.{:?}", core);
    }

}

#[ariel_os::task(autostart, peripherals)]
async fn btn_task(peripherals: pins::InputPeripherals) {   
    let _btn_a = Input::new(peripherals.switch_a, Pull::Up);
    
    let btn_b_builder = Input::builder(peripherals.switch_b, Pull::Up);
    let btn_b_builder = btn_b_builder.schmitt_trigger(true);
    let _btn_b = btn_b_builder.build();

}