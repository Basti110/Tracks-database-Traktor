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

    pub fn find_entry(&mut self, name: String) -> Option<&mut OrgEntry> {
        let mut count = 0;
        for mut entry in &self.orgs {
            if entry.name == name {
                break;
            }
            count += 1;
        }
        return Some(&mut self.orgs[count]);
        None
    }
}