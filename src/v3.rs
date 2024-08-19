//! This module encompasses all types needed to send mail using version 3 of the mail
//! send API.

use std::collections::{HashMap, HashSet};

use data_encoding::BASE64;
use reqwest::header::{self, HeaderMap, HeaderValue, InvalidHeaderValue};
use serde::Serialize;
use serde_json::{to_value, value::Value, value::Value::Object, Map};

#[cfg(feature = "blocking")]
use reqwest::blocking::Response as BlockingResponse;
use reqwest::{Client, Response};

use crate::error::{RequestNotSuccessful, SendgridError, SendgridResult};

const V3_API_URL: &str = "https://api.sendgrid.com/v3/mail/send";

/// Just a redefinition of a map to store string keys and values.
pub type SGMap = HashMap<String, String>;

/// Used to send a V3 message body.
#[derive(Clone, Debug)]
pub struct Sender {
    api_key: String,
    client: Client,
    #[cfg(feature = "blocking")]
    blocking_client: reqwest::blocking::Client,
    host: String,
}

/// Used for open tracking settings.
#[derive(Clone, Serialize)]
pub struct OpenTrackingSetting {
    /// Whether or not to enable open tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// The substitution tag to use for the open tracking URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_tag: Option<String>,
}

/// Used for subscription tracking settings.
#[derive(Clone, Serialize)]
pub struct SubscriptionTrackingSetting {
    /// Whether or not to enable subscription tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

/// Used for click tracking settings.
#[derive(Clone, Serialize)]
pub struct ClickTrackingSetting {
    /// Whether or not to enable click tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Whether or not to enable text/plain content in the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_text: Option<bool>,
}

/// Used for all tracking settings.
#[derive(Clone, Serialize)]
pub struct TrackingSettings {
    /// Used for click tracking settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<ClickTrackingSetting>,

    /// Used for open tracking settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<OpenTrackingSetting>,

    /// Used for subscription tracking settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tracking: Option<SubscriptionTrackingSetting>,
}

/// The main structure for a V3 API mail send call. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Serialize)]
pub struct Message {
    from: Email,
    subject: String,
    personalizations: Vec<Personalization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ip_pool_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<Email>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<Content>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_settings: Option<TrackingSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    asm: Option<ASM>,
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
    dynamic_template_data: Option<Map<String, Value>>,

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

/// An object allowing you to specify how to handle unsubscribes.
#[derive(Default, Serialize)]
pub struct ASM {
    group_id: u32,
    groups_to_display: HashSet<u32>,
}

impl Sender {
    /// Construct a new V3 message sender.
    pub fn new(api_key: String) -> Sender {
        Sender {
            api_key,
            client: Client::new(),
            #[cfg(feature = "blocking")]
            blocking_client: reqwest::blocking::Client::new(),
            host: V3_API_URL.to_string(),
        }
    }

    /// Sets the host to use for the API. This is useful if you are using a proxy or a local
    /// development server. It should be a full URL, including the protocol.
    pub fn set_host<S: Into<String>>(&mut self, host: S) {
        self.host = host.into();
    }

