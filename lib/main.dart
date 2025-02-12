import 'package:flutter/material.dart';
import 'package:frostsnapp2/src/rust/api/device_list.dart';
import 'package:frostsnapp2/src/rust/frb_generated.dart';
import 'package:usb_serial/usb_serial.dart';
import 'dart:io';

Future<void> main() async {
  await RustLib.init();
  final UsbSerialImpl usbSerial;

  if (Platform.isAndroid) {
    debugPrint("android");
    final List<String> devices = [];
    usbSerial = await usbAndroid(listDevices: () async {
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
    usbSerial = await usbOrdinary();
  }

  final devices = start(
    usbBackend: usbSerial,
  );
  runApp(MyApp(devices: devices));
}

class MyApp extends StatelessWidget {
  final Stream<List<String>> devices;
  MyApp({super.key, required this.devices});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: StreamBuilder(
            stream: devices,
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
