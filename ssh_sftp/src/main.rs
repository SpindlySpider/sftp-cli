use ssh2::{Sftp, Session};
use std::{io::{Read, self},net::{ TcpStream}};

struct sftp{
    hostname:String,
    port:String,
    host_port:String,
    username:String,
    password:String, //storing password here maybe a potentually bad idea
    session: Session,
    alive:bool,
    server_selected:bool,
}

fn sftp_build(hostname:String,port:String,
    username:String,password:String)->sftp{
        let host_port:String = format!("{}:{}",&hostname.clone(),&port.clone());
        let tcp:TcpStream = std::net::TcpStream::connect(&host_port).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password(&username.clone(), &password.clone()).unwrap();
        assert!(session.authenticated());
        let mut alive:bool = true;
        let mut server_selected:bool = true;
    
        let sftp = session.sftp().unwrap();

        let sftp_client:sftp = sftp { hostname: hostname, port: port
            , host_port: host_port, username: username
            , password: password,session:session,
            alive:alive,server_selected:server_selected};

        return sftp_client;
}

fn sftp_main (sftp_client:&mut sftp){
    let mut input:String = String::new();
    while sftp_client.alive{
        std::io::stdin().read_line(&mut input).expect("failed to read input");

        sftp_choice(&input,sftp_client);
    }

}

fn sftp_choice(userinput:&String, sftp_client:&mut sftp)-> i8
    {
    let mut return_value:i8 = 0;
    // if it returns a 0 then its correct
    //anything else is either a error condition or a condition
    if userinput == "exit"{
        sftp_client.alive = false;
        return return_value;
    }
    else if userinput == "ls"{  
        return return_value;
    }
    else if userinput == "sw"{
        let invert:bool =sftp_client.server_selected;
        sftp_client.server_selected = !invert;
        return return_value;
    }
    else{
        return  return_value;
    }

}


fn main() {
    println!("Hello, world!");
}