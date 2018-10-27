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
            name: "".to_string(),
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
        let mut reader = BufReader::new(f);
        let mut orgList = OrgList::new();

        //let test = reader.nex();
        let mut line = "".to_string();
        //let mut len: usize = 0;
        
        loop {
            line = "".to_string();
            let len = reader.read_line(&mut line)?;
            let mut entry = OrgEntry::new();
            if len == 0 {
                break;
            }   
            line.pop();
            if line.match_first_chars( "**".to_string()) {
                line.drain(..3);
                entry.name = line.clone();
                //println!("Name: {}", line);
                loop {
                    line = "".to_string();
                    let len2 = reader.read_line(&mut line)?;
                    line.pop();
                    if len2 == 0 {
                        break;
                    }
                    if line.match_first_chars( ":END:".to_string()) {
                        //println!("End");
                        break;
                    }
                    if line.match_first_chars( ":PROPERTIES:".to_string()) {
                        //println!("Release");
                    } 
                    else if line.match_first_chars( ":Author:".to_string()) {
                        //println!("Author");
                    } 
                    else if line.match_first_chars( ":Author+:".to_string()) {
                        //println!("Author");
                    } 
                    else if line.match_first_chars( ":Title:".to_string()) {
                        //println!("Title");
                    }
                    else if line.match_first_chars( ":Title+:".to_string()) {
                        //println!("Title");
                    }
                    else if line.match_first_chars( ":Version:".to_string()) {
                        //println!("Version");
                    }
                    else if line.match_first_chars( ":Year:".to_string()) {
                        //println!("Year");
                    }
                    else if line.match_first_chars( ":Release:".to_string()) {
                        //println!("Release");
                    } 
                    else if line.match_first_chars( ":Notes:".to_string()) {
                        //println!("Release");
                    } 
                    else { 
                        println!("{}", line);
                    }
                }
            }
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

