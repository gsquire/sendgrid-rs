use reqwest::header::{self, HeaderMap, HeaderValue};

#[cfg(not(feature = "async"))]
use reqwest::blocking::Response;
#[cfg(feature = "async")]
use reqwest::Response;

use url::form_urlencoded::Serializer;

use crate::{
    error::{RequestNotSuccessful, SendgridResult},
    mail::Mail,
};

static API_URL: &str = "https://api.sendgrid.com/api/mail.send.json?";

/// This is the struct that allows you to authenticate to the SendGrid API.
/// It contains the API key which allows you to send messages as well as a blocking request client.
#[derive(Clone, Debug)]
pub struct SGClient {
    api_key: String,
    #[cfg(feature = "async")]
    client: reqwest::Client,
    #[cfg(not(feature = "async"))]
    client: reqwest::blocking::Client,
}

// Given a form value and a key, generate the correct key.
fn make_form_key(form: &str, key: &str) -> String {
    let mut value = String::with_capacity(form.len() + key.len() + 2);
    value.push_str(form);
    value.push('[');
    value.push_str(key);
    value.push(']');

    value
}

// Use the URL form encoder to properly generate the body used in the mail send request.
fn make_post_body(mut mail_info: Mail) -> SendgridResult<String> {
    let body = String::new();
    let mut encoder = Serializer::new(body);

    for to in mail_info.to.iter() {
        encoder.append_pair("to[]", to.address);
        encoder.append_pair("toname[]", to.name);
    }

    for cc in mail_info.cc.iter() {
        encoder.append_pair("cc[]", &cc);
    }

    for bcc in mail_info.bcc.iter() {
        encoder.append_pair("bcc[]", &bcc);
    }

    for (attachment, contents) in &mail_info.attachments {
        encoder.append_pair(&make_form_key("files", attachment), &contents);
    }

    for (id, value) in &mail_info.content {
        encoder.append_pair(&make_form_key("content", id), &value);
    }

    encoder.append_pair("from", &mail_info.from);
    encoder.append_pair("subject", &mail_info.subject);
    encoder.append_pair("html", &mail_info.html);
    encoder.append_pair("text", &mail_info.text);
    encoder.append_pair("fromname", &mail_info.from_name);
    encoder.append_pair("replyto", &mail_info.reply_to);
    encoder.append_pair("date", &mail_info.date);
    encoder.append_pair("headers", &mail_info.make_header_string()?);
    encoder.append_pair("x-smtpapi", &mail_info.x_smtpapi);

    Ok(encoder.finish())
}

impl SGClient {
    /// Makes a new SendGrid cient with the specified API key. This will panic if you are using the
    /// default TLS backend and do not have a default TLS backend available. If you are using the
    /// RustTLS backend, this can never panic because RustTLS is statically linked.
    pub fn new<S: Into<String>>(key: S) -> SGClient {
        #[cfg(feature = "async")]
        use reqwest as rq;
        #[cfg(not(feature = "async"))]
        use reqwest::blocking as rq;

        let builder = rq::ClientBuilder::new();
        #[cfg(feature = "rustls")]
        let builder = builder.use_rustls_tls();
        let client = builder.build().unwrap();

        SGClient {
            api_key: key.into(),
            client,
        }
    }

    /// Sends a messages through the SendGrid API. It takes a Mail struct as an argument. It returns
    /// the string response from the API as JSON.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sendgrid::SendgridError;
    ///
    /// fn main() -> Result<(), SendgridError> {
    ///     let my_secret_key = std::env::var("SENDGRID_KEY").expect("need SENDGRID_KEY to test");
    ///     use sendgrid::{Mail, SGClient};
    ///
    ///     let mail = Mail::new()
    ///         .add_from("my-email@address.com")
    ///         .add_text("hi!")
    ///         .add_subject("Hello")
    ///         .add_to(("your-email@address.com", "Your Name").into());
    ///     let response = SGClient::new(my_secret_key)
    ///         .send(mail)?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(not(feature = "async"))]
    pub fn send(&self, mail_info: Mail) -> SendgridResult<Response> {
        let post_body = make_post_body(mail_info)?;
        let resp = self
            .client
            .post(API_URL)
            .headers(self.headers()?)
            .body(post_body)
            .send()?;

        if let Err(_) = resp.error_for_status_ref() {
            return Err(RequestNotSuccessful::new(resp.status(), resp.text()?).into());
        }

        Ok(resp)
    }

    /// Sends a messages through the SendGrid API. It takes a Mail struct as an argument. It returns
    /// the string response from the API as JSON.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use sendgrid::SendgridError;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), SendgridError> {
    ///     let my_secret_key = std::env::var("SENDGRID_KEY").expect("need SENDGRID_KEY to test");
    ///     use sendgrid::{Mail, SGClient};
    ///
    ///     let mail = Mail::new()
    ///         .add_from("my-email@address.com")
    ///         .add_text("hi!")
    ///         .add_subject("Hello")
    ///         .add_to(("your-email@address.com", "Your Name").into());
    ///     let response = SGClient::new(my_secret_key)
    ///         .send(mail)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn send(&self, mail_info: Mail<'_>) -> SendgridResult<Response> {
        let post_body = make_post_body(mail_info)?;
        let resp = self
            .client
            .post(API_URL)
            .headers(self.headers()?)
            .body(post_body)
            .send()
            .await?;

        if let Err(_) = resp.error_for_status_ref() {
            return Err(RequestNotSuccessful::new(resp.status(), resp.text().await?).into());
        }

        Ok(resp)
    }

    fn headers(&self) -> SendgridResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key.clone()))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        headers.insert(header::USER_AGENT, HeaderValue::from_static("sendgrid-rs"));
        Ok(headers)
    }
}

#[test]
fn basic_message_body() {
    use crate::mail::Destination;

    let m = Mail::new()
        .add_to(Destination {
            address: "test@example.com",
            name: "Testy mcTestFace",
        })
        .add_from("me@example.com")
        .add_subject("Test")
        .add_text("It works");

    let body = make_post_body(m);
    let want = "to%5B%5D=test%40example.com&toname%5B%5D=Testy+mcTestFace&from=me%40example.com&subject=Test&\
                html=&text=It+works&fromname=&replyto=&date=&headers=%7B%7D&x-smtpapi=";
    assert_eq!(body.unwrap(), want);
}

#[test]
fn test_proper_key() {
    let want = "files[test.jpg]";
    let got = make_form_key("files", "test.jpg");
    assert_eq!(want, got);
}
