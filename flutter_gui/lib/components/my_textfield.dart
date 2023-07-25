import "package:flutter/material.dart";

class MyTextField extends StatelessWidget {
  const MyTextField({super.key});

  @override
  Widget build(BuildContext context) {
    return const Padding(
              padding: EdgeInsets.symmetric(horizontal:25.0 ),
              child: TextField(decoration: InputDecoration(
                enabledBorder: OutlineInputBorder(borderSide: BorderSide(
                  color: Colors.white
                )),
                focusedBorder: OutlineInputBorder(borderSide: BorderSide(
                  color: Color.fromARGB(255, 187, 170, 170)
                ),

                
            
                ),
                fillColor: Color.fromARGB(76, 255, 254, 254),
                filled: true,
              ),
              ),
            );
  }
}