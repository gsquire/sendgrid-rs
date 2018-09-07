use errors::{SendgridErrorKind, SendgridResult};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json;

#[derive(Debug)]
/// This is a representation of a valid SendGrid message. It has support for
/// all of the fields in the V2 API.
pub struct Mail {
    pub to: Vec<String>,
    pub to_names: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub from: String,
    pub subject: String,
    pub html: String,
    pub text: String,
    pub from_name: String,
    pub reply_to: String,
    pub date: String,
    pub attachments: HashMap<String, String>,
    pub content: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub x_smtpapi: String,
}

impl Mail {
    /// Returns a new Mail struct to send with a client. All of the fields are
    /// initially empty.
    pub fn new() -> Mail {
        Mail {
            to: Vec::new(),
            to_names: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            from: String::new(),
            subject: String::new(),
            html: String::new(),
            text: String::new(),
            from_name: String::new(),
            reply_to: String::new(),
            date: String::new(),
            attachments: HashMap::new(),
            content: HashMap::new(),
            headers: HashMap::new(),
            x_smtpapi: String::new(),
        }
    }

    /// Adds a CC recipient to the Mail struct.
    pub fn add_cc<T: Into<String>>(mut self, cc_addr: T) -> Mail {
        self.cc.push(cc_addr.into());
        self
    }

    /// Adds a to recipient to the Mail struct.
    pub fn add_to<T: Into<String>>(mut self, to_addr: T) -> Mail {
        self.to.push(to_addr.into());
        self
    }

    /// Set the from address for the Mail struct. This can be changed, but there
    /// is only one from address per message.
    pub fn add_from<T: Into<String>>(mut self, from_addr: T) -> Mail {
        self.from = from_addr.into();
        self
    }

    /// Set the subject of the message.
    pub fn add_subject<T: Into<String>>(mut self, subject: T) -> Mail {
        self.subject = subject.into();
        self
    }

    /// This function sets the HTML content for the message.
    pub fn add_html<T: Into<String>>(mut self, html: T) -> Mail {
        self.html = html.into();
        self
    }

    /// Add a name for the "to" field in the message. The number of to names
    /// must match the number of "to" addresses.
    pub fn add_to_name<T: Into<String>>(mut self, to_name: T) -> Mail {
        self.to_names.push(to_name.into());;
        self
    }

    /// Set the text content of the message.
    pub fn add_text<T: Into<String>>(mut self, text: T) -> Mail {
        self.text = text.into();
        self
    }

    /// Add a BCC address to the message.
    pub fn add_bcc<T: Into<String>>(mut self, bcc_addr: T) -> Mail {
        self.bcc.push(bcc_addr.into());
        self
    }

    /// Set the from name for the message.
    pub fn add_from_name<T: Into<String>>(mut self, from_name: T) -> Mail {
        self.from_name = from_name.into();
        self
    }

    /// Set the reply to address for the message.
    pub fn add_reply_to<T: Into<String>>(mut self, reply_to: T) -> Mail {
        self.reply_to = reply_to.into();
        self
    }

    /// Set the date for the message. This must be a valid RFC 822 timestamp.
    pub fn add_date(mut self, date: String) -> Mail {
        self.date = date;
        self
    }

    /// Convenience method when using Mail as a builder
    pub fn build(self) -> Mail {
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
    pub fn add_attachment<P: AsRef<Path>>(mut self, path: P) -> SendgridResult<Mail> {
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
    pub fn add_content(mut self, id: &str, value: &str) -> Mail {
        self.content.insert(id.to_string(), value.to_string());
        self
    }

    /// Add a custom header for the message. These are usually prefixed with
    /// 'X' or 'x' per the RFC specifications.
    pub fn add_header(mut self, header: &str, value: &str) -> Mail {
        self.headers.insert(header.to_string(), value.to_string());
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
    pub fn add_x_smtpapi(mut self, x_smtpapi: String) -> Mail {
        self.x_smtpapi = x_smtpapi;
        self
    }
}
