# ssh_sftp

using rust for back end with file exploring and ssh sftp file transfer.

using c bindings will use flutter for front end 

# goals 
### goals(main):

* cli file transfer
* sql databse(previous connections and relavitve file path)
* gui implimenation
* export to mobile using flutter front end

### side goals:

* auto complelte on cli 
* files transfer with existing metadata, such as creation data, modify date and what not
* get files to download which have spaces in their names 
* status of upload/download
* setting permisions of mkdir (using mode status, default it is set as 0o755)
* adding ssh terminal intergration
* customisable renaming flags https://docs.rs/ssh2/latest/ssh2/struct.RenameFlags.html
    
## cli commands 
(✅complete, ❌incomplete )
- cd [dir name] - change directories✅
- dir - gives abosulte path ✅
- sw - switches between remote host and local host ✅
- exit - closes connection ✅
- ls - lists files and directories ✅
- mkdir [dir name] [path] - make directory ❌/✅ (path function is not working yet)
- rmdir [dir name] [path]- remove directory ❌/✅(path function is not working yet)
- rm [file name] [path]-remove file ✅
- rename [file/folder name] [new name]- rename file ✅
- download [file/folder name] [local path] - downloads file/folder to local host ❌/✅ (only works for files atm)
- upload [file/folder name] [remote host path]- uploads file/folder to remote host ❌/✅ (only works for files atm)
- move [file/folder name] [new destination path] [new name]- moves a file/folder to a new location ❌/✅ (files only atm)
