//! This module encompasses all types needed to send mail using version 3 of the mail
//! send API.
use std::collections::HashMap;

use data_encoding::BASE64;
use reqwest::header::{self, HeaderMap, HeaderValue, InvalidHeaderValue};
use serde::Serialize;
use serde_json;

#[cfg(feature = "async")]
use crate::errors::SendgridError;

use crate::errors::SendgridResult;

const V3_API_URL: &str = "https://api.sendgrid.com/v3/mail/send";

/// Just a redefinition of a map to store string keys and values.
pub type SGMap = HashMap<String, String>;

/// Used to send a V3 message body.
#[derive(Clone, Debug)]
pub struct Sender {
    api_key: String,
}

/// The main structure for a V3 API mail send call. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Default, Serialize)]
pub struct Message {
    from: Email,
    subject: String,
    personalizations: Vec<Personalization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<Content>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,
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
    dynamic_template_data: Option<SGMap>,

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

impl Sender {
    /// Construct a new V3 message sender.
    pub fn new(api_key: String) -> Sender {
        Sender { api_key }
    }

    fn get_headers(&self) -> Result<HeaderMap, InvalidHeaderValue> {
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
        Ok(headers)
    }

    #[cfg(feature = "async")]
    /// Send an asynchronous V3 message and return a future.
    /// The function needs to be polled as futures are lazy. For further information see
    /// [the future documentation](https://docs.rs/futures/0.1.29/futures/future/trait.Future.html) and
    /// [the tokio documentation](https://tokio.rs/docs/getting-started/futures/).
    pub async fn send(&self, mail: &Message) -> SendgridResult<reqwest::Response> {
        use reqwest::Client;

        let headers_res = self.get_headers();

        if let Ok(headers) = headers_res {
            Client::new()
                .post(V3_API_URL)
                .headers(headers)
                .body(mail.gen_json())
                .send()
                .await
                .map_err(|err| SendgridError::from(err))
        } else {
            Err(SendgridError::from(headers_res.unwrap_err()))
        }
    }

    #[cfg(not(feature = "async"))]
    /// Send a V3 message and return the status code or an error from the request.
    pub fn send(&self, mail: &Message) -> SendgridResult<reqwest::blocking::Response> {
        let client = reqwest::blocking::Client::new();
        let headers = self.get_headers()?;
        let body = mail.gen_json();
        let res = client.post(V3_API_URL).headers(headers).body(body).send()?;
        Ok(res)
    }
}

impl Message {
    /// Construct a new V3 message.
    pub fn new() -> Message {
        Message::default()
    }

    /// Set the from address.
    pub fn set_from(mut self, from: Email) -> Message {
        self.from = from;
        self
    }

    /// Set the subject.
    pub fn set_subject(mut self, subject: &str) -> Message {
        self.subject = String::from(subject);
        self
    }

    /// Set the template id.
    pub fn set_template_id(mut self, template_id: &str) -> Message {
        self.template_id = Some(String::from(template_id));
        self
    }

    /// Add content to the message.
    pub fn add_content(mut self, c: Content) -> Message {
        match self.content {
            None => {
                let mut content = Vec::new();
                content.push(c);
                self.content = Some(content);
            }
            Some(ref mut content) => content.push(c),
        };
        self
    }

    /// Add a personalization to the message.
    pub fn add_personalization(mut self, p: Personalization) -> Message {
        self.personalizations.push(p);
        self
    }

    /// Add an attachment to the message.
    pub fn add_attachment(mut self, a: Attachment) -> Message {
        match self.attachments {
            None => {
                let mut attachments = Vec::new();
                attachments.push(a);
                self.attachments = Some(attachments);
            }
            Some(ref mut attachments) => attachments.push(a),
        };
        self
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
    pub fn set_email(mut self, email: &str) -> Email {
        self.email = String::from(email);
        self
    }

    /// Set an optional name.
    pub fn set_name(mut self, name: &str) -> Email {
        self.name = Some(String::from(name));
        self
    }
}

impl Content {
    /// Construct a new content type.
    pub fn new() -> Content {
        Content::default()
    }

    /// Set the type of this content.
    pub fn set_content_type(mut self, content_type: &str) -> Content {
        self.content_type = String::from(content_type);
        self
    }

    /// Set the corresponding message for this content.
    pub fn set_value(mut self, value: &str) -> Content {
        self.value = String::from(value);
        self
    }
}

impl Personalization {
    /// Construct a new personalization block for this message.
    pub fn new() -> Personalization {
        Personalization::default()
    }

    /// Add a to field.
    pub fn add_to(mut self, to: Email) -> Personalization {
        self.to.push(to);
        self
    }

    /// Add a CC field.
    pub fn add_cc(mut self, cc: Email) -> Personalization {
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
        self
    }

    /// Add a BCC field.
    pub fn add_bcc(mut self, bcc: Email) -> Personalization {
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
        self
    }

    /// Add a headers field.
    pub fn add_headers(mut self, headers: SGMap) -> Personalization {
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
        self
    }

    /// Add a dynamic template data field.
    pub fn add_dynamic_template_data(mut self, dynamic_template_data: SGMap) -> Personalization {
        match self.dynamic_template_data {
            None => {
                let mut h = HashMap::new();
                for (name, value) in dynamic_template_data {
                    h.insert(name, value);
                }
                self.dynamic_template_data = Some(h);
            }
            Some(ref mut h) => {
                h.extend(dynamic_template_data);
            }
        }
        self
    }
}

impl Attachment {
    /// Construct a new attachment for this message.
    pub fn new() -> Attachment {
        Attachment::default()
    }

    /// The raw body of the attachment.
    pub fn set_content(mut self, c: &[u8]) -> Attachment {
        self.content = BASE64.encode(c);
        self
    }

    /// The base64 body of the attachment.
    pub fn set_base64_content(mut self, c: &str) -> Attachment {
        self.content = String::from(c);
        self
    }

    /// Sets the filename for the attachment.
    pub fn set_filename(mut self, filename: &str) -> Attachment {
        self.filename = filename.into();
        self
    }

    /// Set an optional mime type. Sendgrid will default to 'application/octet-stream'
    /// if unspecified.
    pub fn set_mime_type(mut self, mime: &str) -> Attachment {
        self.mime_type = Some(String::from(mime));
        self
    }
}
