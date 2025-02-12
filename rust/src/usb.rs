use std::{collections::HashSet, future::Future,  sync::Arc, time::Duration};

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use futures::{future, pin_mut, Stream, StreamExt as _};
use tokio::io::{ AsyncReadExt,  AsyncWriteExt};
use tokio_serial::SerialStream;

use crate::frb_generated::StreamSink;

pub trait UsbPort {
    fn read_port(&mut self) ->  impl Future<Output=Vec<u8>> + Send;
    fn write_port(&mut self, bytes: Vec<u8>) -> impl Future<Output=()> + Send;
}


pub struct AndroidUsb {
    pub(crate) list_devices: Option<Box<dyn Fn() -> DartFnFuture<Vec<String>> + Send + Sync>>,
    pub(crate) open_port: Arc<dyn Fn(String) -> DartFnFuture<DartOpaque> + Send + Sync>,
    pub(crate) poll_port: Arc<dyn Fn(DartOpaque) -> DartFnFuture<Vec<u8>> + Send + Sync>,
    pub(crate) write_port: Arc<dyn Fn(DartOpaque, Vec<u8>) -> DartFnFuture<()> + Send + Sync>,
}

pub struct OrdinaryUsb {}

pub trait UsbSerialBackend {
    fn port_events(&mut self) -> impl Stream<Item = Vec<String>> + Send + 'static;
    fn open_port(
        &self,
        name: String,
    ) -> impl Future<Output = impl UsbPort + Send> + Send;
}

impl UsbSerialBackend for AndroidUsb {
    fn port_events(&mut self) -> impl Stream<Item = Vec<String>> + Send + 'static {
        let list_devices = self
            .list_devices
            .take()
            .expect("can't start port events twice");
        async_stream::stream! {
           loop {
               let devices = (list_devices)().await;
               yield devices;
           }
        }
    }

    fn open_port(
        &self,
        name: String,
    ) -> impl Future<Output = impl UsbPort + Send> + Send {
        struct DartPort {
            port: DartOpaque,
            poll_port: Arc<dyn Fn(DartOpaque) -> DartFnFuture<Vec<u8>> + Send + Sync>,
            write_port: Arc<dyn Fn(DartOpaque, Vec<u8>) -> DartFnFuture<()> + Send + Sync>,
        }

        impl UsbPort for DartPort {
            fn read_port(&mut self) ->  impl Future<Output=Vec<u8>> {
                async {
                    (self.poll_port)(self.port.clone()).await
                }
            }

            fn write_port(&mut self, bytes: Vec<u8>) -> impl Future<Output=()> {
                (self.write_port)(self.port.clone(),bytes)
            }

        }

        async {
            let port = (self.open_port)(name).await;
            DartPort {
                port,
                poll_port: Arc::clone(&self.poll_port),
                write_port: Arc::clone(&self.write_port),
            }
        }
    }
}

impl UsbSerialBackend for OrdinaryUsb {
    fn port_events(&mut self) -> impl Stream<Item = Vec<String>> + Send + 'static {
        futures::stream::unfold((), move |_| async {
            match tokio_serial::available_ports() {
                Ok(ports) => {
                    let names = ports
                        .iter()
                        .map(|port| port.port_name.clone())
                        .collect::<Vec<String>>();
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    Some((names, ()))
                }
                Err(_) => todo!(),
            }
        })
    }

    fn open_port(
        &self,
        name: String,
    ) -> impl Future<Output = impl UsbPort + Send> + Send {
        let stream = SerialStream::open(&tokio_serial::new(name, 14_400))
            .expect("TODO: Figure this out later");

        future::ready(OrdinaryUsbPort {
            inner: stream
        })

    }
}

pub struct OrdinaryUsbPort {
    inner: SerialStream,
}

impl UsbPort for OrdinaryUsbPort {
    async fn read_port(&mut self) ->  Vec<u8> {
        let mut buf = [0u8;256];
        let read = self.inner.read(&mut buf[..]).await.expect("todo");
        buf[..read].to_vec()
    }

    async fn write_port(&mut self, bytes: Vec<u8>)  {
        self.inner.write_all(&bytes[..]).await.expect("TODO")
    }
}

const MAGIC_BYTES_LEN: usize = 7;
const MAGICBYTES_RECV_UPSTREAM: [u8; MAGIC_BYTES_LEN] = [0xff, 0x5d, 0xa3, 0x85, 0xd4, 0xee, 0x5a];

pub async fn start_usb_loop<T: UsbSerialBackend>(mut usb_backend: T, stream: StreamSink<Vec<u8>>) {
    let mut open_ports = HashSet::<String>::new();
    let events = usb_backend.port_events();
    pin_mut!(events);
    while let Some(ports) = events.next().await {
        // TODO: Fix this up later.
        for port_name in &ports {
            if open_ports.contains(port_name) {
                continue;
            }
            let mut port = usb_backend.open_port(port_name.clone()).await;
            open_ports.insert(port_name.clone());

            port.write_port(MAGICBYTES_RECV_UPSTREAM.to_vec()).await;

            loop {
                let bytes = port.read_port().await;
                stream.add(bytes).unwrap();
            }
        }
    }
    todo!()
}
