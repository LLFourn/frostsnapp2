use std::sync::Arc;

use flutter_rust_bridge::{DartFnFuture, DartOpaque};

use crate::{
    frb_generated::StreamSink,
    usb::{AndroidUsb, OrdinaryUsb, start_usb_loop},
};

// #[frb(ignore)]



pub async fn start_usb_android(
    list_devices: impl Fn() -> DartFnFuture<Vec<String>> + Send + Sync + 'static,
    open_port: impl Fn(String) -> DartFnFuture<DartOpaque> + Send + Sync + 'static,
    poll_port: impl Fn(DartOpaque) -> DartFnFuture<Vec<u8>> + Send + Sync + 'static,
    write_port: impl Fn(DartOpaque, Vec<u8>) -> DartFnFuture<()> + Send + Sync + 'static,
    stream: StreamSink<Vec<u8>>,
) {
    start_usb_loop(
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

pub async fn start_usb_ordinary(stream: StreamSink<Vec<u8>>) {
    start_usb_loop(OrdinaryUsb {}, stream).await;
    // UsbSerialImpl {
    //     inner: Box::new(OrdinaryUsb {}),
    // }
}