    /// Sets client to use for the API. This is useful if you want to customize the client.
    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    /// Sets blocking client to use for the API. This is useful if you want to customize the client.
    #[cfg(feature = "blocking")]
    pub fn set_blocking_client(&mut self, blocking_client: reqwest::blocking::Client) {
        self.blocking_client = blocking_client;
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

    /// Send a V3 message and return the HTTP response or an error.
    pub async fn send(&self, mail: &Message) -> SendgridResult<Response> {
        let headers = self.get_headers()?;

        let resp = self
            .client
            .post(&self.host)
            .headers(headers)
            .body(mail.gen_json())
            .send()
            .await?;

        if resp.error_for_status_ref().is_err() {
            return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
        }

        Ok(resp)
    }

    #[cfg(feature = "blocking")]
    /// Send a V3 message and return the HTTP response or an error.
    pub fn blocking_send(&self, mail: &Message) -> SendgridResult<BlockingResponse> {
        let headers = self.get_headers()?;
        let body = mail.gen_json();

        let resp = self
            .blocking_client
            .post(&self.host)
            .headers(headers)
            .body(body)
            .send()?;

        if resp.error_for_status_ref().is_err() {
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
            reply_to: None,
            content: None,
            attachments: None,
            template_id: None,
            categories: None,
            ip_pool_name: None,
            tracking_settings: None,
            asm: None,
        }
    }

    /// Set the from address.
    pub fn set_from(mut self, from: Email) -> Message {
        self.from = from;
        self
    }

    /// Set the Reply-To header.
    pub fn set_reply_to(mut self, reply_to: Email) -> Message {
        self.reply_to = Some(reply_to);
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

    /// Set the IP pool name.
    pub fn set_ip_pool_name(mut self, ip_pool_name: &str) -> Message {
        self.ip_pool_name = Some(String::from(ip_pool_name));
        self
    }

    /// Set tracking settings.
    pub fn set_tracking_settings(mut self, tracking_settings: TrackingSettings) -> Message {
        self.tracking_settings = Some(tracking_settings);
        self
    }

    /// Set unsubscribe settings.
    pub fn set_asm(mut self, asm: ASM) -> Message {
        self.asm = Some(asm);
        self
    }

    /// Add a category.
    pub fn add_category(mut self, category: &str) -> Message {
        self.categories
            .get_or_insert_with(Vec::new)
            .push(String::from(category));
        self
    }

    /// Add multiple categories.
    pub fn add_categories(mut self, categories: &[String]) -> Message {
        self.categories
            .get_or_insert_with(Vec::new)
            .extend_from_slice(categories);
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

    /// Add a custom_args field.
    pub fn add_custom_args(mut self, custom_args: SGMap) -> Personalization {
        self.custom_args
            .get_or_insert_with(|| SGMap::with_capacity(custom_args.len()))
            .extend(custom_args);
        self
    }

    /// Add a substitutions field.
    pub fn add_substitutions(mut self, substitutions: SGMap) -> Personalization {
        self.substitutions
            .get_or_insert_with(|| SGMap::with_capacity(substitutions.len()))
            .extend(substitutions);
        self
    }

    /// Add a dynamic template data field.
    pub fn add_dynamic_template_data(mut self, dynamic_template_data: SGMap) -> Personalization {
        // We can safely unwrap & unreachable here since SGMap will always serialize
        // to a JSON object.
        let new_vals = match to_value(dynamic_template_data).unwrap() {
            Object(map) => map,
            _ => unreachable!(),
        };
        self.dynamic_template_data
            .get_or_insert_with(|| Map::with_capacity(new_vals.len()))
            .extend(new_vals);
        self
    }

    /// Add a dynamic template data fields from a JSON object.
    pub fn add_dynamic_template_data_json<T: Serialize + ?Sized>(
        mut self,
        json_object: &T,
    ) -> SendgridResult<Personalization> {
        let new_vals = match to_value(json_object)? {
            Object(map) => map,
            _ => return Err(SendgridError::InvalidTemplateValue),
        };
        self.dynamic_template_data
            .get_or_insert_with(|| Map::with_capacity(new_vals.len()))
            .extend(new_vals);
        Ok(self)
    }

    /// Set the subject.
    pub fn set_subject(mut self, subject: &str) -> Personalization {
        self.subject = Some(String::from(subject));
        self
    }

    /// Set send at.
    pub fn set_send_at(mut self, send_at: u64) -> Personalization {
        self.send_at = Some(send_at);
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

impl ASM {
    /// Construct an object allowing you to specify how to handle unsubscribes.
    pub fn new() -> Self {
        Default::default()
    }

    /// The unsubscribe group to associate with this email.
    pub fn set_group_id(mut self, group_id: u32) -> Self {
        self.group_id = group_id;
        self
    }

    /// A set containing the unsubscribe groups that you would like to be displayed on the unsubscribe preferences page.
    pub fn set_groups_to_display(
        mut self,
        groups_to_display: HashSet<u32>,
    ) -> SendgridResult<Self> {
        if groups_to_display.len() > 25 {
            return Err(SendgridError::TooManyItems);
        }

        self.groups_to_display = groups_to_display;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::v3::{
        ClickTrackingSetting, Email, Message, OpenTrackingSetting, Personalization,
        SubscriptionTrackingSetting, TrackingSettings, ASM,
    };
    use serde::Serialize;
    use std::collections::HashSet;

    #[derive(Serialize)]
    struct OuterModel {
        inners: Vec<InnerModel>,
    }

    #[derive(Serialize)]
    struct InnerModel {
        x: String,
        y: String,
        z: String,
    }

    #[test]
    fn ip_pool_name() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .set_ip_pool_name("test_ip_pool")
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"ip_pool_name":"test_ip_pool"}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn single_category() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .add_category("test_category")
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"categories":["test_category"]}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn click_tracking_setting() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .set_tracking_settings(TrackingSettings {
                click_tracking: Some(ClickTrackingSetting {
                    enable: Some(true),
                    enable_text: None,
                }),
                open_tracking: None,
                subscription_tracking: None,
            })
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"tracking_settings":{"click_tracking":{"enable":true}}}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn open_tracking_setting() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .set_tracking_settings(TrackingSettings {
                click_tracking: None,
                open_tracking: Some(OpenTrackingSetting {
                    enable: Some(true),
                    substitution_tag: None,
                }),
                subscription_tracking: None,
            })
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"tracking_settings":{"open_tracking":{"enable":true}}}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn subscription_tracking_setting() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .set_tracking_settings(TrackingSettings {
                click_tracking: None,
                open_tracking: None,
                subscription_tracking: Some(SubscriptionTrackingSetting { enable: Some(true) }),
            })
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"tracking_settings":{"subscription_tracking":{"enable":true}}}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn multiple_categories() {
        let json_str_add_vec = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .add_categories(&[
                String::from("test_category1"),
                String::from("test_category2"),
            ])
            .gen_json();
        let json_str_multiple_adds = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .add_category("test_category1")
            .add_category("test_category2")
            .gen_json();
        let json_str_vec_and_single = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .add_category("test_category1")
            .add_categories(&[String::from("test_category2")])
            .gen_json();

        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"categories":["test_category1","test_category2"]}"#;
        assert_eq!(json_str_add_vec, expected);
        assert_eq!(json_str_multiple_adds, expected);
        assert_eq!(json_str_vec_and_single, expected);
    }

