#[macro_use]
pub mod utils;
pub mod org_parser;
pub mod string_traits;
pub mod xml_obj;
pub mod manager;

extern crate regex;
extern crate clap;
use clap::{Arg, App};

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
use regex::Regex;


//Const Settings
//const GENERATE_DATA: bool = false;
const TRACK_LIST_PATH: &str = "src/files/tracks-sample-mini.txt";
const FILE_DIR: &str = "files-mini/";
const MAX_FILE_NAME_LEN: usize = 55;

fn main() -> io::Result<()> {

    println!("|--- Delete files");
    fs::remove_dir_all(FILE_DIR)?;
    fs::remove_file("src/files/tracks-mini-2.org")?;
    fs::remove_file("src/files/collection-mini-2.nml")?;
    fs::copy("src/files/tracks-mini.org", "src/files/tracks-mini-2.org")?;
    fs::copy("src/files/collection-mini.nml", "src/files/collection-mini-2.nml")?;
    println!("---| Delete files");

    let matches = App::new("Tracks-database-Traktor")
                            .version("1.0")
                            .author("Sebastian Preuß <sebastian.preuss@rwth-aachen.com>")
                            .about("Rename tracks and sync with org and NML")
                            .arg(Arg::with_name("verbose")
                                .short("v")
                                .help("Sets verbosity"))
                            .arg(Arg::with_name("generate")
                                .short("g")
                                .help("generate test files"))     
                            .arg(Arg::with_name("len")
                                .short("l")
                                .help(&format!("Max name length. Default = {}", MAX_FILE_NAME_LEN))
                                .takes_value(true))                         
                            .get_matches();

    let max_len = match matches.value_of("length") {
        None => MAX_FILE_NAME_LEN,
        Some(x) => match x.parse::<usize>() {
            Err(_e) => MAX_FILE_NAME_LEN,
            Ok(x) => x,
        },
    };

    if true { //matches.is_present("generate") {
        if !Path::new(FILE_DIR).exists() {
            fs::create_dir(FILE_DIR)?;
        }
        println!("|--- Write files from tracks-sample.txt");
        let now = Instant::now();
        write_files_from_list()?;
        let dur = now.elapsed();
        println!("---| write files from tracks-sample.txt in {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
    }
    
    println!("|--- Move mp3 and m4a to the right year");
    let now = Instant::now();
    sort_mp3_m4a(FILE_DIR);
    let dur = now.elapsed();
    println!("---| move mp3 and m4a to the right year in {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
    
    println!("|--- start Manager");
    let now = Instant::now();
    let mut manager = match Manager::new(&"src/files/tracks-mini-2.org".to_string(), &"src/files/collection-mini-2.nml".to_string(), max_len) {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not Start manager")),
    };
    let dur = now.elapsed();
    println!("---| start Manager in {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());

    println!("|--- Read Files and Update");
    let now = Instant::now();
    manager.read_files(&FILE_DIR.to_string())?;
    let dur = now.elapsed();
    println!("---| Read Files and Update in {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());

    println!("|--- Write Files");
    let now = Instant::now();
    manager.write_files()?;
    let dur = now.elapsed();
    println!("---| Write Files in {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
    Ok(())
}

fn write_files_from_list() -> io::Result<()> {
    let f = File::open(TRACK_LIST_PATH)?;
    let reader = BufReader::new(f);

    let begin = String::from(FILE_DIR);
    for buffer in reader.lines() {
        let line = buffer.unwrap().clone();
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

fn move_file_to_year(path_src: &String, path_dst: &str, ext: &str) -> io::Result<()> {
    let files = fs::read_dir(path_src)?;
    let ext_len = ext.len();
    let exp = format!("{}{}{}", r"(?P<year>\d{4})(?P<ext>.", ext, r")$");
    let re = Regex::new(&exp).unwrap();

    for file in files {
        let file_name = get_file_name(file)?;
        let len = file_name.len();

        let capture = match re.captures(&file_name) {
            None => continue,
            Some(x) => x,
        };

        let year = &capture["year"];
        let extension = &capture["ext"];

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

        let new_path = format!("{}{}{}", new_path, new_name, extension);
        fs::rename(format!("{}{}",path_src, file_name), new_path)?;
    }
    Ok(())
}

fn sort_mp3_m4a(path: &str) {
    let mp3_path = format!("{}{}", path, "1mp3/");
    let m4a_path = format!("{}{}", path, "1m4a/");
    
    match move_file_to_year(&mp3_path, FILE_DIR, "mp3") {
        Err(e) => println!("{}", e),
        Ok(_x) => (),
    };

    match move_file_to_year(&m4a_path, FILE_DIR, "m4a") {
        Err(e) => println!("{}", e),
        Ok(_x) => (),
    };
}