use mail::Mail;

use std::borrow::Cow;
use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, Bearer, ContentType, Headers, UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel};

static API_URL: &'static str = "https://api.sendgrid.com/api/mail.send.json?";

pub struct SGClient {
    api_key: String,
}

impl SGClient {
    fn make_post_body<'a>(self, mut mail_info: Mail) -> Cow<'a, str> {
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

        for cc in mail_info.cc.iter() {
            body.push_str("&cc[]=");
            body.push_str(&cc[..]);
        }

        body.push_str("&from=");
        body.push_str(mail_info.from);

        body.push_str("&subject=");
        body.push_str(mail_info.subject);

        body.push_str("&html=");
        body.push_str(mail_info.html);

        body.into()
    }

    pub fn new(key: String) -> SGClient {
        SGClient {api_key: key}
    }

    pub fn send(self, mail_info: Mail) {
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

        let post_body = self.make_post_body(mail_info).into_owned();
        let mut res = client.post(API_URL)
            .headers(headers)
            .body(&post_body[..])
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        println!("Response: {}", body);
    }
}
