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

fn sftp_choice(userinput:&String, sftp_client:&mut sftp)
    {
    if userinput == "exit"{
        sftp_client.alive = false;
    }
    else if userinput == "ls"{  
    }
    else if userinput == "sw"{
        let invert:bool =sftp_client.server_selected;
        sftp_client.server_selected = !invert;
    }
    else{
    }

}


fn main() {
    let mut sftp_client:sftp = sftp_build(String::from("192.168.1.166"),
     String::from("blaa"),
      String::from("bladsadsad"),
      String::from( "blaa"));
    sftp_main(&mut sftp_client);
}