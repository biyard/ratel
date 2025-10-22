use crate::{
    types::{EntityType, Partition},
    utils::telegram::{ArcTelegramBot, TelegramButton},
};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct TelegramChannel {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub chat_id: i64,
    pub lang: Option<String>,
}

impl TelegramChannel {
    pub fn new(chat_id: i64, lang: Option<String>) -> Self {
        Self {
            pk: Partition::TelegramChannel,
            sk: EntityType::TelegramChannel(chat_id.to_string()), // Note
            created_at: 0,
            updated_at: 0,
            chat_id,
            lang,
        }
    }
    pub async fn add_channel(
        cli: &aws_sdk_dynamodb::Client,
        chat_id: i64,
        lang: Option<String>,
    ) -> crate::Result<Self> {
        let channel = TelegramChannel::new(chat_id, lang);
        channel.create(cli).await?;
        Ok(channel)
    }

    pub async fn remove_channel(
        cli: &aws_sdk_dynamodb::Client,
        chat_id: i64,
    ) -> crate::Result<Self> {
        let res = TelegramChannel::delete(
            cli,
            &Partition::TelegramChannel,
            Some(EntityType::TelegramChannel(chat_id.to_string())),
        )
        .await?;
        Ok(res)
    }
    pub async fn send_message_to_channels(
        cli: &aws_sdk_dynamodb::Client,
        bot: &ArcTelegramBot,
        content: String,
        button: Option<TelegramButton>,
    ) -> crate::Result<()> {
        let mut bookmark = None::<String>;
        loop {
            let mut options = TelegramChannelQueryOption::builder().limit(100);
            if let Some(b) = &bookmark {
                options = options.bookmark(b.clone());
            }

            let (queried_channels, next_bookmark) =
                TelegramChannel::query(cli, Partition::TelegramChannel, options).await?;
            let chat_ids: Vec<i64> = queried_channels.into_iter().map(|c| c.chat_id).collect();
            bot.send_message(chat_ids, &content, button.clone()).await?;
            match next_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }
        Ok(())
    }
}
