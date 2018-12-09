use org_parser::{OrgEntry, OrgList};
use xml_obj::{XmlDoc, XmlTag};
use std::io;
use std::fs;
use std::fs::{File, DirEntry};
use std::io::{Error, ErrorKind};
use string_traits::StringUtils;
use std::rc::Rc;
use std::cell::RefCell;

static SEPARATE_AUTHOR: &'static [&str] = &["feat", "ft", "presents", "pres", "with", "introduce"];
static SEPARATE_VERSION: &'static [&str] = &["Remix", "Mix", "Dub"];

#[macro_use]
macro_rules! entry {
    ($e:expr) => ((self.org).orgs[$e]);
}

pub struct Manager {
    pub org: OrgList,
    pub xml: XmlDoc,
}

impl Manager {
    pub fn new(org_path: &String, nml_path: &String) -> Option<Manager> {
        let org = match OrgList::parse_file(&org_path) {
            Err(e) => {println!("Error: {}", e); return None;},
            Ok(x) => x,
        };

        let xml = match XmlDoc::parse(&nml_path) {
            Err(e) => {println!("Error: {}", e); return None;},
            Ok(x) => x,
        };

        Some(Manager {
            org: org,
            xml: xml,
        })
    }
}

impl Manager {
    pub fn read_files(&mut self, path: String, max_size: usize) -> io::Result<()> {
        let folders = fs::read_dir(path)?;
        let mut count = 0;
        println!("---------------------------- file length > {} ------------------------", max_size);

        for folder in folders {
            let folder_path: String = folder.unwrap().path().display().to_string();
            //println!("Name: {}", path);
            let files = fs::read_dir(folder_path)?;
            for file in files { 
                let file_name = Manager::get_file_name(file)?;
                let info = self.get_file_information(file_name)?;

                let index: usize = match (self.org).find_entry(&info.name) {
                    Some(x) => x,
                    None => self.get_new_org_entry(&info)?,
                };
 
                //(self.org).orgs[index].name = "".to_string();
                let xml = self.xml.find_file(&info.name);
                
                //self.rename();
                //self.
                //entry!(index).name = "".to_string();

                if info.name.len() > max_size {
                    //println!("Name: {}", file_name);
                    //let entry = Manager::get_new_org_entry(&file_name)?;
                    //&self.org.add(entry);
                    //count += 1;
                }
            }
        }
        println!("Count {}; ", count);
        //entry_list.write_file();
        Ok(())
        //return Ok(());
    }

    pub fn rename(&self, org_idx: usize, xml: Option<Rc<RefCell<XmlTag>>>, file: Result<DirEntry, Error>) -> io::Result<()>{
        let file_name = Manager::get_file_name(file)?;
        let author_pos = get_author_name_pos(&file_name)?;
        let version_pos = get_version_name_pos(&file_name)?;
        //(self.org).orgs[org_idx]
        return Err(Error::new(ErrorKind::NotFound, "Error"));
    }

    pub fn get_file_name(file: Result<DirEntry, Error>) -> io::Result<String> {
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

    fn get_file_information(&mut self, file_name: String) -> io::Result<FileInfo> {
        let mut info = FileInfo::new();
        let author_pos = get_author_name_pos(&file_name)?;
        let version_pos = get_version_name_pos(&file_name)?;
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

        // println!("author: {}", author);
        // println!("author short: {}", shorter_author(&author.to_string()));
        // println!("title: {}", title);
        // println!("title short: {}", shorter_title(&title.to_string()));
        // println!("version: {}", version);
        // println!("version shorter: {}", shorter_version(&version.to_string()));

        let mut shorter_name = shorter_author(&author.to_string());
        shorter_name.push_str(" - ");
        shorter_name.push_str(shorter_title(&title.to_string()).as_str());
        shorter_name.push_str(shorter_version(&version.to_string()).as_str());

        info.short_name = shorter_name;
        info.name = file_name.clone();
        info.author = author.to_string();
        info.title = title.to_string();
        info.version = version.to_string();
        return Ok(info);
    }

    fn get_new_org_entry(&mut self, info: &FileInfo) -> io::Result<usize> {
        let mut entry = OrgEntry::new();
        entry.name = info.short_name.clone();
        entry.author = info.author.clone();
        entry.title = info.title.clone();
        entry.version = info.version.clone();
        //println!("shorter_name: {}", shorter_name);
        Ok(self.org.add(entry))
    }
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
    return remove_first_p(title);
}

fn shorter_version(version: &String) -> String {
    let first_pos = match version.find("(") {
        Some(x) => x,
        None => return version.clone(),
    };

    let last_pos = match version.find(")") {
        Some(x) => x,
        None => return version.clone(),
    };

    if first_pos != 0 || last_pos != version.len() - 1 {
        return remove_first_p(version);
    }

    let len = SEPARATE_VERSION.len();
    for i in 0..len {
        match version.find(SEPARATE_VERSION[i]) {
            Some(x) => return SEPARATE_VERSION[i].to_string(),
            None => continue,
        };
    }
    version.clone()
}

fn remove_first_p(name: &String) -> String {
    let mut open_pos: usize = 0;
    let mut close_pos: usize = 0;
    let mut found_open = false;

    {
        let mut count = 0;
        let mut char_pos: usize = 0;
        let mut chars = name.chars();
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
        return name.rsubstring(open_pos, close_pos);
    }
    name.clone()
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
    //let pos = file_name.get_pos('-')?;
    //let author = file_name.substring(0, pos);
    let mut pos = match file_name.find(" - ") {
        Some(x) => x,
        None => return Err(Error::new(ErrorKind::InvalidData, "Can not find char in String")),
    };
    //pos += 2;
    Ok(pos)
}

pub struct FileInfo {
    pub name: String,
    pub short_name: String,
    pub author: String,
    pub title: String,
    pub version: String,
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            name: "".to_string(),
            short_name: "".to_string(),
            author: "".to_string(),
            title: "".to_string(),
            version: "".to_string(),
        }
    }
}