import "package:flutter/material.dart";
import "pages/remote_host_login.dart";
import 'dart:ffi';


void main(){
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: IpLogin(),
    );
  }
}