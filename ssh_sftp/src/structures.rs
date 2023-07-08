use std::{path::{PathBuf, self, Path}, fs::ReadDir};

use ssh2::{Sftp, Session, FileStat, FileType, Channel};
pub struct sftp{
    pub hostname:String,
    pub port:String,
    pub host_port:String,
    pub username:String,
    pub password:String, //storing password here maybe a potentually bad idea
    pub session: Session,
    pub alive:bool,
    pub server_selected:bool,
    pub sftp:Sftp,
    pub folder_marker:String,
    pub cli_leader:String,

}

pub struct file_metadata{
    //used be able to pass filestat type to other functions
    // can add more detials to this area 

    pub filepath:PathBuf,
    pub size:Option<u64>,
    pub filestat:Option<FileStat>,
    pub file:Option<ReadDir>,
    pub filetype:Option<std::fs::FileType>

}