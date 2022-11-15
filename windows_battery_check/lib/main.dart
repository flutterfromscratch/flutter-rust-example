import 'package:flutter/material.dart';
import 'package:windows_battery_check/bridge_generated.dart';
import 'package:windows_battery_check/native.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        title: 'Flutter Demo',
        theme: ThemeData(
          // This is the theme of your application.
          //
          // Try running your application with "flutter run". You'll see the
          // application has a blue toolbar. Then, without quitting the app, try
          // changing the primarySwatch below to Colors.green and then invoke
          // "hot reload" (press "r" in the console where you ran "flutter run",
          // or simply save your changes to "hot reload" in a Flutter IDE).
          // Notice that the counter didn't reset back to zero; the application
          // is not restarted.
          primarySwatch: Colors.blue,
        ),
        home: const HomePage());
  }
}

class HomePage extends StatelessWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Flutter Battery Windows"),
      ),
      body: Center(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            FutureBuilder(
              future: api.getBatteryStatus(),
              builder: (context, data) {
                return Text(
                  'System has battery present: ${data.data}',
                  style: TextStyle(
                      color: (data.data ?? false) ? Colors.green : Colors.red),
                );
              },
            ),
            StreamBuilder(
              stream: api.batteryEventStream(),
              builder: (context, data) {
                if (data.hasData) {
                  return Column(
                    children: [
                      Text(
                          "Charge rate in milliwatts: ${data.data!.chargeRatesInMilliwatts.toString()}"),
                      Text(
                          "Design capacity in milliwatts: ${data.data!.designCapacityInMilliwattHours.toString()}"),
                      Text(
                          "Full charge in milliwatt hours: ${data.data!.fullChargeCapacityInMilliwattHours.toString()}"),
                      Text(
                          "Remaining capacity in milliwatts: ${data.data!.remainingCapacityInMilliwattHours}"),
                      Text("Battery status is ${data.data!.status}")
                    ],
                  );
                }
                return Column(
                  children: [
                    Text("Waiting for a battery event."),
                    Text(
                        "If you have a desktop computer with no battery, this event will never come..."),
                    CircularProgressIndicator(),
                  ],
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}
