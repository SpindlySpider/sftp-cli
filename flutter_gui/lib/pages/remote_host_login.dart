import "package:flutter/material.dart";
import "package:flutter_gui/components/my_button.dart";
import "package:flutter_gui/components/my_textfield.dart";


void main(){
  


}

void startConnection(){
  print("fdsfsd");
}

class IpLogin extends StatelessWidget {
  IpLogin({super.key});

  final ipTextEditController = TextEditingController();
  final portTextEditController = TextEditingController();
  final usernameTextEditController = TextEditingController();
  final passwordTextEditController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color.fromARGB(255, 50, 44, 53),
      body: SafeArea(
        child: Center(
          child: Column(children:  [
            //general explaination
            const SizedBox(height: 50,),

            const Text("please enter IP, Port (if not port 22), username and password",
            style: TextStyle(color:Color.fromARGB(255, 209, 206, 199),
            fontSize: 16,),
            ),

            const SizedBox(height: 10,),




            
            
              
            //ip text field
              MyTextField(
                controller:ipTextEditController,
                hintText: "ip address" ,
                obscureText: false,

                ),
              const SizedBox(height: 10,),
            //port text feild
              MyTextField(
                controller:portTextEditController,
                hintText: "port" ,
                obscureText: false,

                ),
              const SizedBox(height: 10,),
            //username
              MyTextField(
                controller:usernameTextEditController,
                hintText: "username" ,
                obscureText: false,

                ),
              const SizedBox(height: 10,),
            //password text feild
              MyTextField(
                controller:passwordTextEditController,
                hintText: "password" ,
                obscureText: true,
                ),
            //connect button
            const SizedBox(height: 10,),

            const MyButton(buttonText: "connect",onTap: startConnection,),
              
          ],
          ),
        ),
      ),
    );
  }
}