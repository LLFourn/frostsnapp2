import 'package:flutter/material.dart';
import 'package:frostsnapp2/src/rust/api/device_list.dart';
import 'package:frostsnapp2/src/rust/frb_generated.dart';
import 'package:usb_serial/usb_serial.dart';
import 'dart:io';

Future<void> main() async {
  await RustLib.init();
  final PortLister portLister;

  if (Platform.isAndroid) {
    debugPrint("android");
    final List<String> devices = [];
    portLister = await startAndroidUsb(dartCallback: () async {
      final usbStream = UsbSerial.usbEventStream!;
      final UsbEvent event = await usbStream.first;
      if (event.event == UsbEvent.ACTION_USB_DETACHED) {
        if (event.device != null) {
          devices.remove(event.device!.deviceName);
        }
        {
          debugPrint("device didn't have name");
        }
      } else if (event.event == UsbEvent.ACTION_USB_ATTACHED) {
        if (event.device != null) {
          devices.add(event.device!.deviceName);
        }
        {
          debugPrint("device didn't have name");
        }
      }

      debugPrint("end callback");
      return devices;
    });
  } else {
    portLister = await startOrdinaryUsb();
  }
  runApp(MyApp(portLister: portLister));
}

class MyApp extends StatelessWidget {
  final PortLister portLister;
  MyApp({super.key, required this.portLister});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: StreamBuilder(
            stream: portLister.subPorts(),
            builder: (ctx, snap) {
              if (!snap.hasData) {
                return CircularProgressIndicator();
              }

              return Text("devices: ${snap.data!.toString()}");
            }),
      ),
    );
  }
}
