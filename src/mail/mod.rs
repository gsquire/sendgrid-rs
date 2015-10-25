#[derive(Debug)]
pub struct Mail {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub from: &'static str,
    pub subject: &'static str,
    pub html: &'static str,
}

impl Mail {
    pub fn new() -> Mail {
        Mail {to: Vec::new(), cc: Vec::new(), from: "", subject: "", html: ""}
    }

    pub fn add_cc(&mut self, cc_addr: &'static str) {
        self.cc.push(cc_addr.to_string())
    }

    pub fn add_to(&mut self, to_addr: &'static str) {
        self.to.push(to_addr.to_string())
    }

    pub fn add_from(&mut self, from_addr: &'static str) {
        self.from = from_addr
    }

    pub fn add_subject(&mut self, subject: &'static str) {
        self.subject = subject
    }

    pub fn add_html(&mut self, html: &'static str) {
        self.html = html
    }
}
