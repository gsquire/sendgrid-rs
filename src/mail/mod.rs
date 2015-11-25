extern crate rustc_serialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use self::rustc_serialize::json;

#[derive(Debug)]
pub struct Mail {
    pub to: Vec<String>,
    pub to_names: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub from: &'static str,
    pub subject: &'static str,
    pub html: &'static str,
    pub text: &'static str,
    pub from_name: &'static str,
    pub reply_to: &'static str,
    pub date: String,
    pub attachments: HashMap<String, String>,
    pub content: HashMap<String, String>,
    pub headers: HashMap<String, String>
}

impl Mail {
    pub fn new() -> Mail {
        Mail {to: Vec::new(), to_names: Vec::new(), cc: Vec::new(),
            bcc: Vec::new(), from: "", subject: "", html: "", text: "",
            from_name: "", reply_to: "", date: String::new(),
            attachments: HashMap::new(), content: HashMap::new(),
            headers: HashMap::new()}
    }

    pub fn add_cc(&mut self, cc_addr: &'static str) {
        self.cc.push(cc_addr.to_string())
    }

    pub fn add_to(&mut self, to_addr: &'static str) {
        self.to.push(to_addr.to_string())
    }

    pub fn add_from(&mut self, from_addr: &'static str) {
        self.from = from_addr
    }

    pub fn add_subject(&mut self, subject: &'static str) {
        self.subject = subject
    }

    pub fn add_html(&mut self, html: &'static str) {
        self.html = html
    }

    pub fn add_to_name(&mut self, to_name: &'static str) {
        self.to_names.push(to_name.to_string());
    }

    pub fn add_text(&mut self, text: &'static str) {
        self.text = text
    }

    pub fn add_bcc(&mut self, bcc_addr: &'static str) {
        self.bcc.push(bcc_addr.to_string())
    }

    pub fn add_from_name(&mut self, from_name: &'static str) {
        self.from_name = from_name
    }

    pub fn add_reply_to(&mut self, reply_to: &'static str) {
        self.reply_to = reply_to
    }

    pub fn add_date(&mut self, date: String) {
        self.date = date
    }

    pub fn add_attachment(&mut self, path: &str) {
        let file = File::open(path);
        match file {
            Ok(mut f) => {
                let mut data = String::new();
                let read = f.read_to_string(&mut data);
                match read {
                    Ok(_) => { self.attachments.insert(path.to_string(), data); },
                    Err(e) => { panic!("Could not read file: {:?}", e); }
                }
            },
            Err(e) => { panic!("Could not open file: {:?}", e); }
        }
    }

    pub fn add_content(&mut self, id: &str, value: &str) {
        self.content.insert(id.to_string(), value.to_string());
    }

    pub fn add_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_string(), value.to_string());
    }

    pub fn make_header_string(&mut self) -> String {
        let headers = json::encode(&self.headers);
        match headers {
            Ok(h) => h,
            Err(e) => { panic!("Could not encode headers: {:?}", e); }
        }
    }
}
