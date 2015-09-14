#[derive(Clone, Copy, Debug)]
pub struct Mail {
    pub to: &'static str,
    pub from: &'static str,
    pub subject: &'static str,
    pub html: &'static str,
}

impl Mail {
    pub fn new() -> Mail {
        Mail {to: "", from: "", subject: "", html: ""}
    }

    fn update_mail(&mut self, field: &'static str, value: &'static str) {
        match field {
            "to" => self.to = value,
            "from" => self.from = value,
            "subject" => self.subject = value,
            "html" => self.html = value,
            _ => panic!("Value not supported!")
        }
    }

    pub fn add_to(&mut self, to_addr: &'static str) {
        self.update_mail("to", to_addr);
    }

    pub fn add_from(&mut self, from_addr: &'static str) {
        self.update_mail("from", from_addr);
    }

    pub fn add_subject(&mut self, subject: &'static str) {
        self.update_mail("subject", subject);
    }

    pub fn add_html(&mut self, html: &'static str) {
        self.update_mail("html", html);
    }
}
