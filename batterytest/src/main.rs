use std::io::stdin;
use std::ptr::{null, null_mut};
use windows::core::{IInspectable, Interface};
use windows::Devices::Power::Battery;
use windows::Foundation::TypedEventHandler;
use windows::System::Power::BatteryStatus;
use windows::Win32::System::Com::CoInitialize;

use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use crate::ChargingState::{Charging, Discharging, Idle, NotPresent, Unknown};

#[derive(Debug)]
struct BatteryUpdate {
    charge_rates_in_milliwatts: Option<i32>,
    design_capacity_in_milliwatt_hours: Option<i32>,
    full_charge_capacity_in_milliwatt_hours: Option<i32>,
    remaining_capacity_in_milliwatt_hours: Option<i32>,
    status: ChargingState,
}

#[derive(Debug)]
enum ChargingState{
    Charging = 3,
    Discharging = 1,
    Idle = 2,
    NotPresent = 0,
    Unknown =255,
}

fn main() {
    println!("Hello, world!");

    let mut powerStatus: SYSTEM_POWER_STATUS = SYSTEM_POWER_STATUS::default();

    unsafe {
        GetSystemPowerStatus(&mut powerStatus);
        println!("{:?}", powerStatus);

    }

    Battery::AggregateBattery().unwrap().ReportUpdated(&TypedEventHandler::<Battery, IInspectable>::new(|battery, inspectable| {
        let aggBattery = Battery::AggregateBattery();
        let report = aggBattery.unwrap().GetReport().unwrap();

        let batteryOutcome = BatteryUpdate{
            charge_rates_in_milliwatts: match report.ChargeRateInMilliwatts() {
                Ok(charge_rate) => {
                    Some(charge_rate.GetInt32().unwrap())
                }
                Err(_) => {
                    None
                }
            },
            design_capacity_in_milliwatt_hours: match report.DesignCapacityInMilliwattHours() {
                Ok(design_capacity) => {
                    Some(design_capacity.GetInt32().unwrap())
                }
                Err(_) => {
                    None
                }
            },
            full_charge_capacity_in_milliwatt_hours: match report.FullChargeCapacityInMilliwattHours() {
                Ok(full_charge) => {
                    Some(full_charge.GetInt32().unwrap())
                }
                Err(_) => {
                    None
                }
            },
            remaining_capacity_in_milliwatt_hours: match report.RemainingCapacityInMilliwattHours() {
                Ok(remaining_capacity) => {
                    Some(remaining_capacity.GetInt32().unwrap())
                }
                Err(_) => {
                    None
                }
            },
            status: match report.Status().unwrap().0{
                3 => Charging,
                1 => Discharging,
                2 => Idle,
                0 => NotPresent,
                _ => Unknown
            }
        };

        // println!()

        println!("Handler Update{:?}", batteryOutcome);
        Ok(())
    })).expect("sdds");

    stdin().read_line(&mut String::new()).unwrap();
}