    #[test]
    fn dynamic_template_data_sgmap() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(
                Personalization::new(Email::new("to_email@test.com")).add_dynamic_template_data(
                    [
                        ("Norway".to_string(), "100".to_string()),
                        ("Denmark".to_string(), "50".to_string()),
                        ("Iceland".to_string(), "10".to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                ),
            )
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}],"dynamic_template_data":{"Denmark":"50","Iceland":"10","Norway":"100"}}]}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn dynamic_template_data_json() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(
                Personalization::new(Email::new("to_email@test.com"))
                    .add_dynamic_template_data_json(&OuterModel {
                        inners: vec![
                            InnerModel {
                                x: "1".to_string(),
                                y: "2".to_string(),
                                z: "3".to_string(),
                            },
                            InnerModel {
                                x: "1".to_string(),
                                y: "2".to_string(),
                                z: "3".to_string(),
                            },
                        ],
                    })
                    .unwrap(),
            )
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}],"dynamic_template_data":{"inners":[{"x":"1","y":"2","z":"3"},{"x":"1","y":"2","z":"3"}]}}]}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn asm() {
        let json_str = Message::new(Email::new("from_email@test.com"))
            .add_personalization(Personalization::new(Email::new("to_email@test.com")))
            .set_asm(
                ASM::new()
                    .set_group_id(123)
                    .set_groups_to_display(HashSet::from([123]))
                    .unwrap(),
            )
            .gen_json();
        let expected = r#"{"from":{"email":"from_email@test.com"},"subject":"","personalizations":[{"to":[{"email":"to_email@test.com"}]}],"asm":{"group_id":123,"groups_to_display":[123]}}"#;
        assert_eq!(json_str, expected);
    }
}
