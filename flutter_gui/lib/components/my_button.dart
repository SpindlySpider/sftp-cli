import "package:flutter/material.dart";

class MyButton extends StatelessWidget {
  final String buttonText;
  final Function()? onTap;
  const MyButton({super.key,
  required  this.buttonText,
  required this.onTap,
  });
  
  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: EdgeInsets.all(15.0),
        margin: EdgeInsets.symmetric(horizontal: 25.0 ),
        decoration: BoxDecoration(color: Colors.blueGrey,
        borderRadius: BorderRadius.circular(12),
    
    
        ),
        child: Center(
          child: Text(buttonText),
        ),
    
    
    
      ),
    );
  }
}