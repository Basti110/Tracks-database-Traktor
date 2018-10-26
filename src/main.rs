use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, BufReader};
use std::fs;
use std::fs::{File, DirEntry};
use std::path::Path;

const GENERATE_DATA: bool = false;
const TRACK_LIST_PATH: &str = "src/files/tracks-sample.txt";
const FILE_DIR: &str = "files/";
const MAX_FILE_NAME_LEN: usize = 84;

fn main() -> io::Result<()> {
    if GENERATE_DATA {
        write_files_from_list()?;
    }
    check_files(FILE_DIR)?;
    Ok(())
}

fn write_files_from_list() -> io::Result<()> {
    let f = File::open(TRACK_LIST_PATH)?;
    let reader = BufReader::new(f);
    println!("{}", "Generate files");
    let begin = String::from("files/");
    for buffer in reader.lines() {
        let line = buffer.unwrap().clone();//buffer.clone();
        let mut dirs: Vec<&str> = line.split("/").collect();
        let mut path = begin.clone();
        
        for i in 0..(dirs.len() - 1) {
            path.push_str(dirs[i].clone());
            if !Path::new(&path).exists() {
                fs::create_dir(&path)?;
            }
            path.push('/');
        }
        path.push_str(dirs[dirs.len() - 1].clone());
        path = path.replace("\"", "");
        path = path.replace("?", "");
        if !Path::new(&path).exists() {
            let mut file = File::create(&path)?;
            file.write_all(b"Test File!")?;
        }
    }
    Ok(())
}

fn check_files(path: &str) -> io::Result<()> {
    let folders = fs::read_dir(path)?;
    let mut count = 0;
    for folder in folders {
        let folder_path: String = folder.unwrap().path().display().to_string();
        //println!("Name: {}", path);
        let files = fs::read_dir(folder_path)?;
        for file in files {
            let file_name = get_file_name(file)?;
            if file_name.len() > MAX_FILE_NAME_LEN {
                let author = get_author_name(&file_name)?;
                let version = get_version_name(&file_name)?;
                println!("Name: {}", author);
                println!("Name: {}", file_name);
                println!("Name: {}", version);
                count += 1;
            }
        }
    }
    println!("Count {}; ", count);
    Ok(())
}

fn get_file_name(file: Result<DirEntry, Error>) -> io::Result<String> {
    let file_name = file;
    let file_name = match file_name {
        Ok(t)  => t.path(),
        Err(e) => return Err(e.into()),
    };
    let file_name = match file_name.file_name() {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::NotFound, "File path terminates with ..")),
    };
    let file_name = match file_name.to_str() {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not convert path to UTF-8 string")),
    };
    Ok(file_name.to_string())
}

fn get_author_name(file_name: &String) -> io::Result<String> {
    //let pos = file_name.get_pos('-')?;
    //let author = file_name.substring(0, pos);
    let pos = match file_name.find(" - ") {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not find char in String")),
    };
    let slice;
    unsafe {
        slice = file_name.get_unchecked(0..pos);
    }
    Ok(slice.to_string())
}

fn get_version_name(file_name: &String) -> io::Result<String> {
    let mut pos = file_name.rfind_result(").")?;
    let mut slice;
    let mut count = 1;
    loop {
        unsafe {
            slice = file_name.get_unchecked(0..pos);
        } 
        println!("Name: {}", slice);
        let pos1 = match slice.rfind(")") {
            Some(x) => x,
            None => 0,
        };
        let pos2 = slice.to_string().rfind_result("(")?;
        println!("pos1: {} pos2: {}", pos1, pos2);
        if pos1 > pos2 {
            count += 1;
            pos = pos1
        }
        else {
            count -= 1;
            pos = pos2;
        }
        if count == 0 {
            break;
        }
    }
    unsafe {
        slice = file_name.get_unchecked(0..pos);
    } 
    Ok(slice.to_string())
}

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
    fn get_pos(&self, character: char) -> io::Result<usize>;
    fn find_result(&self, pat: &str) -> io::Result<usize>;
    fn rfind_result(&self, pat: &str) -> io::Result<usize>;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }

    fn get_pos(&self, character: char) -> io::Result<usize> {
        match self.chars().position(|c| c == character) {
            Some(x) => Ok(x),
            None => Err(Error::new(ErrorKind::InvalidData, format!("Can not find {} in {}", character, self))),
        }
    }

    fn find_result(&self, pat: &str) -> io::Result<usize> {
        let pos = match self.find(pat) {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::InvalidData, format!("Can not find {} in {}", pat, self))),
        };
        Ok(pos)
    }

    fn rfind_result(&self, pat: &str) -> io::Result<usize> {
        let pos = match self.rfind(pat) {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::InvalidData, format!("Can not find {} in {}", pat, self))),
        };
        Ok(pos)
    }
}