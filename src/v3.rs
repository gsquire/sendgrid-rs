use errors::SendgridResult;

use std::collections::HashMap;

use reqwest::{Client, StatusCode};
use reqwest::header::{Authorization, Bearer, ContentType, Headers, UserAgent};

use data_encoding::base64;

use serde_json;

const V3_API_URL: &'static str = "https://api.sendgrid.com/v3/mail/send";

/// Just a redefinition of a map to store string keys and values.
pub type SGMap = HashMap<String, String>;

/// Used to send a V3 message body.
pub struct V3Sender {
    api_key: String,
}

/// The main structure for a V3 API mail send call. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Serialize)]
pub struct SGMailV3 {
    from: Email,
    subject: String,
    content: Vec<Content>,
    personalizations: Vec<Personalization>,

    #[serde(skip_serializing_if = "check_encode")]
    attachments: Option<Vec<Attachment>>,
}

/// An email with a required address and an optional name field.
#[derive(Clone, Serialize)]
pub struct Email {
    email: String,

    #[serde(skip_serializing_if = "check_encode")]
    name: Option<String>,
}

/// The body of an email with the content type and the message.
#[derive(Clone, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    value: String,
}

/// A personalization block for a V3 message. It has to at least contain one email as a to
/// address. All other fields are optional.
#[derive(Serialize)]
pub struct Personalization {
    to: Vec<Email>,

    #[serde(skip_serializing_if = "check_encode")]
    cc: Option<Vec<Email>>,

    #[serde(skip_serializing_if = "check_encode")]
    bcc: Option<Vec<Email>>,

    #[serde(skip_serializing_if = "check_encode")]
    subject: Option<String>,

    #[serde(skip_serializing_if = "check_encode")]
    headers: Option<SGMap>,

    #[serde(skip_serializing_if = "check_encode")]
    substitutions: Option<SGMap>,

    #[serde(skip_serializing_if = "check_encode")]
    custom_args: Option<SGMap>,

    #[serde(skip_serializing_if = "check_encode")]
    send_at: Option<u64>,
}

/// An attachment block for a V3 message. Content and filename are required. If the
/// mime_type is unspecified, the email will use Sendgrid's default for attachments
/// which is 'application/octet-stream'.
#[derive(Serialize)]
pub struct Attachment {
    content: String,

    filename: String,

    #[serde(rename = "type", skip_serializing_if = "check_encode")]
    mime_type: Option<String>,

    #[serde(skip_serializing_if = "check_encode")]
    disposition: Option<String>,

    #[serde(skip_serializing_if = "check_encode")]
    content_id: Option<String>,
}

// Checks if a value in the V3 message should be added to the JSON or not.
fn check_encode<T>(value: &Option<T>) -> bool {
    match *value {
        Some(_) => false,
        None => true,
    }
}

impl V3Sender {
    /// Construct a new V3 message sender.
    pub fn new(api_key: String) -> V3Sender {
        V3Sender { api_key: api_key }
    }

    /// Send a V3 message and return the status code or an error from the request.
    pub fn send(&self, mail: &SGMailV3) -> SendgridResult<StatusCode> {
        let client = Client::new()?;
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: self.api_key.to_owned() }));
        headers.set(ContentType::json());
        headers.set(UserAgent::new("sendgrid-rs"));

        let body = mail.gen_json();
        let res = client.post(V3_API_URL)?.headers(headers).body(body).send()?;
        Ok(res.status())
    }
}

impl SGMailV3 {
    /// Construct a new V3 message.
    pub fn new() -> SGMailV3 {
        SGMailV3 {
            from: Email::new(),
            subject: String::new(),
            content: Vec::new(),
            personalizations: Vec::new(),
            attachments: None,
        }
    }

    /// Set the from address.
    pub fn set_from(&mut self, from: Email) {
        self.from = from;
    }

    /// Set the subject.
    pub fn set_subject(&mut self, subject: &str) {
        self.subject = String::from(subject);
    }

    /// Add content to the message.
    pub fn add_content(&mut self, content: Content) {
        self.content.push(content);
    }

    /// Add a personalization to the message.
    pub fn add_personalization(&mut self, p: Personalization) {
        self.personalizations.push(p);
    }

    /// Add an attachment to the message.
    pub fn add_attachment(&mut self, a: Attachment) {
        match self.attachments {
            None => {
                let mut attachments = Vec::new();
                attachments.push(a);
                self.attachments = Some(attachments);
            }
            Some(ref mut attachments) => attachments.push(a),
        };
    }

    fn gen_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Email {
    /// Construct a new email type.
    pub fn new() -> Email {
        Email {
            email: String::new(),
            name: None,
        }
    }

    /// Set the address for this email.
    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }

    /// Set an optional name.
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }
}

impl Content {
    /// Construct a new content type.
    pub fn new() -> Content {
        Content {
            content_type: String::new(),
            value: String::new(),
        }
    }

    /// Set the type of this content.
    pub fn set_content_type(&mut self, content_type: &str) {
        self.content_type = String::from(content_type);
    }

    /// Set the corresponding message for this content.
    pub fn set_value(&mut self, value: &str) {
        self.value = String::from(value);
    }
}

impl Personalization {
    /// Construct a new personalization block for this message.
    pub fn new() -> Personalization {
        Personalization {
            to: Vec::new(),
            cc: None,
            bcc: None,
            subject: None,
            headers: None,
            substitutions: None,
            custom_args: None,
            send_at: None,
        }
    }

    /// Add a to field.
    pub fn add_to(&mut self, to: Email) {
        self.to.push(to);
    }

    /// Add a CC field.
    pub fn add_cc(&mut self, cc: Email) {
        match self.cc {
            None => {
                let mut ccs = Vec::new();
                ccs.push(cc);
                self.cc = Some(ccs);
            }
            Some(ref mut c) => {
                c.push(cc);
            }
        }
    }

    // Add a BCC field
    pub fn add_bcc(&mut self, bcc: Email) {
        match self.bcc {
            None => {
                let mut bccs = Vec::new();
                bccs.push(bcc);
                self.bcc = Some(bccs);
            }
            Some(ref mut b) => {
                b.push(bcc);
            }
        }
    }
}

impl Attachment {
    /// Construct a new attachment for this message.
    pub fn new() -> Attachment {
        Attachment {
            content: String::new(),
            filename: String::new(),
            mime_type: None,
            disposition: None,
            content_id: None,
        }
    }

    /// The raw body of the attachment.
    pub fn set_content(&mut self, c: &[u8]) {
        self.content = base64::encode(c);
    }

    /// Sets the filename for the attachment.
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.into();
    }

    /// Set an optional mime type. Sendgrid will default to 'application/octet-stream'
    /// if unspecified.
    pub fn set_mime_type(&mut self, mime: &str) {
        self.mime_type = Some(String::from(mime));
    }
}
