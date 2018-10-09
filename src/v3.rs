use std::collections::HashMap;

use data_encoding::BASE64;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json;

pub use reqwest::Response;

use errors::SendgridResult;

const V3_API_URL: &str = "https://api.sendgrid.com/v3/mail/send";

/// Just a redefinition of a map to store string keys and values.
pub type SGMap = HashMap<String, String>;

/// Used to send a V3 message body.
pub struct V3Sender {
    api_key: String,
}

/// The main structure for a V3 API mail send call. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Default, Serialize)]
pub struct SGMailV3 {
    from: Email,
    subject: String,
    content: Vec<Content>,
    personalizations: Vec<Personalization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,
}

/// An email with a required address and an optional name field.
#[derive(Clone, Default, Serialize)]
pub struct Email {
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// The body of an email with the content type and the message.
#[derive(Clone, Default, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    value: String,
}

/// A personalization block for a V3 message. It has to at least contain one email as a to
/// address. All other fields are optional.
#[derive(Default, Serialize)]
pub struct Personalization {
    to: Vec<Email>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<Email>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<Vec<Email>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<SGMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<SGMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_args: Option<SGMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

/// An attachment block for a V3 message. Content and filename are required. If the
/// mime_type is unspecified, the email will use Sendgrid's default for attachments
/// which is 'application/octet-stream'.
#[derive(Default, Serialize)]
pub struct Attachment {
    content: String,

    filename: String,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    disposition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_id: Option<String>,
}

impl V3Sender {
    /// Construct a new V3 message sender.
    pub fn new(api_key: String) -> V3Sender {
        V3Sender { api_key }
    }

    /// Send a V3 message and return the status code or an error from the request.
    pub fn send(&self, mail: &SGMailV3) -> SendgridResult<Response> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key.clone()))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(header::USER_AGENT, HeaderValue::from_static("sendgrid-rs"));

        let body = mail.gen_json();
        let res = client.post(V3_API_URL).headers(headers).body(body).send()?;
        Ok(res)
    }
}

impl SGMailV3 {
    /// Construct a new V3 message.
    pub fn new() -> SGMailV3 {
        SGMailV3::default()
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
        Email::default()
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
        Content::default()
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
        Personalization::default()
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

    /// Add a BCC field.
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

    /// Add a headers field.
    pub fn add_headers(&mut self, headers: SGMap) {
        match self.headers {
            None => {
                let mut h = HashMap::new();
                for (name, value) in headers {
                    h.insert(name, value);
                }
                self.headers = Some(h);
            }
            Some(ref mut h) => {
                h.extend(headers);
            }
        }
    }
}

impl Attachment {
    /// Construct a new attachment for this message.
    pub fn new() -> Attachment {
        Attachment::default()
    }

    /// The raw body of the attachment.
    pub fn set_content(&mut self, c: &[u8]) {
        self.content = BASE64.encode(c);
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
