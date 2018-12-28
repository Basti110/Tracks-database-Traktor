#[macro_use]
pub mod utils;
pub mod org_parser;
pub mod string_traits;
pub mod xml_obj;
pub mod manager;
extern crate regex;
use org_parser::{OrgEntry, OrgList};
use string_traits::StringUtils;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, BufReader};
use std::fs;
use std::fs::{File, DirEntry};
use std::path::Path;
use std::str;
use xml_obj::XmlDoc; 
use std::time::{Duration, Instant};
use manager::Manager;


//Const Settings
const GENERATE_DATA: bool = false;
const TRACK_LIST_PATH: &str = "src/files/tracks-sample-mini.txt";
const FILE_DIR: &str = "files-mini/";
const MAX_FILE_NAME_LEN: usize = 80;
static SEPARATE_AUTHOR: &'static [&str] = &["feat", "ft", "presents", "pres", "with", "introduce"];
static SEPARATE_VERSION: &'static [&str] = &["Remix", "Mix", "Dub"];


fn main() -> io::Result<()> {
    if GENERATE_DATA {
        if !Path::new(FILE_DIR).exists() {
            fs::create_dir(FILE_DIR)?;
        }
        println!("|--- Write files from tracks-sample.txt");
        write_files_from_list()?;
        println!("---| write files from tracks-sample.txt");
        println!("|--- Move mp3 and m4a to the right year");
        match sort_mp3_m4a(FILE_DIR) {
            Err(e) => println!("{}", e),
            Ok(_x) => (),
        };
        println!("---| move mp3 and m4a to the right year");
        //println!("Rename files and check length");
        //check_files(FILE_DIR)?; //rename_files 
    }
    // //let mut xml = XmlDoc::new();
    // //let mut org = OrgList::parse_file(&"files/collection.nml".to_string());
    // //let mut xml = XmlDoc::parse(&"test".to_string());
    // println!("second Round");
    // let now = Instant::now();
    // //let output = xml.find_file(&"MANDY vs Booka Shade - Body Language (Tocadisco Remix).wav".to_string());
    // //println!("output: {}", output);
    // let dur = now.elapsed();
    // println!("Find Time: {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
    //println!("------- Start: New names ---------------");
    println!("|--- start Manager");
    let mut manager = match Manager::new(&"src/files/tracks-mini.org".to_string(), &"src/files/collection.nml".to_string()) {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not Start manager")),
    };
    println!("---| start Manager");
    println!("|--- Read Files and Update");
    return manager.read_files(&FILE_DIR.to_string(), 80);
    println!("---| Read Files and Update");
}

fn write_files_from_list() -> io::Result<()> {
    let f = File::open(TRACK_LIST_PATH)?;
    let reader = BufReader::new(f);
    //println!("{}", "Generate files");
    let begin = String::from(FILE_DIR);
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

fn get_author_name_pos(file_name: &String) -> io::Result<usize> {
    //let pos = file_name.get_pos('-')?;
    //let author = file_name.substring(0, pos);
    let mut pos = match file_name.find(" - ") {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not find char in String")),
    };
    //pos += 2;
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