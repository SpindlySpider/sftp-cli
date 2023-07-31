import "package:flutter/material.dart";

class MyTextField extends StatelessWidget {
    final TextEditingController controller;
    final String hintText;
    final bool obscureText;
    
  const MyTextField({
    super.key,
    required this.controller,
    required this.hintText,
    required this.obscureText,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
              padding:const EdgeInsets.symmetric(horizontal:25.0 ),
              child: TextField(
                controller: controller,
                obscureText: obscureText,
                decoration: InputDecoration(
                enabledBorder:const OutlineInputBorder(borderSide: BorderSide(
                  color: Color.fromARGB(255, 252, 241, 255)
                )),
                focusedBorder:const OutlineInputBorder(borderSide: BorderSide(
                  color: Color.fromARGB(255, 179, 170, 187)
                ),

                
            
                ),
                fillColor:const Color.fromARGB(75, 255, 255, 255),
                filled: true,
                hintText: hintText,
                
              ),
              ),
            );
  }
}