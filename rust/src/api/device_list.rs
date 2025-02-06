use flutter_rust_bridge::frb;
pub use frostsnap_core::DeviceId;

#[frb(mirror(DeviceId))]
pub struct _DeviceId(pub [u8; 33]);

// #[derive(Clone, Debug)]
// pub struct DeviceListChange {
//     pub kind: DeviceListChangeKind,
//     pub index: usize,
//     pub device: ConnectedDevice,
// }

// #[derive(Clone, Debug)]
// pub struct DeviceListUpdate {
//     pub changes: Vec<DeviceListChange>,
//     pub state: DeviceListState,
// }

// #[derive(Clone, Debug)]
// pub struct DeviceListState {
//     pub devices: Vec<ConnectedDevice>,
//     pub state_id: usize,
// }

// impl DeviceListState {
//     #[flutter_rust_bridge::frb(sync)]
//     pub fn get_device(&self, id: DeviceId) -> Option<ConnectedDevice> {
//         self.devices.iter().find(|device| device.id == id).cloned()
//     }
// }

// #[derive(Clone, Debug)]
// pub struct ConnectedDevice {
//     pub name: Option<String>,
//     // NOTE: digest should always be present in any device that is actually plugged in
//     pub firmware_digest: String,
//     pub latest_digest: Option<String>,
//     pub id: DeviceId,
// }

// impl ConnectedDevice {
//     pub fn ready(&self) -> bool {
//         self.name.is_some() && !self.needs_firmware_upgrade().0
//     }

//     pub fn needs_firmware_upgrade(&self) -> bool {
//         // We still want to have this return true even when we don't have firmware in the app so we
//         // know that the device needs a firmware upgrade (even if we can't give it to them).
//         Some(self.firmware_digest.as_str()) != self.latest_digest.as_deref()
//     }
// }

#[frb(sync)]
pub fn device_list_hello_world() -> DeviceId {
    DeviceId::from_bytes([42u8; 33])
}
