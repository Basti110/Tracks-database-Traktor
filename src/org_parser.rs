use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub struct OrgEntry {
    pub name: String,
    pub author: String,
    author_add: String,
    title: String,
    title_add: String,
    version: String,
    year: String,
    release: String,
    notes: String
}

impl OrgEntry {
    pub fn new() -> OrgEntry {
        OrgEntry {
            name: "".to_string(),
            author: "".to_string(),
            author_add: "".to_string(),
            title: "".to_string(),
            title_add: "".to_string(),
            version: "".to_string(),
            year: "".to_string(),
            release: "".to_string(), 
            notes: "".to_string(), 
        }
    }

    pub fn to_string(self) -> String {
        let mut string = "** ".to_string();
        string.push_str(&self.name);
        string.push_str(&"\n");
        string.push_str(&":PROPERTIES:\n");
        if self.author != "".to_string() {
            string.push_str(&":Author:    ");
            string.push_str(&self.author);
            string.push_str(&"\n");
        }
        if self.author_add != "".to_string() {
            string.push_str(&":Author+:   ");
            string.push_str(&self.author_add);
            string.push_str(&"\n");
        }
        if self.title != "".to_string() {
            string.push_str(&":Title:     ");
            string.push_str(&self.title);
            string.push_str(&"\n");
        }
        if self.title_add != "".to_string() {
            string.push_str(&":Title+:    ");
            string.push_str(&self.title_add);
            string.push_str(&"\n");
        }
        if self.version != "".to_string() {
            string.push_str(&":Version:   ");
            string.push_str(&self.version);
            string.push_str(&"\n");
        }
        if self.year != "".to_string() {
            string.push_str(&":Year:      ");
            string.push_str(&self.year);
            string.push_str(&"\n");
        }
        if self.release != "".to_string() {
            string.push_str(&":Release:   ");
            string.push_str(&self.release);
            string.push_str(&"\n");
        }
        if self.notes != "".to_string() {
            string.push_str(&":Notes:     git ");
            string.push_str(&self.notes);
            string.push_str(&"\n");
        }
        string.push_str(&":END:");
        string
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
                        //println!("PROPERTIES");
                    } 
                    else if line.match_first_chars( ":Author:".to_string()) {
                        line.drain(..8);
                        entry.author = line.trim().to_string();
                        //println!("Author");
                    } 
                    else if line.match_first_chars( ":Author+:".to_string()) {
                        line.drain(..9);
                        entry.author_add = line.trim().to_string();
                        //println!("Author+");
                    } 
                    else if line.match_first_chars( ":Title:".to_string()) {
                        line.drain(..7);
                        entry.title = line.trim().to_string();
                        //println!("Title");
                    }
                    else if line.match_first_chars( ":Title+:".to_string()) {
                        line.drain(..8);
                        entry.title_add = line.trim().to_string();
                        //println!("Title+");
                    }
                    else if line.match_first_chars( ":Version:".to_string()) {
                        line.drain(..9);
                        entry.version = line.trim().to_string();
                        //println!("Version");
                    }
                    else if line.match_first_chars( ":Year:".to_string()) {
                        line.drain(..6);
                        entry.year = line.trim().to_string();
                        //println!("Year");
                    }
                    else if line.match_first_chars( ":Release:".to_string()) {
                        line.drain(..9);
                        entry.release = line.trim().to_string();
                        //println!("Release");
                    } 
                    else if line.match_first_chars( ":Notes:".to_string()) {
                        line.drain(..7);
                        entry.notes = line.trim().to_string();
                        //println!("Notes");
                    } 
                    else { 
                        println!("{}", line);
                    }
                }
                println!("{}", entry.to_string());
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

