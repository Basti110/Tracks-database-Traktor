use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, BufReader};
use std::fs;
use std::fs::{File, DirEntry};
use std::path::Path;
mod org_parser;
const GENERATE_DATA: bool = false;
const TRACK_LIST_PATH: &str = "src/files/tracks-sample.txt";
const FILE_DIR: &str = "files/";
const MAX_FILE_NAME_LEN: usize = 80;
static SEPARATE_AUTHOR: &'static [&str] = &["feat", "ft", "presents", "pres", "with", "introduce"];

fn main() -> io::Result<()> {
    if GENERATE_DATA {
        write_files_from_list()?;
        sort_mp3_m4a("files/")?;
        check_files(FILE_DIR)?;
    }
    //let test = "Hello World".to_string();
    //let lol = test.substring(2, 5);
    //println!("Name: {}", test);
    check_files(FILE_DIR)?;
    //check_files(FILE_DIR)?;
    // let mut entry = org_parser::OrgEntry::new(); 
    // entry.name = "test".to_string();
    // entry.author = "autor 1".to_string();
    // let mut entry_list = org_parser::OrgList::new();  
    // entry_list.add(entry);
    // {
    //     let entry2 = entry_list.find_entry("test".to_string()).unwrap();
    //     entry2.author = "lol".to_string();
    // }
    // let entry3 = entry_list.find_entry("test".to_string());
    // println!("{}", entry3.unwrap().author);

    //let entry = org_parser::OrgList::parse_file(&"src/files/tracks.org".to_string())?;
    Ok(())
}

fn write_files_from_list() -> io::Result<()> {
    let f = File::open(TRACK_LIST_PATH)?;
    let reader = BufReader::new(f);
    //println!("{}", "Generate files");
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
                println!("Name: {}", file_name);
                get_name_parts(&file_name)?;
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

fn get_name_parts(file_name: &String) -> io::Result<()> {
    let author_pos = get_author_name_pos(file_name)?;
    let version_pos = get_version_name_pos(file_name)?;
    let author;
    let title;
    let mut version;
    let mut offset = " - ".len();
    unsafe {
        author = file_name.get_unchecked(0..author_pos);
        title = file_name.get_unchecked(author_pos + offset..version_pos);
        version = file_name.get_unchecked(version_pos..file_name.len());
    }
    let extension_pos = version.to_string().rfind_result(").")?;
    offset = ")".len();
    unsafe {
        version = version.get_unchecked(0..extension_pos + offset);
    }
    println!("author: {}", author);
    println!("author short: {}", shorter_author(&author.to_string()));
    println!("----title: {}", title);
    println!("----title short: {}", shorter_title(&title.to_string()));
    println!("version: {}", version);

    Ok(())
}

fn get_author_name_pos(file_name: &String) -> io::Result<usize> {
    //let pos = file_name.get_pos('-')?;
    //let author = file_name.substring(0, pos);
    let pos = match file_name.find(" - ") {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not find char in String")),
    };
    Ok(pos)
}

fn shorter_author(author: &String) -> String {
    let len = SEPARATE_AUTHOR.len();
    let mut pos: usize = 255;
    for x in 0..len {
        pos = match author.find(SEPARATE_AUTHOR[x]) {
            Some(x) => if x < pos {
                x
            }
            else {
                pos
            }
            None => pos,
        };
    } 
    if pos != 255 {
        match author.get(0..pos) {
            Some(x) => {
                let mut y = x.trim().to_string();
                y.push_str("_");
                return y 
            },
            None => return author.clone(),
        };
    }
    author.clone()
}

fn shorter_title(title: &String) -> String {
    match title.find("(") {
        Some(x) => x,
        None => return title.clone(),
    };
    let mut open_pos: usize = 0;
    let mut close_pos: usize = 0;
    {
        let mut count = 0;
        let mut char_pos: usize = 0;
        let mut chars = title.chars();
        
        loop {
            let c = match chars.next() {
                Some(x) => x,
                None => break,
            };

            if c == '(' {
                if count == 0 {
                    open_pos = char_pos;
                }
                count += 1;
            }
            else if c == ')' {
                close_pos = char_pos;
                count -= 1;
            }
            if count == 0 && open_pos != 0 {
                break;
            }
            char_pos += c.len_utf8();
        }
    }
    if open_pos != 0 && open_pos < close_pos {
        let mut new_title = title.get(0..open_pos).unwrap().to_string();
        let end = title.get(close_pos + 1..).unwrap();
        new_title.push_str(end);
        return new_title;
    }
    title.clone()
}

fn get_version_name_pos(file_name: &String) -> io::Result<usize> {
    let mut pos = file_name.rfind_result(").")?;
    let mut slice;
    let mut count = 1;
    loop {
        unsafe {
            slice = file_name.get_unchecked(0..pos);
        } 
        let pos1 = match slice.rfind(")") {
            Some(x) => x,
            None => 0,
        };
        let pos2 = slice.to_string().rfind_result("(")?;
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
    Ok(pos)
}

fn move_file_to_year(path_src: &String, path_dst: &str, ext: &str) -> io::Result<()> {
    let files = fs::read_dir(path_src)?;
    let ext_len = "ext".len();
    for file in files {
        let file_name = get_file_name(file)?;
        let len = file_name.len();

        let extension_with_year = match file_name.get(len - (ext_len + 6)..len) {
            Some(x) => x,
            None => continue,
        };

        if extension_with_year.chars().next().unwrap() != '.' {
            continue;
        }

        let year = match extension_with_year.get(1..5) {
            Some(x) => x,
            None => continue,
        };

       match year.parse::<i32>(){
            Ok(x) => x,
            Err(e) => continue,
        };
        
        let new_path = format!("{}{}{}", path_dst, year, "/");

        if !Path::new(&new_path).exists() {
            fs::create_dir(&new_path)?;
        }
        
        let new_name = match file_name.get(0..len - (ext_len + 6)) {
            Some(x) => x,
            None => continue,
        };

        let new_path = format!("{}{}{}", new_path, new_name, ext);
        fs::rename(format!("{}{}",path_src, file_name), new_path)?;
    }
    Ok(())
}

fn sort_mp3_m4a(path: &str) -> io::Result<()> {
    let mp3_path = format!("{}{}", path, "1mp3/");
    let m4a_path = format!("{}{}", path, "1m4a/");
    move_file_to_year(&mp3_path, "files/", "mp3")?;
    move_file_to_year(&m4a_path, "files/", "m4a")?;
    Ok(())
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