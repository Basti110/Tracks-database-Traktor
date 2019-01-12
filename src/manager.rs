use org_parser::{OrgEntry, OrgList};
use xml_obj::{XmlDoc, XmlTag};
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path};
use string_traits::StringUtils;
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;


static SEPARATE_AUTHOR: &'static [&str] = &["feat", "ft", "presents", "pres", "with", "introduce"];
static SEPARATE_AUTHOR_SHORT: &'static [&str] = &["&", "vs"];
static SEPARATE_VERSION: &'static [&str] = &["Remix", "Mix", "Dub"];

pub struct Manager {
    pub org_path: String,
    pub nml_path: String,
    pub org: OrgList,
    pub nml: XmlDoc,
    pub verbose: bool,
    pub max_len: usize,
}

impl Manager {
    pub fn new(org_path: &String, nml_path: &String, max_len: usize) -> Option<Manager> {
        let org = match OrgList::parse_file(&org_path) {
            Err(e) => {println!("Error: {}", e); return None;},
            Ok(x) => x,
        };

        let nml = match XmlDoc::parse(&nml_path) {
            Err(e) => {println!("Error: {}", e); return None;},
            Ok(x) => x,
        };

        Some(Manager {
            org_path: org_path.clone(),
            nml_path: nml_path.clone(),
            org: org,
            nml: nml,
            verbose: true,
            max_len: max_len,
        })
    }
}

impl Manager {
    pub fn debug(&self, out: &str) {
        if self.verbose {
            println!("  {}", &out);
        }
    }

    pub fn write_files(&self) -> io::Result<()> {
        self.org.write_file(&self.org_path)?;
        self.nml.write_file(&self.nml_path)?;
        Ok(())
    }

    pub fn read_files(&mut self, path: &String) -> io::Result<()> {
        let folders = fs::read_dir(path)?;
        let count = 0;
        

        for f in folders {
            let folder = match f {
                Ok(x) => x.path(),
                Err(e) => return Err(e.into()),
            };

            let folder_path: String = folder.display().to_string();
            //println!("Name: {}", path);
            
            let mut folder_name = match folder.file_name() {
                Some(x) => x.to_str().unwrap(),
                None => "",
            };

            let re = Regex::new(r"^\d{4}$").unwrap();
            if !re.is_match(folder_name) {
                folder_name = "";
            }

            //let mut relative_path = folder.path();
            let path: Vec<_> = folder.components().map(|comp| comp.as_os_str().to_str().unwrap().to_string()).collect();

            let files = fs::read_dir(folder_path.clone())?;
            for f in files { 
                let file = match f {
                    Ok(t) => t,
                    Err(e) => return Err(e.into()),
                };
                //println!("--- get Filename");
                let file_name = Manager::get_file_name(&file)?;
                self.debug(&format!("\n\n New File: {}", file_name));
                //println!("--- Filename = \"{}\"", file_name);
                
                let mut info = self.get_file_information(file_name)?;
                
                //let mut path: Vec<String> = vec![];
                //let mut relative_path = file.path();
                //let mut absolute_path = try!(std::env::current_dir());
                //absolute_path.push(relative_path);

                info.path = path.clone();
                info.year = folder_name.to_string();

                self.debug("---------- search org ----------------");
                let index: usize = match (self.org).find_entry(&info.name) {
                    Some(x) => {self.debug("Found entry"); x},
                    None => {self.debug("Create new entry"); self.get_new_org_entry(&info)?},
                };

                self.debug("---------- search nml ----------------");
                let xml = self.nml.find_file(&info.name);

                if xml.is_some() {
                    self.debug("Found entry");
                } 
                else {
                    self.debug("Not found entry");
                }

                self.debug("---------- rename file ---------------");
                self.rename(index, xml, &folder_path, &info)?;
            }
        }
        println!("Count {}; ", count);
        Ok(())
    }

