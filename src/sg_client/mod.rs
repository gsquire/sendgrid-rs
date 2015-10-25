use mail::Mail;

use std::borrow::Cow;
use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, Bearer, Headers};

static API_URL: &'static str = "https://api.sendgrid.com/api/mail.send.json?";

pub struct SGClient {
    api_key: String,
}

impl SGClient {
    fn make_post_body<'a>(self, mail_info: Mail) -> Cow<'a, str> {
        let mut body = String::new();

        body.push_str("to=");
        body.push_str(mail_info.to);

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

        let post_body = self.make_post_body(mail_info).into_owned();
        let full_url = format!("{}{}", API_URL, post_body);
        let mut res = client.post(&full_url[..])
            .headers(headers)
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        println!("Response: {}", body);
    }
}
