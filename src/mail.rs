use errors::{SendgridErrorKind, SendgridResult};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json;

#[derive(Debug)]
/// This is a representation of a valid SendGrid message. It has support for
/// all of the fields in the V2 API.
pub struct Mail<'a> {
    pub to: Vec<&'a str>,
    pub to_names: Vec<&'a str>,
    pub cc: Vec<&'a str>,
    pub bcc: Vec<&'a str>,
    pub from: &'a str,
    pub subject: &'a str,
    pub html: &'a str,
    pub text: &'a str,
    pub from_name: &'a str,
    pub reply_to: &'a str,
    pub date: &'a str,
    pub attachments: HashMap<String, String>,
    pub content: HashMap<String, &'a str>,
    pub headers: HashMap<String, &'a str>,
    pub x_smtpapi: &'a str,
}

impl<'a> Mail<'a> {
    /// Returns a new Mail struct to send with a client. All of the fields are
    /// initially empty.
    pub fn new() -> Mail<'a> {
        Mail {
            to: Vec::new(),
            to_names: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            from: "",
            subject: "",
            html: "",
            text: "",
            from_name: "",
            reply_to: "",
            date: "",
            attachments: HashMap::new(),
            content: HashMap::new(),
            headers: HashMap::new(),
            x_smtpapi: "",
        }
    }

    /// Adds a CC recipient to the Mail struct.
    pub fn add_cc(mut self, cc_addr: &'a str) -> Mail<'a> {
        self.cc.push(cc_addr.as_ref());
        self
    }

    /// Adds a to recipient to the Mail struct.
    pub fn add_to(mut self, to_addr: &'a str) -> Mail<'a> {
        self.to.push(to_addr.as_ref());
        self
    }

    /// Set the from address for the Mail struct. This can be changed, but there
    /// is only one from address per message.
    pub fn add_from(mut self, from_addr: &'a str) -> Mail<'a> {
        self.from = from_addr.as_ref();
        self
    }

    /// Set the subject of the message.
    pub fn add_subject(mut self, subject: &'a str) -> Mail<'a> {
        self.subject = subject.as_ref();
        self
    }

    /// This function sets the HTML content for the message.
    pub fn add_html(mut self, html: &'a str) -> Mail<'a> {
        self.html = html.as_ref();
        self
    }

    /// Add a name for the "to" field in the message. The number of to names
    /// must match the number of "to" addresses.
    pub fn add_to_name(mut self, to_name: &'a str) -> Mail<'a> {
        self.to_names.push(to_name.as_ref());;
        self
    }

    /// Set the text content of the message.
    pub fn add_text(mut self, text: &'a str) -> Mail<'a> {
        self.text = text.as_ref();
        self
    }

    /// Add a BCC address to the message.
    pub fn add_bcc(mut self, bcc_addr: &'a str) -> Mail<'a> {
        self.bcc.push(bcc_addr.as_ref());
        self
    }

    /// Set the from name for the message.
    pub fn add_from_name(mut self, from_name: &'a str) -> Mail<'a> {
        self.from_name = from_name.as_ref();
        self
    }

    /// Set the reply to address for the message.
    pub fn add_reply_to(mut self, reply_to: &'a str) -> Mail<'a> {
        self.reply_to = reply_to.as_ref();
        self
    }

    /// Set the date for the message. This must be a valid RFC 822 timestamp.
    pub fn add_date(mut self, date: &'a str) -> Mail<'a> {
        self.date = date.as_ref();
        self
    }

    /// Convenience method when using Mail as a builder
    pub fn build(self) -> Mail<'a> {
        self
    }

    /// Add an attachment for the message. You can pass the name of a file as a
    /// path on the file system.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let message = Mail::new()
    ///     .add_attachment("/path/to/file/contents.txt");
    /// ```
    pub fn add_attachment<P: AsRef<Path>>(mut self, path: P) -> SendgridResult<Mail<'a>> {
        let mut file = File::open(&path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        if let Some(name) = path.as_ref().to_str() {
            self.attachments.insert(String::from(name), data);
        } else {
            return Err(SendgridErrorKind::InvalidFilename.into());
        }

        Ok(self)
    }

    /// Add content for inline images in the message.
    pub fn add_content(mut self, id: String, value: &'a str) -> Mail<'a> {
        self.content.insert(id, value);
        self
    }

    /// Add a custom header for the message. These are usually prefixed with
    /// 'X' or 'x' per the RFC specifications.
    pub fn add_header(mut self, header: String, value: &'a str) -> Mail<'a> {
        self.headers.insert(header, value);
        self
    }

    /// Used internally for string encoding. Not needed for message building.
    pub(crate) fn make_header_string(&mut self) -> SendgridResult<String> {
        let string = serde_json::to_string(&self.headers)?;
        Ok(string)
    }

    /// Add an X-SMTPAPI string to the message. This can be done by using the
    /// 'rustc_serialize' crate and JSON encoding a map or custom struct. Or
    /// a regular String type can be escaped and used.
    pub fn add_x_smtpapi(mut self, x_smtpapi: &'a str) -> Mail<'a> {
        self.x_smtpapi = x_smtpapi;
        self
    }
}
