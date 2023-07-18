mod structures;
use ssh2::File;
use structures::sftp;
use structures::file_metadata;
use ssh2::{Sftp, Session, FileStat};
use std::fs::FileType;
use std::io::Read;
use std::io::Write;
use std::{net::TcpStream,fs::{self, ReadDir},path::{Path, PathBuf},env};
use rpassword;


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
        let cli_leader:String = String::from("~");
        let sftp:Sftp = session.sftp().unwrap();

        let sftp_client:sftp = sftp { hostname, port
            , host_port, username
            , password,session,
            alive,server_selected,
        sftp,folder_marker,
        cli_leader};

        return sftp_client;
}
fn split_to_vec_string(raw_string:&str)->Vec<&str>{
    let temp_str = raw_string.split(" ");
    let mut return_vec:Vec<&str> = Vec::new();
    for string in temp_str{
        return_vec.push(string);
    }
    return return_vec;

}

fn sftp_main (sftp_client:&mut sftp){
    println!("connection established");
    let mut remote_cwd:PathBuf = sftp_client.sftp.realpath(Path::new(".")).unwrap();
    while sftp_client.alive{
        let cwd:String = format!("{}{}", list_cwd_dir(sftp_client,remote_cwd.to_path_buf()).display(),sftp_client.cli_leader);
        print!("{}",cwd);
        std::io::stdout().flush(); // allows print! to active without taking input buffer
        let mut raw_input:String = String::new();
        std::io::stdin().read_line(&mut raw_input).expect("failed to read input");
        raw_input = raw_input.trim().to_string();
        let input = split_to_vec_string(&raw_input);
        sftp_choice(&input,sftp_client,&mut remote_cwd);

    }

}

fn list_cwd_dir(sftp_client:&sftp,remote_cwd: PathBuf)->PathBuf{
    let path:PathBuf;
    if sftp_client.server_selected{
        path = sftp_client.sftp.realpath(remote_cwd.as_path()).unwrap();
    }
    else{
        path = env::current_dir().unwrap();
    }

    return path;
}

