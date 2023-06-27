mod structures;
use structures::sftp;
use structures::file_metadata;
use ssh2::{Sftp, Session, FileStat};

use std::fs::FileType;
use std::{net::TcpStream,fs::{self, ReadDir},path::{Path, PathBuf},env};




fn sftp_build(hostname:String,port:String,
    username:String,password:String)->sftp{
        let host_port:String = format!("{}:{}",&hostname.clone(),&port.clone());
        let tcp:TcpStream = std::net::TcpStream::connect(&host_port).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password(&username.clone(), &password.clone()).unwrap();
        assert!(session.authenticated());
        let alive:bool = true;
        let server_selected:bool = false;
        let folder_marker:String = String::from("📁");
    
        let sftp:Sftp = session.sftp().unwrap();

        let sftp_client:sftp = sftp { hostname, port
            , host_port, username
            , password,session,
            alive,server_selected,
        sftp,folder_marker};

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

fn output_files_string(files:&Vec<file_metadata>,sftp_client:&sftp)->Vec<String>{
    //used to output a string of all of the files
    //need to covnert filestat to a useable file
    let mut files_list:Vec<String> = Vec::new();
    for entity in files{
        if entity.filestat != None{//may cause error since it returns a option type
            //if this is a server file 
            let entity_option:Option<FileStat> = entity.filestat.clone();
            let entity_stat:FileStat = entity_option.unwrap();
            if entity_stat.is_dir(){
                //if this is a dir 
                let dir_string:String = String::from(format!("{}{}",sftp_client.folder_marker,entity.filepath.display()));
                files_list.push(dir_string);
            }
            else{
                //this is a file or somthing else but not a dir
                let dir_string:String = String::from(format!("{}",entity.filepath.display()));
                files_list.push(dir_string);
            }
        }
        else{
            let entity_stat:FileType = entity.filetype.unwrap();
            if entity_stat.is_dir(){
                let dir_string:String = String::from(format!("{}{}",sftp_client.folder_marker,entity.filepath.display()));
                files_list.push(dir_string);
            }
            else{
                let dir_string:String = String::from(format!("{}",entity.filepath.display()));
                files_list.push(dir_string);
            }

        }
    }

    return files_list;



}
fn list_files(sftp_client:&sftp)-> Vec<file_metadata>{
    //maybe just make this return a indivudal one for each type rather than trying to make a common file type
    //return vector of all files, can append a folder symbol if its a symbol 
    let mut files:Vec<file_metadata> = Vec::new();
    let store_cwd_dir:PathBuf = list_cwd_dir(sftp_client);
    let current_dir:&Path = store_cwd_dir.as_path();

    if sftp_client.server_selected{
        let server_files:Vec<(PathBuf, FileStat)>;

        server_files = sftp_client.sftp.readdir(current_dir).unwrap();//this turns of path buff,file stat
        for i in 0..server_files.len(){
            let temp_metadata_file:file_metadata = file_metadata{
                filepath: server_files[i].0.clone(),
                filestat:Some(server_files[i].1.clone()),
                size:None,
                file:None,
                filetype:None
                };
            files.push(temp_metadata_file);
        }
    }
    else{
        let file_path_list:ReadDir = fs::read_dir(current_dir).unwrap(); //returns pathbuff ;
        //ugh need to the readdir indexable to be able to get filepath 
        //then need to get filestat
        for entry_result in file_path_list{
            let entry = entry_result.unwrap();
            let entry_filepath = entry.path();
            let entry_metadata = entry_filepath.metadata().unwrap();
            let entry_file_type:std::fs::FileType = entry_metadata.file_type();
            let temp_metadata_file:file_metadata = file_metadata{
                filepath: entry_filepath,//may  cause a error since its trying to convert entrydir->pathbuf
                filetype:Some(entry_file_type),
                filestat:None,
                size:None,
                file:None
        };
        files.push(temp_metadata_file);
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
        let file_metadata = list_files(sftp_client);
        let output_list:Vec<String> = output_files_string(&file_metadata, sftp_client);
        for index in 0..output_list.len(){
            println!("{}",output_list[index]);
        } 
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

    let mut sftp_client:sftp = sftp_build(hostname,
        port,
        username,
        password);
    sftp_main(&mut sftp_client);
}



