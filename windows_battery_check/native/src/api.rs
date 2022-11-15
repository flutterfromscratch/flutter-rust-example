use std::sync::RwLock;
use flutter_rust_bridge::StreamSink;
use anyhow::{anyhow, Result};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

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