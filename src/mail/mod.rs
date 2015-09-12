#[derive(Clone)]
pub struct Mail {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub html: String,
}

impl Mail {
    fn update_mail(mut self, field: &'static str, value: &'static str) {
        match field {
            "to" => self.to = String::from(value),
            "from" => self.from = String::from(value),
            "subject" => self.subject = String::from(value),
            "html" => self.html = String::from(value),
            _ => panic!("Value not supported!")
        }
    }

    pub fn add_to(self, to_addr: &'static str) {
        self.update_mail("to", to_addr);
    }

    pub fn add_from(self, from_addr: &'static str) {
        self.update_mail("from", from_addr);
    }

    pub fn add_subject(self, subject: &'static str) {
        self.update_mail("subject", subject);
    }

    pub fn add_html(self, html: &'static str) {
        self.update_mail("html", html);
    }
}
