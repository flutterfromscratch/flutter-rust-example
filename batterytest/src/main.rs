use std::io::stdin;
use std::ptr::{null, null_mut};
use windows::core::IInspectable;
use windows::Devices::Power::Battery;
use windows::Foundation::TypedEventHandler;
use windows::Win32::System::Com::CoInitialize;

use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

fn main() {
    println!("Hello, world!");

    let mut powerStatus: SYSTEM_POWER_STATUS = SYSTEM_POWER_STATUS::default();

    unsafe {
        GetSystemPowerStatus(&mut powerStatus);
        println!("{:?}", powerStatus);
    }


    let battery = windows::Devices::Power::Battery::AggregateBattery().expect("works");
    let outcome = windows::Devices::Power::Battery::GetReport(&battery).expect("okay");
    let handler = TypedEventHandler::<Battery, IInspectable>::new(|x,a| {
        println!("{:?}", x);
        Ok(())
    });
    battery.ReportUpdated(&handler).expect("blahhh");
    // let handler = windows::Foundation::TypedEventHandler::new();
    // battery.ReportUpdated();
    println!("{:?}", outcome);
    stdin().read_line(&mut String::new()).unwrap();
}



