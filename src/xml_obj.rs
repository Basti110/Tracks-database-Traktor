use std::rc::{Weak, Rc};
use std::cell::RefCell;
extern crate quick_xml;
use xml_obj::quick_xml::Reader;
use xml_obj::quick_xml::events::Event;
use xml_obj::quick_xml::events::BytesStart;
use std::borrow::Cow;
use std::str;
use std::time::{Duration, Instant};
use std::fs;
use std::fs::{File};
use std::io::prelude::*;

pub struct Attribute {
    pub key: String,
    pub value: String
}

impl Attribute {
    pub fn new(key: String, value: String) -> Attribute {
        Attribute {
            key: key,
            value: value,
        }
    }

    pub fn to_string(&self) -> String {
        //println!(" {}=\"{}\"", self.key, self.value);
        return format!(" {}=\"{}\"", self.key, self.value);
    }
}

pub struct XmlTag {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub childs: Vec<Rc<RefCell<XmlTag>>>,
    pub count: usize,
    pub parent: Weak<RefCell<XmlTag>>,
    pub text: String,
}

impl XmlTag {
    pub fn new() -> XmlTag {
        XmlTag {
            name: "".to_string(),
            attributes: Vec::new(),
            childs: vec![],
            count: 0,
            parent: Weak::new(),
            text: "".to_string(),
        }
    }

    pub fn add_empty_child(parent: Rc<RefCell<XmlTag>>) -> Rc<RefCell<XmlTag>> {
        let tag = Rc::new(RefCell::new(XmlTag::new()));
        value!(parent).childs.push(Rc::clone(&tag));
        value!(parent).count += 1;
        //let new_ref = Rc::new(parent);
        value!(tag).parent = Rc::downgrade(&parent);
        return tag;
    }

    pub fn to_string_start(&mut self) -> String {
        let mut string = "".to_string();
        for tag in &self.childs {
            string.push_str(value!(tag).to_string().as_str());
        }
        return string;
    }

    pub fn to_string(&mut self) -> String {
        let mut string = "".to_string();
        string.push_str(format!("<{}", self.name).as_str());
        //println!("{}", &(*self.attributes.borrow_mut()));
        for attr in &self.attributes {
            string.push_str(attr.to_string().as_str());
        }
        string.push_str(">");
        let mut first = true;
        for tag in &self.childs {
            if first {
                string.push_str("\n");
                first = false;
            }
            string.push_str(value!(tag).to_string().as_str());
            string.push_str("\n");
        }
        string.push_str(format!("</{}>", self.name).as_str());
        return string;
    }
}

pub struct XmlDoc {
    pub start: Rc<RefCell<XmlTag>>,
}

impl XmlDoc {
    pub fn new() -> XmlDoc {
        XmlDoc {
            start: Rc::new(RefCell::new(XmlTag::new())),
        }
    }

    pub fn parse(&mut self) -> XmlDoc {
        let now = Instant::now();
        let xml = r#"<tag1 att1 = "Moin">
                        <tag2 lol= "haha"><!--Test comment-->💖Test</tag2>
                        <tag2>Test 2</tag2>
                    </tag1>"#;
        let src: &[u8] = include_bytes!("files/collection.nml");
        //let mut reader = Reader::from_str(xml);
        let mut reader = Reader::from_reader(src);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut txt = Vec::new();
        
        let tag = XmlTag::new();//Rc<XmlTag>
        let mut tag = Rc::new(RefCell::new(tag)); 

        let xml_doc = XmlDoc {
            start: Rc::clone(&tag),
        };
        let mut count = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) => { 
                    if count < 0 {
                        continue;
                    }
                    let mut new_tag = XmlTag::add_empty_child(Rc::clone(&tag));
                    let mut name = BytesStart::owned_name(e.name());
                    let mut name = name.name();
                    let mut name = str::from_utf8(name).unwrap().to_string(); //Handle unwrap
                    value!(new_tag).name = name;
                    //(*Rc::get_mut(&mut (*parent.childs.borrow_mut()).last()).unwrap()).name = name;
                    //new_tag.name = "test".to_string();
                    XmlDoc::add_attributes(e, Rc::clone(&new_tag));
                    tag = Rc::clone(&new_tag);
                    count += 1;
                },           
                Ok(Event::End(e)) => {
                    if count > 0 {
                        count -= 1;
                        let strong = value!(tag).parent.upgrade().unwrap();
                        // let strong = match strong {
                        //     Some(x) => x,
                        //     None => return None,
                        // };
                        tag = Rc::clone(&strong);
                        //let test = value!(tag).parent.clone();
                        //tag = test.upgrade().unwrap();
                    }
                },
                Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).expect("Error!")),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        //println!("{}", xml_doc.write());

        

        let dur = now.elapsed();
        println!("Read Time: {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
        
        let mut file = match File::create("foo.txt") {
            Err(e) => { 
                println!("Can not create file");
                return xml_doc;
            },
            Ok(x) => x,
        };

        let now = Instant::now();
        file.write_all(xml_doc.write().as_bytes());
        let dur = now.elapsed();
        println!("Write Time: {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());

        return xml_doc;
    }

    pub fn write(&self) -> String {
        return self.start.borrow_mut().to_string();
    }

    fn add_attributes(e: BytesStart, tag: Rc<RefCell<XmlTag>>) {
        let value_vec = e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>();
        let key_vec = e.attributes().map(|a| a.unwrap().key).collect::<Vec<_>>();
        let count = e.attributes().count();
        for i in 0..count {
            let key = String::from_utf8_lossy(key_vec[i].clone());
            let value = decode_utf8_lossy(value_vec[i].clone());
            //println!("{}: {}", key, value);
            let key = format!("{}", key);
            let value = format!("{}", value);
            let attribute = Attribute::new(key, value);
            value!(tag).attributes.push(attribute);
        }
    }
}

pub fn decode_utf8(input: Cow<[u8]>) -> Result<Cow<str>, str::Utf8Error> {
    match input {
        Cow::Borrowed(bytes) => {
            match str::from_utf8(bytes) {
                Ok(s) => Ok(s.into()),
                Err(e) => Err(e),
            }
        }
        Cow::Owned(bytes) => {
            match String::from_utf8(bytes) {
                Ok(s) => Ok(s.into()),
                Err(e) => Err(e.utf8_error()),
            }
        }
    }
}

pub fn decode_utf8_lossy(input: Cow<[u8]>) -> Cow<str> {
    match input {
        Cow::Borrowed(bytes) => String::from_utf8_lossy(bytes),
        Cow::Owned(bytes) => {
            match String::from_utf8_lossy(&bytes) {
                Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(bytes) }.into(),
                Cow::Owned(s) => s.into(),
            }
        }
    }
}