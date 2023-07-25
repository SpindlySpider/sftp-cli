import "package:flutter/material.dart";
import "package:flutter_gui/components/my_textfield.dart";


void main(){
  


}

class IpLogin extends StatelessWidget {
  const IpLogin({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      backgroundColor: Color.fromARGB(255, 19, 20, 105),
      body: SafeArea(
        child: Center(
          child: Column(children:  [
            //general explaination
            SizedBox(height: 50,),

            Text("please enter IP, Port (if not port 22), username and password",
            style: TextStyle(color:Color.fromARGB(255, 209, 206, 199),
            fontSize: 16,),
            ),

            SizedBox(height: 50,),




            
            
              
            //ip text field
              MyTextField(),
              SizedBox(height: 50,),
            //port text feild
              MyTextField(),
              SizedBox(height: 50,),
            //username
              MyTextField(),
              SizedBox(height: 50,),
            //password text feild
              MyTextField(),
            //connect button 
              
          ],
          ),
        ),
      ),
    );
  }
}