use mail::Mail;

use std::borrow::Cow;
use std::io::Read;

use hyper::Client;
use hyper::error::Error;
use hyper::header::{Authorization, Bearer, ContentType, Headers, UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel};

static API_URL: &'static str = "https://api.sendgrid.com/api/mail.send.json?";

/// This is the struct that allows you to authenticate to the SendGrid API.
/// It's only field is the API key which allows you to send messages.
pub struct SGClient {
    api_key: String,
}

fn make_post_body<'a>(mut mail_info: Mail) -> Cow<'a, str> {
    let mut body = String::new();

    // The leading POST data should not start with an ampersand.
    let first_to = mail_info.to.remove(0);
    body.push_str("to[]=");
    body.push_str(&first_to[..]);

    // Now, add anymore if need be.
    for to in mail_info.to.iter() {
        body.push_str("&to[]=");
        body.push_str(&to[..]);
    }

    for to_name in mail_info.to_names.iter() {
        body.push_str("&toname[]=");
        body.push_str(&to_name[..]);
    }

    for cc in mail_info.cc.iter() {
        body.push_str("&cc[]=");
        body.push_str(&cc[..]);
    }

    for bcc in mail_info.bcc.iter() {
        body.push_str("&bcc[]=");
        body.push_str(&bcc[..]);
    }

    for (attachment, contents) in &mail_info.attachments {
        body.push_str("&files[");
        body.push_str(attachment);
        body.push_str("]=");
        body.push_str(contents);
    }

    for (id, value) in &mail_info.content {
        body.push_str("&content[");
        body.push_str(id);
        body.push_str("]=");
        body.push_str(value);
    }

    body.push_str("&from=");
    body.push_str(&mail_info.from);

    body.push_str("&subject=");
    body.push_str(&mail_info.subject);

    body.push_str("&html=");
    body.push_str(&mail_info.html);

    body.push_str("&text=");
    body.push_str(&mail_info.text);

    body.push_str("&fromname=");
    body.push_str(&mail_info.from_name);

    body.push_str("&replyto=");
    body.push_str(&mail_info.reply_to);

    body.push_str("&date=");
    body.push_str(&mail_info.date[..]);

    body.push_str("&headers=");
    body.push_str(&mail_info.make_header_string()[..]);

    body.push_str("&x-smtpapi=");
    body.push_str(&mail_info.x_smtpapi[..]);

    body.into()
}

impl SGClient {
    /// Makes a new SendGrid cient with the specified API key.
    pub fn new(key: String) -> SGClient {
        SGClient {api_key: key}
    }

    /// Sends a messages through the SendGrid API. It takes a Mail struct as an
    /// argument. It returns the string response from the API as JSON.
    /// It sets the Content-Type to be application/x-www-form-urlencoded.
    pub fn send(self, mail_info: Mail) -> Result<String, Error> {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Bearer { token: self.api_key.to_owned() }
                )
        );
        headers.set(
            ContentType(
                Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])
                )
        );
        headers.set(
            UserAgent("sendgrid-rs".to_owned())
        );

        let post_body = make_post_body(mail_info).into_owned();
        let mut res = try!(client.post(API_URL)
            .headers(headers)
            .body(&post_body[..])
            .send());
        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(body)
    }
}

#[test]
fn basic_message_body() {
    let mut m = Mail::new();
    m.add_to("test@example.com");
    m.add_from("me@example.com");
    m.add_subject("Test");
    m.add_text("It works");

    let body = make_post_body(m);
    let comparison = "to[]=test@example.com&from=me@example.com&subject=Test\
        &html=&text=It works&fromname=&replyto=&date=&headers={}&x-smtpapi=";
    assert_eq!(body, comparison);
}