    pub fn rename(&mut self, org_idx: usize, xml: Option<Rc<RefCell<XmlTag>>>, _path: &String, info: &FileInfo) -> io::Result<()>{
        // Rename Path
        self.debug(&info.short_name);
        //let old_path = format!("{}{}{}", &path, "/", info.name);
        //let new_path = format!("{}{}{}", &path, "/", info.short_name);

        //Rename Org
        (self.org).orgs[org_idx].name = info.short_name.clone();


        //Rename NML
        if xml.is_some() {
            let mut key = "".to_string();
            let mut new_key = "".to_string();
            let xml_ref = Rc::clone(&xml.unwrap());
            for t in &value!(xml_ref).childs {
                if value!(t).name == "LOCATION".to_string() {
                    let mut volume = "".to_string();
                    let mut dir = "".to_string();
                    let mut file = "".to_string();
                    let mut new_dir = "".to_string();

                    for mut attr in value!(t).attributes.iter_mut() {
                        if attr.key == "FILE".to_string() {
                            file = attr.value.clone();
                            attr.value = info.short_name.clone();
                        }
                        if attr.key == "VOLUME".to_string() {
                            volume = attr.value.clone();
                        }
                        if attr.key == "DIR".to_string() {
                            dir = attr.value.clone();
                           
                            for p in &info.path {
                                new_dir.push_str("/:");
                                new_dir.push_str(p.as_str());
                            }
                            new_dir.push_str("/:");
                            attr.value = new_dir.clone();
                        }
                    }
                    key = volume.clone() + &dir + &file;
                    new_key = volume + &new_dir + &info.short_name.clone();     
                }
            }
            value!(&self.nml.start).replace_primarykey(&key, &new_key);
        }
        Ok(())
    }

    pub fn get_path(file: &DirEntry) -> io::Result<String> {
        let path = file.path();
        let path = match path.parent() {
            Some(x) => x, 
            None => Path::new("/"), 
        };
        let path = match path.to_str() {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::NotFound, "File path terminates with ..")),
        };

        Ok(path.to_string())
    }

    pub fn get_file_name(file: &DirEntry) -> io::Result<String> {
        let file_name = file.path();
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

    fn get_file_information(&mut self, file_name: String) -> io::Result<FileInfo> {
        let mut info = FileInfo::new();
        let author_pos = get_author_name_pos(&file_name)?;
        let version_pos = get_version_name_pos(&file_name)?;
        let extension = get_extension(&file_name);
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

        let short_author = shorter_author(&author, SEPARATE_AUTHOR);
        let short_title = shorter_title(&title);
        let short_version = shorter_version(&version.to_string());

        // println!("File Name: {}", file_name);
        // println!("author   : {}", short_author.0.to_string());
        // println!("author+  : {}", short_author.1.to_string());
        // println!("title    : {}", short_title.0.to_string());
        // println!("title+   : {}", short_title.1.to_string());
        // println!("version  : {}", short_version.0.to_string());
        // println!("version+ : {}", short_version.1.to_string());
        // println!("Extension: {}", extension);

        let author = short_author.0.to_string();
        let mut short_name = author.clone() + "_ - " + short_title.0 + &short_version.0 + &extension;
        let count = short_name.chars().count();
        if count > self.max_len {
            //println!("-- to long: {}", count);
            let initals = get_initials(&short_title.0.to_string());
            let author = shorter_author(&author, SEPARATE_AUTHOR_SHORT);
            short_name = author.0.to_string() + "_ - " + &initals + &short_version.0 + &extension;
        }

        info.short_name = short_name;
        info.name = file_name.clone();
        info.author = short_author.0.to_string();
        info.author_add = short_author.1.to_string();
        info.title = short_title.0.to_string();
        info.title_add = short_title.1.to_string();
        info.version = short_version.0.to_string();
        info.version_add = short_version.1.to_string();
        return Ok(info);
    }

    fn get_new_org_entry(&mut self, info: &FileInfo) -> io::Result<usize> {
        let mut entry = OrgEntry::new();
        entry.name = info.short_name.clone();
        entry.author = info.author.clone();
        entry.author_add = info.author_add.clone();
        entry.title = info.title.clone();
        entry.title_add = info.title_add.clone();
        entry.version = info.version.clone();
        entry.version_add = info.version_add.clone();
        entry.year = info.year.clone();
        Ok(self.org.add(entry))
    }
}

