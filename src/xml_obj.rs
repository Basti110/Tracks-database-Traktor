use std::rc::{Weak, Rc};
use std::cell::RefCell;
extern crate quick_xml;
use xml_obj::quick_xml::Reader;
use xml_obj::quick_xml::events::Event;
use xml_obj::quick_xml::events::BytesStart;
use std::borrow::Cow;
use std::str;
use std::time::{Duration, Instant};

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
        return format!(" {}=\"{}\"", self.key, self.value);
    }
}

pub struct XmlTag {
    pub name: RefCell<String>,
    pub attributes: RefCell<Vec<Attribute>>,
    pub childs: RefCell<Vec<Rc<XmlTag>>>,
    pub count: RefCell<usize>,
    pub parent: RefCell<Weak<XmlTag>>,
    pub text: String,
}

impl XmlTag {
    pub fn new() -> XmlTag {
        XmlTag {
            name: RefCell::new("".to_string()),
            attributes: RefCell::new(Vec::new()),
            childs: RefCell::new(vec![]),
            count: RefCell::new(0),
            parent: RefCell::new(Weak::new()),
            text: "".to_string(),
        }
    }

    pub fn add_empty_child(parent: Rc<XmlTag>) -> Rc<XmlTag> {
        let tag = Rc::new(XmlTag::new());
        *tag.parent.borrow_mut() = Rc::downgrade(&parent);
        parent.childs.borrow_mut().push(Rc::clone(&tag));
        *parent.count.borrow_mut() += 1;
        return tag;
    }

    pub fn to_string(&self) -> String {
        let mut string = "".to_string();
        string.push_str(format!("<{}", self.name.borrow_mut()).as_str());
        for attr in &(*self.attributes.borrow_mut()) {
            string.push_str(attr.to_string().as_str());
        }
        string.push_str(">");
        let mut first = true;
        for tag in &(*self.childs.borrow_mut()) {
            if first {
                string.push_str("\n");
                first = false;
            }
            string.push_str(tag.to_string().as_str());
            string.push_str("\n");
        }
        string.push_str(format!("</{}>", self.name.borrow_mut()).as_str());
        return string;
    }
}

pub struct XmlDoc {
    pub start: RefCell<Rc<XmlTag>>,
}

impl XmlDoc {
    
    pub fn new() -> XmlDoc {
        let vec: Vec<Rc<XmlTag>> = Vec::new();
        XmlDoc {
            start: RefCell::new(Rc::new(XmlTag::new())),
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
        let mut tag = Rc::new(tag); 

        let xml_doc = XmlDoc {
            start: RefCell::new(Rc::clone(&tag)),
        };
        let mut count = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) => { 
                    if count < 0 {
                        continue;
                    }
                    let mut new_tag = XmlTag::add_empty_child(tag);
                    let mut name = BytesStart::owned_name(e.name());
                    let mut name = name.name();
                    let mut name = str::from_utf8(name).unwrap().to_string(); //Handle unwrap
                    *new_tag.name.borrow_mut() = name;
                    //(*Rc::get_mut(&mut (*parent.childs.borrow_mut()).last()).unwrap()).name = name;
                    //new_tag.name = "test".to_string();
                    XmlDoc::add_attributes(e, Rc::clone(&new_tag));
                    tag = Rc::clone(&new_tag);
                    count += 1;
                },           
                Ok(Event::End(e)) => {
                    if count > 0 {
                        count -= 1;
                        let test = (*tag.parent.borrow_mut()).clone();
                        tag = test.upgrade().unwrap();
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
        println!("Time: {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
        return xml_doc;
    }

    pub fn write(&self) -> String {
        return self.start.borrow_mut().to_string();
    }

    fn add_attributes(e: BytesStart, tag: Rc<XmlTag>) {
        let value_vec = e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>();
        let key_vec = e.attributes().map(|a| a.unwrap().key).collect::<Vec<_>>();
        let count = e.attributes().count();
        for i in 0..count {
            let key = String::from_utf8_lossy(key_vec[i].clone());
            let value = decode_utf8_lossy(value_vec[i].clone());
            println!("{}: {}", key, value);
            let key = format!("{}", key);
            let value = format!("{}", value);
            let attribute = Attribute::new(key, value);
            tag.attributes.borrow_mut().push(attribute);
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