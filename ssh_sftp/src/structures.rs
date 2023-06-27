use std::{path::PathBuf, fs::ReadDir};

use ssh2::{Sftp, Session, FileStat, FileType};
pub struct sftp{
    pub hostname:String,
    pub port:String,
    pub host_port:String,
    pub username:String,
    pub password:String, //storing password here maybe a potentually bad idea
    pub session: Session,
    pub alive:bool,
    pub server_selected:bool,
    pub sftp:Sftp
}

pub struct file_metadata{
    //used be able to pass filestat type to other functions
    // can add more detials to this area 

    filepath:PathBuf,
    size:Option<u64>,
    filestat:Option<FileStat>,
    file:Option<ReadDir>,
    filetype:Option<FileType>

}