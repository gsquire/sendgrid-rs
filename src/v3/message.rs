//! Provides types related to [`crate::v3::Message`].

use serde::Serialize;

/// The settings to use when sending the [`crate::v3::Message`].
/// See the [api docs](https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send#request-body)
/// for details.
#[derive(Default, Serialize)]
pub struct MailSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_list_management: Option<BypassListManagement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_spam_management: Option<BypassSpamManagement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_bounce_management: Option<BypassBounceManagement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_unsubscribe_management: Option<BypassUnsubscribeManagement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<Footer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox_mode: Option<SandboxMode>,
}

/// Used for the bypass list management setting.
#[derive(Default, Serialize)]
pub struct BypassListManagement {
    enable: bool,
}

/// Used for the bypass spam management setting.
#[derive(Default, Serialize)]
pub struct BypassSpamManagement {
    enable: bool,
}

/// Used for the bypass bounce management setting.
#[derive(Default, Serialize)]
pub struct BypassBounceManagement {
    enable: bool,
}

/// Used for the bypass unsubscribe management setting.
#[derive(Default, Serialize)]
pub struct BypassUnsubscribeManagement {
    enable: bool,
}

/// Used to provide a footer for the [`crate::v3::Message`].
#[derive(Default, Serialize)]
pub struct Footer {
    enable: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
}

/// Used for the sandbox mode setting.
#[derive(Default, Serialize)]
pub struct SandboxMode {
    enable: bool,
}

impl MailSettings {
    /// Create a new default [`MailSettings`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the bypass list management setting.
    pub fn set_bypass_list_management(
        mut self,
        bypass_list_management: BypassListManagement,
    ) -> Self {
        self.bypass_list_management = Some(bypass_list_management);
        self
    }

    /// Set the bypass spam management setting.
    pub fn set_bypass_spam_management(
        mut self,
        bypass_spam_management: BypassSpamManagement,
    ) -> Self {
        self.bypass_spam_management = Some(bypass_spam_management);
        self
    }

    /// Set the bypass bounce management setting.
    pub fn set_bypass_bounce_management(
        mut self,
        bypass_bounce_management: BypassBounceManagement,
    ) -> Self {
        self.bypass_bounce_management = Some(bypass_bounce_management);
        self
    }

    /// Set the bypass unsubscrie management setting.
    pub fn set_bypass_unsubscribe_management(
        mut self,
        bypass_unsubscribe_management: BypassUnsubscribeManagement,
    ) -> Self {
        self.bypass_unsubscribe_management = Some(bypass_unsubscribe_management);
        self
    }

    /// Set the footer setting.
    pub fn set_footer(mut self, footer: Footer) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the sandbox mode setting.
    pub fn set_sandbox_mode(mut self, sandbox_mode: SandboxMode) -> Self {
        self.sandbox_mode = Some(sandbox_mode);
        self
    }
}

impl BypassListManagement {
    /// Create a new default [`BypassListManagement`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable or disable the setting
    pub fn set_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
}

impl BypassSpamManagement {
    /// Create a new default [`BypassSpamManagement`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable or disable the setting
    pub fn set_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
}

impl BypassBounceManagement {
    /// Create a new default [`BypassBounceManagement`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable or disable the setting
    pub fn set_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
}

impl BypassUnsubscribeManagement {
    /// Create a new default [`BypassUnsubscribeManagement`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable or disable the setting
    pub fn set_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
}

impl Footer {
    /// Create a new default [`Footer`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable or disable the footer
    pub fn set_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }

    /// Set the text of the footer.
    pub fn set_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Set the html content of the footer.
    pub fn set_html(mut self, html: String) -> Self {
        self.html = Some(html);
        self
    }
}

impl SandboxMode {
    /// Create a new default [`SandboxMode`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable or disable the setting
    pub fn set_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mail_settings_empty() {
        let settings = MailSettings::default();
        let settings_json = serde_json::to_string(&settings).unwrap();
        let expected = "{}";
        assert_eq!(settings_json, expected);
    }

    #[test]
    fn mail_settings_defaults() {
        let settings = MailSettings::default()
            .set_bypass_list_management(Default::default())
            .set_bypass_spam_management(Default::default())
            .set_bypass_bounce_management(Default::default())
            .set_bypass_unsubscribe_management(Default::default())
            .set_footer(Default::default())
            .set_sandbox_mode(Default::default());
        let settings_json = serde_json::to_string(&settings).unwrap();
        let expected = r#"{"bypass_list_management":{"enable":false},"bypass_spam_management":{"enable":false},"bypass_bounce_management":{"enable":false},"bypass_unsubscribe_management":{"enable":false},"footer":{"enable":false},"sandbox_mode":{"enable":false}}"#;
        assert_eq!(settings_json, expected);
    }

    #[test]
    fn mail_settings() {
        let settings = MailSettings::new()
            .set_bypass_list_management(BypassListManagement::new().set_enable(true))
            .set_bypass_spam_management(BypassSpamManagement::new().set_enable(true))
            .set_bypass_bounce_management(BypassBounceManagement::new().set_enable(true))
            .set_bypass_unsubscribe_management(BypassUnsubscribeManagement::new().set_enable(true))
            .set_footer(
                Footer::new()
                    .set_enable(true)
                    .set_html("html".to_string())
                    .set_text("text".to_string()),
            )
            .set_sandbox_mode(SandboxMode::new().set_enable(true));
        let settings_json = serde_json::to_string(&settings).unwrap();
        let expected = r#"{"bypass_list_management":{"enable":true},"bypass_spam_management":{"enable":true},"bypass_bounce_management":{"enable":true},"bypass_unsubscribe_management":{"enable":true},"footer":{"enable":true,"text":"text","html":"html"},"sandbox_mode":{"enable":true}}"#;
        assert_eq!(settings_json, expected);
    }
}