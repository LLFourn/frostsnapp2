use std::{ pin::Pin, time::Duration};

use flutter_rust_bridge::{frb, DartFnFuture};
pub use frostsnap_core::DeviceId;
use futures::{Stream, StreamExt as _};

use crate::frb_generated::StreamSink;






pub struct AndroidUsb {
    list_devices: Option<Box<dyn Fn() -> DartFnFuture<Vec<String>> + Send + Sync>>,
}


pub async fn start(mut usb_backend: UsbSerial, stream: StreamSink<Vec<String>>) {
    let mut events = usb_backend.inner.port_events();
    loop {
         match events.next().await {
            Some(ports) => { stream.add(ports); },
            None => break,
        }
    }
}


trait UsbSerialBackend {
    fn port_events(&mut self) -> Pin<Box<dyn Stream<Item=Vec<String>>>>;
}

#[frb(opaque)]
pub struct UsbSerial {
    inner: Box<dyn UsbSerialBackend + 'static>
}



impl UsbSerialBackend for AndroidUsb {
    fn port_events(&mut self) -> Pin<Box<dyn Stream<Item=Vec<String>>>> {
        let list_devices = self.list_devices.take().expect("can't start port events twice");
        let stream = futures::stream::unfold((), move |_| async {
            let devices = (list_devices)().await;
            Some((devices, ()))
        });

        Box::pin(stream)
    }
}

pub async fn start_android_usb(dart_callback: impl Fn() -> DartFnFuture<Vec<String>> + 'static + Send + Sync ) -> UsbSerial {
    UsbSerial {
        inner: Box::new(AndroidUsb { list_devices: Some(Box::new(dart_callback)) })
   }

}

#[frb(opaque)]
pub struct OrdinaryUsb {}

pub async fn start_ordinary_usb() -> OrdinaryUsb {
    OrdinaryUsb {}
}

impl UsbSerialBackend for OrdinaryUsb {
    fn port_events(&mut self) -> Pin<Box<dyn Stream<Item=Vec<String>>>> {

        let stream = futures::stream::unfold((), move |_| async {
            match tokio_serial::available_ports() {
                Ok(ports) => {
                    let names = ports
                        .iter()
                        .map(|port| port.port_name.clone())
                        .collect::<Vec<String>>();
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    Some((names, ()))
                },
                Err(_) => todo!(),
            }

        });

        Box::pin(stream)
    }
}

