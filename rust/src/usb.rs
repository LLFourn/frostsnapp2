use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use futures::{pin_mut, Stream};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_serial::SerialStream;

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
    ) -> impl Future<Output = impl AsyncRead + AsyncWrite + Send> + Send;
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
    ) -> impl Future<Output = impl AsyncRead + AsyncWrite + Send> + Send {
        struct A {
            port: DartOpaque,
            poll_port: Arc<dyn Fn(DartOpaque) -> DartFnFuture<Vec<u8>> + Send + Sync>,
            write_port: Arc<dyn Fn(DartOpaque, Vec<u8>) -> DartFnFuture<()> + Send + Sync>,

            current_read: Option<DartFnFuture<Vec<u8>>>,
        }
        impl AsyncRead for A {
            fn poll_read(
                self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut tokio::io::ReadBuf<'_>,
            ) -> std::task::Poll<std::io::Result<()>> {
                match self.get_mut().current_read.take() {
                    Some(fut) => fut,
                    None => {
                        //(self.poll_port)(self.port);
                        todo!()
                    },
                };
                todo!()
            }
        }
        impl AsyncWrite for A {
            fn poll_write(
                self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<Result<usize, std::io::Error>> {
                todo!()
            }

            fn poll_flush(
                self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Result<(), std::io::Error>> {
                todo!()
            }

            fn poll_shutdown(
                self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Result<(), std::io::Error>> {
                todo!()
            }
        }

        let portFut = self.open_port(name);
        async {

        }
        futures::future::ready(A {
        })
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
    ) -> impl Future<Output = impl AsyncRead + AsyncWrite + Send> + Send {
        let stream = SerialStream::open(&tokio_serial::new(name, 14_400))
            .expect("TODO: Figure this out later");
        futures::future::ready(stream)
    }
}
