use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use rustc_serialize::json;

#[derive(Debug)]
/// This is a representation of a valid SendGrid message. It has support for
/// all of the fields in the V2 API.
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
    pub headers: HashMap<String, String>,
    pub x_smtpapi: String
}

impl Mail {
    /// Returns a new Mail struct to send with a client. All of the fields are
    /// initially empty.
    pub fn new() -> Mail {
        Mail {to: Vec::new(), to_names: Vec::new(), cc: Vec::new(),
            bcc: Vec::new(), from: "", subject: "", html: "", text: "",
            from_name: "", reply_to: "", date: String::new(),
            attachments: HashMap::new(), content: HashMap::new(),
            headers: HashMap::new(), x_smtpapi: String::new()}
    }

    /// Adds a CC recipient to the Mail struct.
    pub fn add_cc(&mut self, cc_addr: &'static str) {
        self.cc.push(cc_addr.to_string())
    }

    /// Adds a to recipient to the Mail struct.
    pub fn add_to(&mut self, to_addr: &'static str) {
        self.to.push(to_addr.to_string())
    }

    /// Set the from address for the Mail struct. This can be changed, but there
    /// is only one from address per message.
    pub fn add_from(&mut self, from_addr: &'static str) {
        self.from = from_addr
    }

    /// Set the subject of the message.
    pub fn add_subject(&mut self, subject: &'static str) {
        self.subject = subject
    }

    /// This function sets the HTML content for the message.
    pub fn add_html(&mut self, html: &'static str) {
        self.html = html
    }

    /// Add a name for the "to" field in the message. The number of to names
    /// must match the number of "to" addresses.
    pub fn add_to_name(&mut self, to_name: &'static str) {
        self.to_names.push(to_name.to_string());
    }

    /// Set the text content of the message.
    pub fn add_text(&mut self, text: &'static str) {
        self.text = text
    }

    /// Add a BCC address to the message.
    pub fn add_bcc(&mut self, bcc_addr: &'static str) {
        self.bcc.push(bcc_addr.to_string())
    }

    /// Set the from name for the message.
    pub fn add_from_name(&mut self, from_name: &'static str) {
        self.from_name = from_name
    }

    /// Set the reply to address for the message.
    pub fn add_reply_to(&mut self, reply_to: &'static str) {
        self.reply_to = reply_to
    }

    /// Set the date for the message. This must be a valid RFC 822 timestamp.
    pub fn add_date(&mut self, date: String) {
        self.date = date
    }

    /// Add an attachment for the message. You can pass the name of a file as a
    /// path on the file system.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut message = Mail::new();
    /// message.add_attachment("/path/to/file/contents.txt");
    /// ```
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

    /// Add content for inline images in the message.
    pub fn add_content(&mut self, id: &str, value: &str) {
        self.content.insert(id.to_string(), value.to_string());
    }

    /// Add a custom header for the message. These are usually prefixed with
    /// 'X' or 'x' per the RFC specifications.
    pub fn add_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_string(), value.to_string());
    }

    /// Used internally for string encoding. Not needed for message building.
    pub fn make_header_string(&mut self) -> String {
        let headers = json::encode(&self.headers);
        match headers {
            Ok(h) => h,
            Err(e) => { panic!("Could not encode headers: {:?}", e); }
        }
    }

    /// Add an X-SMTPAPI string to the message. This can be done by using the
    /// 'rustc_serialize' crate and JSON encoding a map or custom struct. Or
    /// a regular String type can be escaped and used.
    pub fn add_x_smtpapi(&mut self, x_smtpapi: String) {
        self.x_smtpapi = x_smtpapi
    }
}
