use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_battery_event_stream(port_: MessagePort) {
    wire_battery_event_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_getBatteryStatus(port_: MessagePort) {
    wire_getBatteryStatus_impl(port_)
}

#[wasm_bindgen]
pub fn wire_init(port_: MessagePort) {
    wire_init_impl(port_)
}

// Section: allocate functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue
