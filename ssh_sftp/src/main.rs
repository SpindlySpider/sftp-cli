use ssh2::{Sftp, Session};
use std::{io::{Read, self}, collections::linked_list, result};
fn main() {
    println!("Hello, world!");
}

fn stfp_initate_connect(hostname:String,port:String,username:String,password:String){

    let host_port:String = format!("{}:{}",&hostname,&port);
    let tcp = std::net::TcpStream::connect(host_port).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(&username, &password).unwrap();
    assert!(session.authenticated());

    let mut sftp = session.sftp().unwrap();

}

fn sftp_main (sftp_session:&Session,ssh2_session:&Session){
    let mut alive:bool = true;
    let mut server_selected:bool = true;
    let mut input = String::new();
    while alive{
        std::io::stdin().read_line(&mut input).expect("failed to read input");
        sftp_choice(userinput, sftp_session, ssh2_session);

    }

}

fn sftp_choice(userinput:String, sftp_session:Option<&Session>,
    ssh2_session:Option<&Session>)-> i8
    {
    let mut return_value:i8 = 0;
    // if it returns a 0 then its correct
    //anything else is either a error condition or a condition
    if userinput == "exit"{
        return_value = 1;
        return return_value;
    }
    else if userinput == "ls"{
        return return_value;
    }
    else{
        return  return_value;
    }

}