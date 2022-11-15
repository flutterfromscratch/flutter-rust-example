#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.49.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

fn wire_battery_event_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "battery_event_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| battery_event_stream(task_callback.stream_sink()),
    )
}
fn wire_getBatteryStatus_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "getBatteryStatus",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| getBatteryStatus(),
    )
}
fn wire_init_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(init()),
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
// Section: impl IntoDart

impl support::IntoDart for BatteryUpdate {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.charge_rates_in_milliwatts.into_dart(),
            self.design_capacity_in_milliwatt_hours.into_dart(),
            self.full_charge_capacity_in_milliwatt_hours.into_dart(),
            self.remaining_capacity_in_milliwatt_hours.into_dart(),
            self.status.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BatteryUpdate {}

impl support::IntoDart for ChargingState {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Charging => 0,
            Self::Discharging => 1,
            Self::Idle => 2,
            Self::NotPresent => 3,
            Self::Unknown => 4,
        }
        .into_dart()
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
