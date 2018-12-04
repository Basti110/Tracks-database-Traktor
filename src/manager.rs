use org_parser::{OrgEntry, OrgList};
use xml_obj::XmlDoc;
use std::io;

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
    pub fn read_files(&mut self, path: String) -> io::Result<()> {
        return Ok(());
    }
}