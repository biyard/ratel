#[derive(Debug, Copy, Clone)]
pub struct TelegramToken(Option<&'static str>);

impl Default for TelegramToken {
    fn default() -> Self {
        Self(option_env!("TELEGRAM_TOKEN").filter(|s| !s.is_empty()))
    }
}

impl TelegramToken {
    pub fn unwrap_or_default(&self) -> &'static str {
        self.0.unwrap_or("")
    }
}
