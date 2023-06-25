mod structures;
use structures::sftp;
use structures::file_metadata;
use ssh2::{Sftp, Session, FileStat};
use std::{io::Read,net::TcpStream,fs::{self, ReadDir},path::{Path, PathBuf},env};




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

fn server_output_files(files:&Vec<file_metadata>){
    //used to output a string of all of the files
    //need to covnert filestat to a useable file
    let mut files_list:Vec<String>;



}
fn list_files(sftp_client:&sftp)-> Vec<file_metadata>{
    //maybe just make this return a indivudal one for each type rather than trying to make a common file type
    //return vector of all files, can append a folder symbol if its a symbol 
    let mut files:Vec<file_metadata>;
    let current_dir:&Path = list_cwd_dir(sftp_client).as_path();
    if sftp_client.server_selected{
        let server_files:Vec<(PathBuf, FileStat)>;

        server_files = sftp_client.sftp.readdir(current_dir).unwrap();//this turns of path buff,file stat
        for i in 0..server_files.len(){
            let mut temp_metadata_file:file_metadata = file_metadata{
                filepath: server_files[i].0,
                filestat:Some(server_files[i].1)
                };
            files.push(temp_metadata_file);
        }
    }
    else{
        let file_path_list:ReadDir;
        let file_status_list:FileStat;
        file_path_list = fs::read_dir(current_dir).unwrap(); //returns pathbuff 
        //ugh need to the readdir indexable to be able to get filepath 
        //then need to get filestat
        for i in 0..file_path_list.count(){
            file_path_list
        }
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