fn shorter_author<'a>(author: &'a str, seperation: &[&str]) -> (&'a str, &'a str) {
    let len = seperation.len();
    let mut pos: usize = 255;
    for x in 0..len {
        pos = match author.find(seperation[x]) {
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
        return author.split_at(pos);
    }

    (author.clone(), "")
}

fn shorter_title<'a>(title: &'a str) -> (&'a str, &'a str) {
    let first_pos = match title.find("(") {
        Some(x) => x,
        None => return (title.clone(), ""),
    };
    title.split_at(first_pos)
}

fn shorter_version(version: &String) -> (String, String) {
    let first_pos = match version.find("(") {
        Some(x) => x,
        None => return (version.clone(), "".to_string()),
    };

    let last_pos = match version.find(")") {
        Some(x) => x,
        None => return (version.clone(), "".to_string()),
    };

    if first_pos != 0 || last_pos != version.len() - 1 {
        return remove_first_p(version);
    }

    let len = SEPARATE_VERSION.len();
    for i in 0..len {
        match version.find(SEPARATE_VERSION[i]) {
            Some(_x) => {
                let s = format!("(Special_{})", SEPARATE_VERSION[i]);
                return (s, version.clone());
            },
            None => continue,
        };
    }
    (version.clone(), "".to_string())
}

fn remove_first_p(name: &String) -> (String, String) {
    let mut open_pos: usize = 0;
    let mut close_pos: usize = 0;
    let mut found_open = false;

    {
        let mut count = 0;
        let mut char_pos: usize = 0;
        let mut chars = name.chars();
        match chars.next() {
                Some(_x) => (),
                None => return (name.clone(), "".to_string()),
        };
        loop {
            let c = match chars.next() {
                Some(x) => x,
                None => break,
            };

            if c == '(' {
                if found_open == false {
                    open_pos = char_pos;
                    found_open = true;
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
    if open_pos < close_pos && found_open == true {
        let a = name.to_string().substring(open_pos + 1, (close_pos - open_pos) + 1); 
        let b = name.to_string().rsubstring(open_pos, close_pos + 1);
        return (b, a);
    }
    (name.clone(), "".to_string())
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

fn get_author_name_pos(file_name: &String) -> io::Result<usize> {
    let pos = match file_name.find(" - ") {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not find char in String")),
    };
    Ok(pos)
}

fn get_extension(file_name: &String) -> String {
    let tuple = match file_name.rfind('.') {
        Some(x) => file_name.split_at(x),
        None => ("", ""),
    };
    return tuple.1.to_string();
}

fn get_initials(text: &String) -> String {
    let mut initials = "".to_string();
    let split: Vec<&str> = text.split(' ').collect();
    for s in split {
        match s.chars().next() {
            Some(x) => initials = initials + &x.to_uppercase().to_string(),
            None => (),
        }

    }

    return initials;
}

pub struct FileInfo {
    pub name: String,
    pub short_name: String,
    pub author: String,
    pub author_add: String,
    pub title: String,
    pub title_add: String,
    pub version: String,
    pub version_add: String,
    pub extension: String,
    pub year: String,
    pub path: Vec<String>, 
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            name: "".to_string(),
            short_name: "".to_string(),
            author: "".to_string(),
            author_add: "".to_string(),
            title: "".to_string(),
            title_add: "".to_string(),
            version: "".to_string(),
            version_add: "".to_string(),
            extension: "".to_string(),
            year: "".to_string(),
            path: vec![],
        }
    }
}