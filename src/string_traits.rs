use std::io;
use std::io::{Error, ErrorKind, BufReader};

pub trait StringUtils {
    //char position
    fn substring(&self, start: usize, len: usize) -> Self;
    //byte position
    fn rsubstring(&self, start: usize, end: usize) -> Self;
    fn get_pos(&self, character: char) -> io::Result<usize>;
    fn find_result(&self, pat: &str) -> io::Result<usize>;
    fn rfind_result(&self, pat: &str) -> io::Result<usize>;
}

impl StringUtils for String {

    fn rsubstring(&self, start: usize, end: usize) -> Self {
        if self.len() < end || end <= start  {
            return self.clone();
        } 
        let mut new_string = self.get(0..start).unwrap().to_string();
        let end = self.get(end + 1..).unwrap();
        new_string.push_str(end);
        return new_string;
    }

    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }

    fn get_pos(&self, character: char) -> io::Result<usize> {
        match self.chars().position(|c| c == character) {
            Some(x) => Ok(x),
            None => Err(Error::new(ErrorKind::InvalidData, format!("Can not find {} in {}", character, self))),
        }
    }

    fn find_result(&self, pat: &str) -> io::Result<usize> {
        let pos = match self.find(pat) {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::InvalidData, format!("Can not find {} in {}", pat, self))),
        };
        Ok(pos)
    }

    fn rfind_result(&self, pat: &str) -> io::Result<usize> {
        let pos = match self.rfind(pat) {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::InvalidData, format!("Can not find {} in {}", pat, self))),
        };
        Ok(pos)
    }
}