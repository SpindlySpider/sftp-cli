# ssh_sftp

using rust for back end with file exploring and ssh sftp file transfer.

using c bindings will use flutter for front end 

# goals 
goals(main):

    cli file transfer (with download/upload progress bar)
    sql databse(previous connections and relavitve file path)
    gui implimenation
    export to mobile
side goals:

    auto complelte on cli 
    
# commands 
(✅completed, ❌underconstruction )
- cd [dir name] - change directories✅
- dir - gives abosulte path ✅
- sw - switches between remote host and local host ✅
- exit - closes connection ✅
- ls - lists files and directories ✅
- mkdir [dir name] - make directory ❌
- rmdir [dir name]- remove directory ❌
- rm [file name]-remove file ❌
- rename [file/folder name] [new name]- rename file ❌
- download [file/folder name] [local path] - downloads file/folder to local host ❌
- upload [file/folder name] [remote host path]- uploads file/folder to remote host ❌
- move [file/folder name] [new path] - moves a file/folder to a new location ❌
