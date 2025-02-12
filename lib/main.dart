import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:frostsnapp2/src/rust/api/device_list.dart';
import 'package:frostsnapp2/src/rust/frb_generated.dart';
import 'package:usb_serial/usb_serial.dart';
import 'dart:io';

Future<void> main() async {
  await RustLib.init();
  //final UsbSerialImpl usbSerial;

  final Stream<List<String>> devicesStream;
  if (Platform.isAndroid) {
    debugPrint("android");
    final List<String> devices = [];
    devicesStream = startUsbAndroid(listDevices: () async {
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
    }, openPort: (portName) async {
      final deviceList = await UsbSerial.listDevices();
      final device = deviceList
          .firstWhereOrNull((device) => device.deviceName == portName);
      if (device == null) {
        throw "Device $portName is not connected";
      } else {
        var port = await device.create();
        return port!;
      }
    }, pollPort: (portObj) async {
      final port = portObj as UsbPort;
      return await port.inputStream!.first;
    }, writePort: (portObj, data) async {
      final port = portObj as UsbPort;
      return await port.write(data);
    });
  } else {
    devicesStream = startUsbOrdinary();
    //usbSerial = await usbOrdinary();
  }

  //final devices = start(
  //  usbBackend: usbSerial,
  //);
  runApp(MyApp(devices: devicesStream));
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
