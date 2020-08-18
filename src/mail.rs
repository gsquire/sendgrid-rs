use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{SendgridError, SendgridResult};

macro_rules! add_field {
    // Create a setter that appends.
    (
        $(#[$outer:meta])*
        $method:ident << $field:ident: $ty:ty
    ) => {
        $(#[$outer])*
        pub fn $method(mut self, data: $ty) -> Mail<'a> {
            self.$field.push(data);
            self
        }
    };

    // Create a setter that stores.
    (
        $(#[$outer:meta])*
        $method:ident = $field:ident: $ty:ty
    ) => {
        $(#[$outer])*
        pub fn $method(mut self, data: $ty) -> Mail<'a> {
            self.$field = data;
            self
        }
    };

    // Create a setter that inserts into a map.
    (
        $(#[$outer:meta])*
        $method:ident <- $field:ident: $ty:ty
    ) => {
        $(#[$outer])*
        pub fn $method(mut self, id: String, data: $ty) -> Mail<'a> {
            self.$field.insert(id, data);
            self
        }
    };
}

/// A destination is a combination of an email address and a name to whom emails can be sent.
#[derive(Debug)]
pub struct Destination<'a> {
    /// The email address to which the email will be sent.
    pub address: &'a str,

    /// The display name of the recipient.
    pub name: &'a str,
}

impl<'a> From<(&'a str, &'a str)> for Destination<'a> {
    fn from((address, name): (&'a str, &'a str)) -> Self {
        Self { address, name }
    }
}

/// This is a representation of a valid SendGrid message. It has support for
/// all of the fields in the V2 API.
#[derive(Debug, Default)]
pub struct Mail<'a> {
    /// The list of people to whom the email will be sent.
    pub to: Vec<Destination<'a>>,

    /// The list of people that are CC'd in this email.
    pub cc: Vec<&'a str>,

    /// The list of people that are BCC'd in this email.
    pub bcc: Vec<&'a str>,

    /// The email address that will be used as sender.
    pub from: &'a str,

    /// The subject field of the email.
    pub subject: &'a str,

    /// When the client is sufficiently modern (this should almost always be the case), the email is
    /// displayed as HTML.
    pub html: &'a str,

    /// This is used as a fallback when either the client is too old or the HTML field was not
    /// provided.
    pub text: &'a str,

    /// This is the name that will be used as sender.
    pub from_name: &'a str,

    /// This is the email address that is used as a reply to field.
    pub reply_to: &'a str,

    /// The date added to the header of this email. For example `Thu, 21 Dec 2000 16:01:07 +0200`.
    pub date: &'a str,

    /// The attachments of this email, smaller than 7MB.
    pub attachments: HashMap<String, String>,

    /// Content IDs of the files to be used as inline images. Content IDs should match the content
    /// IDS used in the HTML markup.
    pub content: HashMap<String, &'a str>,

    /// A collection of key/value pairs in JSON format. This is specifically for non-SendGrid custom
    /// extension headers. Each key represents a header name and the value the header value.
    ///
    /// ### Example
    /// ```json
    /// {"X-Accept-Language": "en", "X-Mailer": "MyApp"}
    /// ```
    pub headers: HashMap<String, &'a str>,

    /// The `X-SMTPAPI` header that is used.
    pub x_smtpapi: &'a str,
}

impl<'a> Mail<'a> {
    /// Returns a new Mail struct to send with a client. All of the fields are
    /// initially empty.
    pub fn new() -> Mail<'a> {
        Mail::default()
    }

    add_field!(
        /// Adds a CC recipient to the Mail struct.
        add_cc
            << cc: &'a str
    );

    add_field!(
        /// Adds a to recipient to the Mail struct.
        add_to
            << to: Destination<'a>
    );

    add_field!(
        /// Set the from address for the Mail struct. This can be changed, but there
        /// is only one from address per message.
        add_from = from: &'a str
    );

    add_field!(
        /// Set the subject of the message.
        add_subject = subject: &'a str
    );

    add_field!(
        /// This function sets the HTML content for the message.
        add_html = html: &'a str
    );

    add_field!(
        /// Set the text content of the message.
        add_text = text: &'a str
    );

    add_field!(
        /// Add a BCC address to the message.
        add_bcc
            << bcc: &'a str
    );

    add_field!(
        /// Set the from name for the message.
        add_from_name = from_name: &'a str
    );

    add_field!(
        /// Set the reply to address for the message.
        add_reply_to = reply_to: &'a str
    );

    // TODO(richo) Should this be a chronos::Utc ?
    add_field!(
        /// Set the date for the message. This must be a valid RFC 822 timestamp.
        add_date = date: &'a str
    );

    /// Convenience method when using Mail as a builder.
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
            return Err(SendgridError::InvalidFilename);
        }

        Ok(self)
    }

    add_field!(
        /// Add content for inline images in the message.
        add_content <- content: &'a str
    );

    add_field!(
        /// Add a custom header for the message. These are usually prefixed with
        /// 'X' or 'x' per the RFC specifications.
        add_header <- headers: &'a str
    );

    /// Used internally for string encoding. Not needed for message building.
    pub(crate) fn make_header_string(&mut self) -> SendgridResult<String> {
        let string = serde_json::to_string(&self.headers)?;
        Ok(string)
    }

    add_field!(
        /// Add an X-SMTPAPI string to the message. This can be done by using the `serde_json` crate
        /// to JSON encode a map or custom struct. Alternatively a regular `String` type can be
        /// escaped and used.
        add_x_smtpapi = x_smtpapi: &'a str
    );
}
