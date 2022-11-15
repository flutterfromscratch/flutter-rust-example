use std::sync::{RwLock, TryLockResult};
use flutter_rust_bridge::StreamSink;
use anyhow::{anyhow, Result};
use windows::core::IInspectable;
use windows::Devices::Power::Battery;
use windows::Foundation::TypedEventHandler;
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use crate::api::ChargingState::{Charging, Discharging, Idle, NotPresent, Unknown};

static BATTERY_REPORT_STREAM: RwLock<Option<StreamSink<BatteryUpdate>>> = RwLock::new(None);

pub fn battery_event_stream(s: StreamSink<BatteryUpdate>) -> Result<()> {
    let mut stream = BATTERY_REPORT_STREAM.write().unwrap();
    *stream = Some(s);
    Ok(())
}

pub fn getBatteryStatus() -> Result<bool>{
    // https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-system_power_status
    let mut powerStatus: SYSTEM_POWER_STATUS = SYSTEM_POWER_STATUS::default();

    unsafe {
        GetSystemPowerStatus(&mut powerStatus);
        Ok(powerStatus.BatteryFlag != 128)
    }
}

pub fn init() {

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
        match BATTERY_REPORT_STREAM.try_read(){
            Ok(s) => {
                s.as_ref().unwrap().add(batteryOutcome);
            }
            Err(_) => {
                println!("Error when writing battery status.");
            }
        }
        // BATTERY_REPORT_STREAM.read().unwrap().unwrap().add(batteryOutcome);
        Ok(())
    })).expect("sdds");

}

#[derive(Debug)]
pub struct BatteryUpdate {
    pub charge_rates_in_milliwatts: Option<i32>,
    pub design_capacity_in_milliwatt_hours: Option<i32>,
    pub full_charge_capacity_in_milliwatt_hours: Option<i32>,
    pub remaining_capacity_in_milliwatt_hours: Option<i32>,
    pub status: ChargingState,
}

#[derive(Debug)]
pub enum ChargingState{
    Charging = 3,
    Discharging = 1,
    Idle = 2,
    NotPresent = 0,
    Unknown =255,
}