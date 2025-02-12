use std::{pin::Pin, time::Duration};

use flutter_rust_bridge::DartFnFuture;
use futures::Stream;


pub struct AndroidUsb {
    pub(crate) list_devices: Option<Box<dyn Fn() -> DartFnFuture<Vec<String>> + Send + Sync>>
}


pub struct OrdinaryUsb {}


pub trait UsbSerialBackend {
    fn port_events(&mut self) -> Pin<Box<dyn Stream<Item = Vec<String>> + Send >>;
}

impl UsbSerialBackend for AndroidUsb {
    fn port_events(&mut self) -> Pin<Box<dyn Stream<Item=Vec<String>> + Send >> {
        let list_devices = self.list_devices.take().expect("can't start port events twice");
        let stream = async_stream::stream! {
           loop {
               let devices = (list_devices)().await;
               yield devices;
           }
        };

        Box::pin(stream)
    }
}





impl UsbSerialBackend for OrdinaryUsb {
    fn port_events(&mut self) -> Pin<Box<dyn Stream<Item=Vec<String>> + Send >> {

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
