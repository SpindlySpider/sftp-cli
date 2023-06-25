use ssh2::{Sftp, Session, FileStat};
use std::{io::Read,net::TcpStream,fs,path::{Path, PathBuf},env};


struct sftp{
    hostname:String,
    port:String,
    host_port:String,
    username:String,
    password:String, //storing password here maybe a potentually bad idea
    session: Session,
    alive:bool,
    server_selected:bool,
    sftp:Sftp
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
        let mut server_selected:bool = false;
    
        let sftp:Sftp = session.sftp().unwrap();

        let sftp_client:sftp = sftp { hostname, port
            , host_port, username
            , password,session,
            alive,server_selected,
        sftp};

        return sftp_client;
}

fn sftp_main (sftp_client:&mut sftp){
    let mut input:String = String::new();
    while sftp_client.alive{
        std::io::stdin().read_line(&mut input).expect("failed to read input");

        sftp_choice(&input,sftp_client);
    }

}

fn list_cwd_dir(sftp_client:&sftp)->PathBuf{
    let path:PathBuf;
    if sftp_client.server_selected{
        path = sftp_client.sftp.realpath(Path::new(".")).unwrap();
    }
    else{
        path = env::current_dir().unwrap();
    }

    return path;
}

fn list_files_remote(){
    
}

fn list_files_client{

    
}

fn list_files(sftp_client:&sftp)-> fs::ReadDir{
    let files:Vec<(PathBuf, FileStat)>;
    let current_dir:&Path = list_cwd_dir(sftp_client).as_path();
    if sftp_client.server_selected{
    files = sftp_client.sftp.readdir(current_dir).unwrap();
    }
    else{
        files = fs::read_dir(current_dir).unwrap()    
    }

    return files;
}


fn sftp_choice(userinput:&String, sftp_client:&mut sftp)
// could make this return a string it might make it a bit easier to 
//read outputs when in flutter.
    {
    if userinput == "exit"{
        sftp_client.alive = false;
    }
    else if userinput =="cd"{

    }
    else if userinput == "ls"{  
    }
    else if userinput =="dir"{
        let path = list_cwd_dir(sftp_client);
        let path_str:&str = path.to_str().unwrap();
        println!("{}",path_str);
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