#[derive(serde::Serialize)]
pub struct SlackMessage<'a> {
    pub text: &'a str,
}

#[macro_export]
macro_rules! notify_error {
    ($hook:expr, $msg:expr) => {
        let client = reqwest::Client::new();
        let payload = btracing::SlackMessage {
            text: &format!("[{}:{}] {}", file!(), line!(), $msg),
        };

        for i in 0..3 {
            if let Ok(_) = client.post($hook).json(&payload).send().await {
                break;
            } else {
                if i == 3 {
                    tracing::error!("Failed to send Slack message");
                    break;
                }
                tracing::warn!("Failed to send Slack message, attempt {}/3", i + 1);
            }
        }
    };
}

#[macro_export]
macro_rules! notify {
    ($hook:expr, $msg:expr) => {
        let client = reqwest::Client::new();
        let payload = btracing::SlackMessage { text: $msg };

        for i in 0..3 {
            if let Ok(_) = client.post($hook).json(&payload).send().await {
                break;
            } else {
                if i == 3 {
                    tracing::error!("Failed to send Slack message");
                    break;
                }
                tracing::warn!("Failed to send Slack message, attempt {}/3", i + 1);
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            tracing::info!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! i {
    ($msg:expr) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            let message = format!("{}", $msg.translate(&Language::En));
            tracing::error!("{}", message);
        }
    };

    ($lang:expr, $msg:expr) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            let message = format!("{}", $msg.translate(&$lang));
            tracing::error!("{}", message);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            tracing::error!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! e {
    ($err:expr) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            let message = format!("{}", $err.translate(&Language::En));
            tracing::error!("{}", message);
        }
    };

    ($lang:expr, $err:expr) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            let message = format!("{}", $err.translate(&$lang));
            tracing::error!("{}", message);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::WARN) {
            tracing::warn!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    }
}
