//! This module encompasses all types needed to send mail using version 3 of the mail
//! send API.

use std::collections::HashMap;

use data_encoding::BASE64;
use reqwest::header::{self, HeaderMap, HeaderValue, InvalidHeaderValue};
use serde::Serialize;

#[cfg(not(feature = "async"))]
use reqwest::blocking::{Client, Response};
#[cfg(feature = "async")]
use reqwest::{Client, Response};

use crate::error::{RequestNotSuccessful, SendgridResult};

const V3_API_URL: &str = "https://api.sendgrid.com/v3/mail/send";

/// Just a redefinition of a map to store string keys and values.
pub type SGMap = HashMap<String, String>;

/// Used to send a V3 message body.
#[derive(Clone, Debug)]
pub struct Sender {
    api_key: String,
    client: Client,
}

/// The main structure for a V3 API mail send call. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Serialize)]
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

/// The Content-Disposition of the attachment specifying how you would like the attachment to be
/// displayed. For example, inline results in the attached file being displayed automatically
/// within the message. By specifying attachment, it will prompt the user to either view or
/// download the file.
#[derive(Clone, Copy, Serialize)]
pub enum Disposition {
    /// Displayed automatically within the message.
    #[serde(rename = "inline")]
    Inline,

    /// Displayed as an attached file.
    #[serde(rename = "attachment")]
    Attachment,
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
    disposition: Option<Disposition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_id: Option<String>,
}

impl Sender {
    /// Construct a new V3 message sender.
    pub fn new(api_key: String) -> Sender {
        Sender {
            api_key,
            client: Client::new(),
        }
    }

    fn get_headers(&self) -> Result<HeaderMap, InvalidHeaderValue> {
        let mut headers = HeaderMap::with_capacity(3);
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
    /// Send a V3 message and return the HTTP response or an error.
    pub async fn send(&self, mail: &Message) -> SendgridResult<Response> {
        let headers = self.get_headers()?;

        let resp = self
            .client
            .post(V3_API_URL)
            .headers(headers)
            .body(mail.gen_json())
            .send()
            .await?;

        if let Err(_) = resp.error_for_status_ref() {
            return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
        }

        Ok(resp)
    }

    #[cfg(not(feature = "async"))]
    /// Send a V3 message and return the HTTP response or an error.
    pub fn send(&self, mail: &Message) -> SendgridResult<Response> {
        let headers = self.get_headers()?;
        let body = mail.gen_json();

        let resp = self
            .client
            .post(V3_API_URL)
            .headers(headers)
            .body(body)
            .send()?;

        if let Err(_) = resp.error_for_status_ref() {
            return Err(RequestNotSuccessful::new(resp.status(), resp.text()?).into());
        }

        Ok(resp)
    }
}

impl Message {
    /// Construct a new V3 message.
    pub fn new(from: Email) -> Message {
        Message {
            from,
            subject: String::new(),
            personalizations: Vec::new(),
            content: None,
            attachments: None,
            template_id: None,
        }
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
        self.content.get_or_insert_with(Vec::new).push(c);
        self
    }

    /// Add a personalization to the message.
    pub fn add_personalization(mut self, p: Personalization) -> Message {
        self.personalizations.push(p);
        self
    }

    /// Add an attachment to the message.
    pub fn add_attachment(mut self, a: Attachment) -> Message {
        self.attachments.get_or_insert_with(Vec::new).push(a);
        self
    }

    fn gen_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Email {
    /// Construct a new email type with name set as None.
    ///
    /// ```rust
    /// use sendgrid::v3::Email;
    ///
    /// let my_email = Email::new("test@mail.com");
    /// ```
    pub fn new<S: Into<String>>(email: S) -> Email {
        Email {
            email: email.into(),
            name: None,
        }
    }

    /// Set an optional name.
    ///
    /// ```rust
    /// use sendgrid::v3::Email;
    ///
    /// let my_email = Email::new("test@mail.com").set_name("My Name");
    /// ```
    pub fn set_name<S: Into<String>>(mut self, name: S) -> Email {
        self.name = Some(name.into());
        self
    }
}

impl Content {
    /// Construct a new content type.
    pub fn new() -> Content {
        Content::default()
    }

    /// Set the type of this content.
    pub fn set_content_type<S: Into<String>>(mut self, content_type: S) -> Content {
        self.content_type = content_type.into();
        self
    }

    /// Set the corresponding message for this content.
    pub fn set_value<S: Into<String>>(mut self, value: S) -> Content {
        self.value = value.into();
        self
    }
}

impl Personalization {
    /// Construct a new personalization block for this message with a single to address.
    pub fn new(email: Email) -> Personalization {
        Personalization {
            to: vec![email],
            cc: None,
            bcc: None,
            subject: None,
            headers: None,
            substitutions: None,
            custom_args: None,
            dynamic_template_data: None,
            send_at: None,
        }
    }

    /// Add a to field.
    pub fn add_to(mut self, to: Email) -> Personalization {
        self.to.push(to);
        self
    }

    /// Add a CC field.
    pub fn add_cc(mut self, cc: Email) -> Personalization {
        self.cc
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(cc);
        self
    }

    /// Add a BCC field.
    pub fn add_bcc(mut self, bcc: Email) -> Personalization {
        self.bcc
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(bcc);
        self
    }

    /// Add a headers field.
    pub fn add_headers(mut self, headers: SGMap) -> Personalization {
        self.headers
            .get_or_insert_with(|| SGMap::with_capacity(headers.len()))
            .extend(headers);
        self
    }

    /// Add a dynamic template data field.
    pub fn add_dynamic_template_data(mut self, dynamic_template_data: SGMap) -> Personalization {
        self.dynamic_template_data
            .get_or_insert_with(|| SGMap::with_capacity(dynamic_template_data.len()))
            .extend(dynamic_template_data);
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
    pub fn set_base64_content<S: Into<String>>(mut self, c: S) -> Attachment {
        self.content = c.into();
        self
    }

    /// Sets the filename for the attachment.
    pub fn set_filename<S: Into<String>>(mut self, filename: S) -> Attachment {
        self.filename = filename.into();
        self
    }

    /// Set an optional mime type. Sendgrid will default to 'application/octet-stream'
    /// if unspecified.
    pub fn set_mime_type<S: Into<String>>(mut self, mime: S) -> Attachment {
        self.mime_type = Some(mime.into());
        self
    }

    /// Set an optional content id.
    pub fn set_content_idm<S: Into<String>>(mut self, content_id: S) -> Attachment {
        self.content_id = Some(content_id.into());
        self
    }

    /// Set an optional disposition.
    pub fn set_disposition(mut self, disposition: Disposition) -> Attachment {
        self.disposition = Some(disposition);
        self
    }
}