fn list_local_host_cwd()->PathBuf{
    let path:PathBuf =env::current_dir().unwrap();
    return path;
}
fn output_files_string(files:&Vec<file_metadata>,sftp_client:&sftp)->Vec<String>{
    //used to output a string of all of the files
    let mut files_list:Vec<String> = Vec::new();
    for entity in files{
        if entity.filestat != None{
            //if this is a server file 
            let entity_option:Option<FileStat> = entity.filestat.clone();
            let entity_stat:FileStat = entity_option.unwrap();
            if entity_stat.is_dir(){
                //if this is a dir 
                let server_folder_name = entity.filepath.file_name().unwrap() ;
                let folder_name = server_folder_name.to_str().unwrap();
                let dir_string:String = String::from(format!("{}{}",sftp_client.folder_marker,folder_name));
                files_list.push(dir_string);
            }
            else{
                //this is a file or somthing else but not a dir
                let server_file_name = entity.filepath.file_name().unwrap() ;
                let file_name = server_file_name.to_str().unwrap();
                let dir_string:String = String::from(format!("{}",file_name));
                files_list.push(dir_string);
            }
        }
        else{
            let entity_stat:FileType = entity.filetype.unwrap();
            if entity_stat.is_dir(){
                let os_folder_name = entity.filepath.file_name().unwrap() ;
                let folder_name = os_folder_name.to_str().unwrap();
                let dir_string:String = String::from(format!("{}{}",sftp_client.folder_marker,folder_name));
                files_list.push(dir_string);
            }
            else{
                let os_file_name = entity.filepath.file_name().unwrap() ;
                let file_name = os_file_name.to_str().unwrap();
                let dir_string:String = String::from(format!("{}",file_name));
                files_list.push(dir_string);
            }

        }
    }

    return files_list;



}
fn list_files(sftp_client:&sftp,remote_cwd:PathBuf)-> Vec<file_metadata>{
    //maybe just make this return a indivudal one for each type rather than trying to make a common file type
    //return vector of all files, can append a folder symbol if its a symbol 
    let mut files:Vec<file_metadata> = Vec::new();
    let store_cwd_dir:PathBuf = list_cwd_dir(sftp_client,remote_cwd);
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

fn change_dir(sftp_client:&mut sftp, dir_to_open:&str, remote_cwd:&mut PathBuf){
    //can use a varible to keep track of current path
    let current_dir = list_cwd_dir(sftp_client,remote_cwd.to_path_buf());
    let abosultepath = current_dir.join(dir_to_open);
    if sftp_client.server_selected{
        match sftp_client.sftp.readdir(abosultepath.as_path()){
            Ok(read_dir_buffer)=>{
                *remote_cwd = abosultepath;
            }
            Err(err)=>{
                println!("invalid dir")
            }
        }

    }
    else{
        match std::env::set_current_dir(abosultepath.as_path()){
            Ok(read_dir_buffer)=>{
                std::env::set_current_dir(abosultepath.as_path()).unwrap();
            }
            Err(err)=>{
                println!("invalid dir")
            }
        }
    }
}

fn vaild_file(sftp_client:&sftp,file_to_check:&str,remote_cwd: PathBuf)->bool{
    //vaildates if a file actually exists 
    let current_dir = list_cwd_dir(sftp_client,remote_cwd.to_path_buf());
    let abosultepath = current_dir.join(file_to_check);
    if sftp_client.server_selected{
        match sftp_client.sftp.open(abosultepath.as_path()){
            Ok(file_buffer)=>{
                return true;
            }
            Err(err)=>{
                return false;
            }
        }

    }
    else{
        match std::fs::File::open(abosultepath){
            Ok(read_dir_buffer)=>{
                return true;
            }
            Err(err)=>{
                return false;
            }
        }
    }
}

fn download(sftp_client:&mut sftp, entry_to_download:&str, local_file_path:Option<PathBuf>,remote_cwd:&mut PathBuf){
    //this will only take from remote host 

    // need to seprately transfer metadata to the file
    if vaild_file(sftp_client, entry_to_download, remote_cwd.to_path_buf()){
        let current_dir = list_cwd_dir(sftp_client,remote_cwd.to_path_buf());
        let abosultepath = current_dir.join(entry_to_download);
        //let file_size = sftp_client.sftp.lstat(abosultepath.as_ref()).unwrap().size.unwrap();
        let mut file_buffer = Vec::new();
        let mut remote_file = sftp_client.sftp.open(&abosultepath).unwrap();
        let mut loop_alive:bool = true;
        //let file_permissions = sftp_client.sftp.lstat(abosultepath.as_ref()).unwrap().perm.unwrap();
        //let remote_file_metadata = remote_file.stat().unwrap();
        while loop_alive{
            let bytes_read = remote_file.read_to_end(&mut file_buffer).unwrap();
            if bytes_read == 0{
                //finsihed reading file
                loop_alive = false;
            }
            else{
                //can use this to see the status of download :)
            }
        }
        let mut local_file:fs::File;
        if local_file_path == None{
            // if there is no specified path then download to cwd
            let mut local_file_path = list_local_host_cwd();
            local_file_path.push(entry_to_download);
            local_file = fs::File::create(local_file_path).unwrap();
        }
        else{
            local_file = fs::File::create(local_file_path.unwrap()).unwrap();
        }
        local_file.write_all(&file_buffer).unwrap();
        println!("successful download")
    }
    else{
        println!("not a vaild file");
    }
}



fn upload(sftp_client:&mut sftp, entry_to_upload:&str, remote_file_path:Option<PathBuf>,remote_cwd:&mut PathBuf){
    //this will only take from local host 

    // need to seprately transfer metadata to the file
    if vaild_file(sftp_client, entry_to_upload, list_local_host_cwd().to_path_buf()){
        let current_dir = list_cwd_dir(sftp_client,list_local_host_cwd().to_path_buf());
        let abosultepath = current_dir.join(entry_to_upload);
        //let file_size = sftp_client.sftp.lstat(abosultepath.as_ref()).unwrap().size.unwrap();
        let mut file_buffer = Vec::new();
        let mut local_file = fs::File::open(abosultepath).unwrap();
        let mut loop_alive:bool = true;
        while loop_alive{
            let bytes_read = local_file.read_to_end(&mut file_buffer).unwrap();
            if bytes_read == 0{
                //finsihed reading file
                loop_alive = false;
            }
            else{
                //can use this to see the status of download :)
            }
        }
        let mut remote_file:File;
        if remote_file_path == None{
            // if there is no specified path then download to cwd
            let mut remote_file_path = remote_cwd.clone();
            remote_file_path.push(entry_to_upload);
            remote_file = sftp_client.sftp.create(remote_file_path.as_path()).unwrap();
        }
        else{
            remote_file = sftp_client.sftp.create(remote_file_path.as_ref().unwrap().as_path()).unwrap();
        }
        remote_file.write_all(&file_buffer).unwrap();
        println!("successful upload")
    }
    else{
        println!("not a vaild file");
    }
}


fn sftp_choice(userinput:&Vec<&str>, sftp_client:&mut sftp,remote_cwd:&mut PathBuf)
// could make this return a string it might make it a bit easier to 
//read outputs when in flutter using c bindings.
    {
    if userinput[0] == "exit"{
        sftp_client.alive = false;
    }
    else if userinput[0] =="cd"{
        change_dir(sftp_client, userinput[1],remote_cwd);
    }
    else if userinput[0] == "ls"{
        let file_metadata = list_files(sftp_client,remote_cwd.to_path_buf());
        let output_list:Vec<String> = output_files_string(&file_metadata, sftp_client);
        for index in 0..output_list.len(){
            println!("{}",output_list[index]);
        } 
    }
    else if userinput[0] =="dir"{
        let path = list_cwd_dir(sftp_client,remote_cwd.to_path_buf());
        let path_str:&str = path.to_str().unwrap();
        println!("{}",path_str);
    }
    else if userinput[0] == "sw"{
        let invert:bool =sftp_client.server_selected;
        sftp_client.server_selected = !invert;
    }
    else if userinput[0] == "mkdir"{

    }
    else if userinput[0] == "rmdir"{

    }
    else if userinput[0] == "rm"{

    }
    else if userinput[0] == "rename"{

    }
    else if userinput[0] == "download"{

        if userinput.len() <1{
            println!("provide file to download")
        }
        else if userinput.len()==2{
            //if download file_name no_path
            download(sftp_client, userinput[1], None, remote_cwd)
        }
        else if userinput.len() == 3{
            //if download file_name path 
            println!("{}",userinput.concat());
            let local_path:PathBuf = PathBuf::from(userinput[2]);
            download(sftp_client, userinput[1], Some(local_path), remote_cwd);

        }
    }
    else if userinput[0] == "upload"{
        if userinput.len() <1{
            println!("provide file to upload")
        }
        else if userinput.len()==2{
            //if upload file_name no_path
            upload(sftp_client, userinput[1], None, remote_cwd)
        }
        else if userinput.len() == 3{
            //if upload file_name path 
            println!("{}",userinput.concat());
            let local_path:PathBuf = PathBuf::from(userinput[2]);
            upload(sftp_client, userinput[1], Some(local_path), remote_cwd);

        }
    }
    else if userinput[0] == "move"{

    }
    else{
        println!("[invalid command]");
    }

}


fn main() {
    let password = rpassword::prompt_password("[input password] ").unwrap();
    //let port = rpassword::prompt_password("[input port] ").unwrap();
    let port = String::from("17927");

    println!("{},{}",password,port);


    let mut sftp_client:sftp = sftp_build(String::from("192.168.1.166"),
    port,
    String::from("root"),
        password);
    sftp_main(&mut sftp_client);
}



