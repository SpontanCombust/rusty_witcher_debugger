use std::{fs::{File, OpenOptions}, sync::mpsc::{Receiver, TryRecvError}, io::{BufReader, Seek, SeekFrom, Read}, time::Duration, path::{Path, PathBuf}};
use directories::UserDirs;

use crate::constants;


/// A function that will keep reading from Witcher 3's script log file until it gets stopped through cancel token or stumbles upon some error.
/// In case of error will return Some with that error message, otherwise will return None.
/// For `printer` parameter you can pass any function or closure that you want to print log text with,
/// the string passed to said printer will consist of one or more lines of text.
pub fn tail_scriptslog<P>( printer: P, refresh_time_millis: u64, cancel_token: Receiver<()>, custom_path: Option<String> ) -> Option<String> 
where P: Fn(&String) -> () {
    if let Some(p) = custom_path {
        tail_scriptslog_loop(Path::new(&p).to_path_buf(), printer, refresh_time_millis, cancel_token)
    } else {
        match scriptslog_file_path() {
            Ok(p) => {
                tail_scriptslog_loop(p, printer, refresh_time_millis, cancel_token)
            }
            Err(e) => {
                Some(e)
            }
        }
    }
}

fn scriptslog_file_path() -> Result<PathBuf, String> {
    let mut docs = None;
    if let Some(ud) = UserDirs::new() {
        if cfg!(windows) {
            if let Some(path) = ud.document_dir() {
                docs = Some(path.to_owned());
            }
        } 
        else if cfg!(unix) {
            if let Some(path) = Some(ud.home_dir()) {
                docs = Some(path.join(constants::LINUX_STEAM_PFX_PATH).to_owned());
            }
        } 
        else {
            unimplemented!();
        }
    }

    if let Some(docs) = docs {
        return Ok( docs.join(Path::new("The Witcher 3").join(constants::SCRIPTSLOG_FILE_NAME)) );
    } else {
        return Err( "Documents directory could not be found.".to_owned() );
    }
}

fn open_scriptslog(path: &PathBuf) -> Result<File, String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true) // so that it can be created if doesn't exist
        .create(true)
        .open(path);

    if let Err(e) = file {
        println!("{:?}", e.kind());
        return Err("File open error: ".to_owned() + &e.to_string());
    } else {
        return Ok(file.unwrap());
    }
}

#[cfg(target_os = "windows")]
fn tail_scriptslog_loop<P>(scriptslog_path: PathBuf, printer: P, refresh_time_millis: u64, cancel_token: Receiver<()>) -> Option<String> 
where P: Fn(&String) -> () {
    match open_scriptslog(&scriptslog_path) {
        Ok(file) => {
            let mut reader = BufReader::new(&file);
            // start from the end of the file
            let mut last_pos = reader.seek( SeekFrom::End(0) ).unwrap();
            let mut text = String::new();

            loop {
                match cancel_token.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(_) => {}
                }

                let filesize = file.metadata().unwrap().len();

                // if the file has been cleared since we've opened it we should go back to its beginning
                if last_pos > filesize {
                    last_pos = reader.seek( SeekFrom::Start(0) ).unwrap();
                }

                text.clear();
                match reader.read_to_string(&mut text) {
                    Ok(size) => {
                        if size > 0 {
                            last_pos += size as u64;
                            printer(&text);
                        }
                    }
                    Err(e) => {
                        return Some("File read error: ".to_owned() + &e.to_string())
                    }
                }

                std::thread::sleep( Duration::from_millis( refresh_time_millis ) );
            }

            None
        }
        Err(e) => {
            Some(e)
        }
    }
}

#[cfg(target_os = "linux")]
fn tail_scriptslog_loop<P>(scriptslog_path: PathBuf, printer: P, refresh_time_millis: u64, cancel_token: Receiver<()>) -> Option<String> 
where P: Fn(&String) -> () {
    let mut last_pos: u64;
    match open_scriptslog(&scriptslog_path) {
        Ok(file) => last_pos = file.metadata().unwrap().len(),
        Err(e) => {
            return Some(e)
        }
    }
    
    let mut text = String::new();
    loop {
        match cancel_token.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(_) => {}
        }
        
        // on linux file system is different in a sense that we have to reopen the file to see the changes made to it
        match open_scriptslog(&scriptslog_path) {
            Ok(file) => {
                let mut reader = BufReader::new(&file);
                let filesize = file.metadata().unwrap().len();

                if last_pos > filesize {
                    last_pos = reader.seek(SeekFrom::Start(0)).unwrap();
                } else {
                    reader.seek(SeekFrom::Start(last_pos)).unwrap();
                }

                text.clear();
                match reader.read_to_string(&mut text) {
                    Ok(size) => {
                        if size > 0 {
                            last_pos += size as u64;
                            printer(&text);
                        }
                    }
                    Err(e) => {
                        return Some("File read error: ".to_owned() + &e.to_string())
                    }
                }
            }
            Err(e) => {
                return Some(e)
            }
        }
        
        std::thread::sleep( Duration::from_millis( refresh_time_millis ) );
    }

    None
}
