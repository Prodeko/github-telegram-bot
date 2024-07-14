use envconfig::Envconfig;
use reqwest::Url;

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "PORT", default = "443")]
    pub port: u16,
    #[envconfig(from = "TELEGRAM_API_URL_BASE", default = "https://api.telegram.org/")]
    pub telegram_api_url_base: Url,
    #[envconfig(from = "TELEGRAM_TOKEN")]
    pub telegram_token: String,
    #[envconfig(from = "GITHUB_TOKEN")]
    pub github_token: String,
    #[envconfig(from = "CHAT_ID")]
    pub chat_id: i64,
}

pub trait Settings {
    fn authorized_telegram_url(&self) -> Url;
}

impl Settings for Config {
    fn authorized_telegram_url(&self) -> Url {
        let token = &self.telegram_token;

        return self
            .telegram_api_url_base
            .join(format!("/bot{token}/").as_str())
            .unwrap();
    }
}
