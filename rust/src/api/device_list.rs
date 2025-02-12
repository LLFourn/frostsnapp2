
use flutter_rust_bridge::{frb, DartFnFuture};
use futures::{ StreamExt as _};

use crate::{frb_generated::StreamSink, usb::{AndroidUsb, OrdinaryUsb}};


pub async fn start(mut usb_backend: UsbSerialImpl, stream: StreamSink<Vec<String>>) {
    let mut events = usb_backend.inner.port_events();
    loop {
         match events.next().await {
            Some(ports) => { stream.add(ports).unwrap(); },
            None => break,
        }
    }
}


#[frb(opaque)]
pub struct UsbSerialImpl {
    inner: Box<dyn crate::usb::UsbSerialBackend + Send + Sync>,
}


pub async fn usb_android(list_devices: impl Fn() -> DartFnFuture<Vec<String>> + Send + Sync + 'static) -> UsbSerialImpl {
    UsbSerialImpl {
        inner: Box::new(AndroidUsb { list_devices: Some(Box::new(list_devices)) })
    }

}

pub async fn usb_ordinary() -> UsbSerialImpl {
    UsbSerialImpl { inner: Box::new(OrdinaryUsb {  }) }
}
