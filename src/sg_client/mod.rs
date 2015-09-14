use mail::Mail;

use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, Bearer, Headers};

static API_URL: &'static str = "https://api.sendgrid.com/api/mail.send.json?";

pub struct SGClient {
    api_key: String,
}

impl SGClient {
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

        let post_body = format!("to={}&from={}&subject={}&html={}", mail_info.to,
                                mail_info.from, mail_info.subject, mail_info.html);

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
