use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub struct OrgEntry {
    pub name: String,
    pub author: String,
    author_add: String,
    title: String,
    version: String,
    year: String,
    release: String,
}

impl OrgEntry {
    pub fn new() -> OrgEntry {
        OrgEntry {
            name: String::from(""),
            author: "".to_string(),
            author_add: "".to_string(),
            title: "".to_string(),
            version: "".to_string(),
            year: "".to_string(),
            release: "".to_string(), 
        }
    }
}

pub struct OrgList {
    orgs: Vec<OrgEntry>,
}

impl OrgList {
    pub fn new() -> OrgList {
        OrgList {
            orgs: Vec::new(),
        }
    }
    pub fn add(&mut self, value: OrgEntry) {
        self.orgs.push(value);
    }

    pub fn parse_file(path: &String) -> io::Result<OrgList> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let mut orgList = OrgList::new();

        for buffer in reader.lines() { 
            let mut line = buffer.unwrap();

            if line.match_first_chars( "**".to_string()) {
                line.drain(..3);
                println!("Name: {}", line);
            }
            // println!("{}{}", char1, char2);
        }

        Ok(orgList)
    }

    pub fn find_entry(&mut self, name: String) -> Option<&mut OrgEntry> {
        let mut count = 0;
        let mut found = false;
        for mut entry in &self.orgs {
            if entry.name == name {
                found = true;
                break;
            }
            count += 1;
        }
        if found {
            return Some(&mut self.orgs[count]);
        }
        None
    }
}

trait StringParse {
    fn match_first_chars(&self, pat: String) -> bool;
}

impl StringParse for String {
    fn match_first_chars(&self, pat: String) -> bool {
        let mut string_chars = self.chars();
        let mut pat_chars = pat.chars();

        loop {
            let char1 = string_chars.next();
            let char2 = pat_chars.next();

            if char2 == None {
                return true;
            }   

            if char1 == None || char1 != char2 {
                return false;
            } 
        }
    }
}

