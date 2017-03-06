use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result;
use std::option::Option;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Read;
use std::io::Write;

struct Upload{
    before_open_callback: fn(),
    before_close_callback: fn(),
}

struct SetMeta{
    
}

enum ExtraInfo {
    UPLOAD(),
    SETMETA(),
}

pub struct FileContext {
    file: Option<File>,
    option: OpenOptions,
    filename: String,
    offset: u64,
    end: u64,
}

impl FileContext {

    pub   fn new(file_name: &str) -> FileContext {
        FileContext {
            file: None,
            option: OpenOptions::new(),
            filename: file_name.to_string(),
            offset: 0,
            end: 0,
        }
    }

    pub    fn set_option(&mut self, option: OpenOptions) {
        self.option = option;
    }

    pub fn open_file(&mut self) -> Result<()> {
        let mut file = match self.file.take() {
            Some(f) => f,
            None => {
                let mut file = try!(self.option.open(&self.filename));
                file
            }
        };
        
        try!(file.seek(SeekFrom::Start(self.offset)));

        self.file = Some(file);
        Ok(())
    }

    pub fn read_file(&mut self, buf: &mut [u8]) -> Result<usize> {
        try!(self.open_file());
        let mut remain_bytes = self.end - self.offset;
        let mut size: usize = 0;
        let mut capacity_bytes = buf.len(); 
        {
            let file = self.file.as_mut().unwrap();
            let read_bytes = if capacity_bytes < remain_bytes as usize {
            capacity_bytes
            } else {
                remain_bytes as usize 
            };
            
            size = try!(file.read(buf));
            self.offset += size as u64;
        }
        if self.offset == self.end {
            self.file.take();
        }
        Ok(size)
    }

    pub fn write_file(&mut self, buf: &[u8]) -> Result<usize> {
        if self.file.is_none() {
            try!(self.open_file())
        }

        let mut size: usize = 0;
        {
            let file = self.file.as_mut().unwrap();
            size = try!(file.write(buf));
            self.offset += size as u64;
        }
        if self.offset < self.end {
        } else {
            self.file.take();
        }

        Ok(size)
    }

    pub fn truncate_file(&mut self) -> Result<()> {
        if self.file.is_none() {
            try!(self.open_file());
        }
        {
            let file = self.file.as_mut().unwrap();
            try!(file.set_len(self.offset));
        }
        self.file.take();
        Ok(())
    }
}

#[test]
fn write_test() {

    let mut o = OpenOptions::new();
    o.read(true).write(true).create(true);
    let mut file = FileContext::new("test.txt");
    file.set_option(o);
    let res = file.open_file();
    assert_eq!(res.is_ok(), true);
    let res = file.write_file("test".as_bytes());
    assert_eq!(res.is_ok(), true);
    assert_eq!(4,4);
}

#[test]
fn read_test() {

    let mut o = OpenOptions::new();
    o.read(true).write(true).create(true);
    let mut file = FileContext::new("test.txt");
    file.end = 10;
    file.set_option(o);
    let res = file.open_file();
    assert_eq!(res.is_ok(), true);
    let mut vec = vec![0; 4];
    let res = file.read_file(&mut vec);
    assert_eq!(res.is_ok(), true);
    let mut txt = String::from_utf8(vec).unwrap();
    assert_eq!(txt, "test");
    assert_eq!(4,4);
}
