use std::{collections::HashSet, sync::Arc};

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use futures::{pin_mut, StreamExt as _};

use crate::{
    frb_generated::StreamSink,
    usb::{AndroidUsb, OrdinaryUsb, UsbSerialBackend},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const MAGIC_BYTES_LEN: usize = 7;
const MAGICBYTES_RECV_UPSTREAM: [u8; MAGIC_BYTES_LEN] = [0xff, 0x5d, 0xa3, 0x85, 0xd4, 0xee, 0x5a];

async fn start<T: UsbSerialBackend>(mut usb_backend: T, stream: StreamSink<Vec<String>>) {
    let mut open_ports = HashSet::<String>::new();
    let events = usb_backend.port_events();
    pin_mut!(events);
    while let Some(ports) = events.next().await {
        // TODO: Fix this up later.
        for port in &ports {
            if open_ports.contains(port) {
                continue;
            }
            let stream = usb_backend.open_port(port.clone()).await;
            open_ports.insert(port.clone());
            let (mut read_half, mut write_half) = tokio::io::split(stream);
            write_half
                .write_all(&MAGICBYTES_RECV_UPSTREAM)
                .await
                .expect("TODO");
            loop {
                let mut read_buf = vec![0_u8; 1];
                let read = read_half
                    .read_exact(read_buf.as_mut_slice())
                    .await
                    .expect("TODO");
                if read == 0 {
                    break;
                }
                println!("WE READ: {:?}", read_buf);
            }
        }
        stream.add(ports).unwrap();
    }
}

pub async fn start_usb_android(
    list_devices: impl Fn() -> DartFnFuture<Vec<String>> + Send + Sync + 'static,
    open_port: impl Fn(String) -> DartFnFuture<DartOpaque> + Send + Sync + 'static,
    poll_port: impl Fn(DartOpaque) -> DartFnFuture<Vec<u8>> + Send + Sync + 'static,
    write_port: impl Fn(DartOpaque, Vec<u8>) -> DartFnFuture<()> + Send + Sync + 'static,
    stream: StreamSink<Vec<String>>,
) {
    start(
        AndroidUsb {
            list_devices: Some(Box::new(list_devices)),
            open_port: Arc::new(open_port),
            poll_port: Arc::new(poll_port),
            write_port: Arc::new(write_port),
        },
        stream,
    )
    .await;
}

pub async fn start_usb_ordinary(stream: StreamSink<Vec<String>>) {
    start(OrdinaryUsb {}, stream).await;
    // UsbSerialImpl {
    //     inner: Box::new(OrdinaryUsb {}),
    // }
}
