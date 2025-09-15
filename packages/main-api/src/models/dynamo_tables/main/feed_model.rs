use super::base_model::*;
use super::serde_helpers as sh;
use crate::types::dynamo_entity_type::EntityType;
use dto::{Feed, FeedType, FeedStatus, UrlType, File, FeedBookmarkUser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoFeed {
    #[serde(flatten)]
    pub base: BaseModel,
    pub id: i64,
    #[serde(with = "sh::feed_type_num")]
    pub feed_type: FeedType,
    pub user_id: i64,
    pub industry_id: i64,
    pub parent_id: Option<i64>,
    pub quote_feed_id: Option<i64>,
    pub title: Option<String>,
    pub html_contents: String,
    pub url: Option<String>,
    #[serde(with = "sh::url_type_num")]
    pub url_type: UrlType,
    pub files: Vec<File>,
    pub rewards: i64,
    #[serde(with = "sh::feed_status_num")]
    pub status: FeedStatus,
    pub likes: i64,
    pub comments: i64,
    pub shares: i64,
    pub is_liked: bool,
    pub is_bookmarked: bool,
    pub onboard: bool,
    // Denormalized fields
    pub author_nickname: String,
    pub author_profile_url: Option<String>,
    pub industry_name: String,
}

impl DynamoFeed {
    pub fn from_postgres_feed(feed: &Feed, author_nickname: String, author_profile_url: Option<String>, industry_name: String) -> Self {
        let pk = format!("{}#{}", FEED_PREFIX, feed.id);
        let sk = METADATA_SK.to_string();
        
        // Create base model with GSIs for different access patterns
        let base = BaseModel::new(pk, sk, EntityType::Feed)
            .with_gsi1(format!("{}#{}", USER_PREFIX, feed.user_id), Some(format!("FEED#{}", feed.created_at)))
            .with_gsi2(format!("{}#{}", INDUSTRY_PREFIX, feed.industry_id), Some(format!("FEED#{}", feed.created_at)));

        Self {
            base,
            id: feed.id,
            feed_type: feed.feed_type,
            user_id: feed.user_id,
            industry_id: feed.industry_id,
            parent_id: feed.parent_id,
            quote_feed_id: feed.quote_feed_id,
            title: feed.title.clone(),
            html_contents: feed.html_contents.clone(),
            url: feed.url.clone(),
            url_type: feed.url_type,
            files: feed.files.clone(),
            rewards: feed.rewards,
            status: feed.status,
            likes: feed.likes,
            comments: feed.comments,
            shares: feed.shares,
            is_liked: feed.is_liked,
            is_bookmarked: feed.is_bookmarked,
            onboard: feed.onboard,
            author_nickname,
            author_profile_url,
            industry_name,
        }
    }

    pub fn from_postgres_feed_bookmark(bookmark: &FeedBookmarkUser) -> FeedBookmark {
        FeedBookmark::new(
            bookmark.feed_id,
            bookmark.user_id,
            "Unknown".to_string() // TODO: Need to fetch user nickname
        )
    }
}

impl DynamoModel for DynamoFeed {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedComment {
    #[serde(flatten)]
    pub base: BaseModel,
    pub parent_feed_id: i64,
    pub comment_feed_id: i64,
    pub user_id: i64,
    pub html_contents: String,
    pub likes: i64,
    pub author_nickname: String,
    pub author_profile_url: Option<String>,
}

impl FeedComment {
    pub fn new(parent_feed_id: i64, comment_feed_id: i64, user_id: i64, html_contents: String, author_nickname: String, author_profile_url: Option<String>) -> Self {
        let pk = format!("{}#{}", FEED_PREFIX, parent_feed_id);
        let sk = format!("{}#{}#{}", COMMENT_PREFIX, chrono::Utc::now().timestamp(), comment_feed_id);
        
        let base = BaseModel::new(pk, sk, EntityType::Comment)
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("COMMENT#{}", chrono::Utc::now().timestamp())));

        Self {
            base,
            parent_feed_id,
            comment_feed_id,
            user_id,
            html_contents,
            likes: 0,
            author_nickname,
            author_profile_url,
        }
    }
}

impl DynamoModel for FeedComment {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedLike {
    #[serde(flatten)]
    pub base: BaseModel,
    pub feed_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub liked_at: i64,
}

impl FeedLike {
    pub fn new(feed_id: i64, user_id: i64, user_nickname: String) -> Self {
        let pk = format!("{}#{}", FEED_PREFIX, feed_id);
        let sk = format!("{}#{}", LIKE_PREFIX, user_id);
        let liked_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, EntityType::Like)
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("FEED_LIKE#{}", liked_at)));

        Self {
            base,
            feed_id,
            user_id,
            user_nickname,
            liked_at,
        }
    }
}

impl DynamoModel for FeedLike {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedBookmark {
    #[serde(flatten)]
    pub base: BaseModel,
    pub feed_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub bookmarked_at: i64,
}

impl FeedBookmark {
    pub fn new(feed_id: i64, user_id: i64, user_nickname: String) -> Self {
        let pk = format!("{}#{}", FEED_PREFIX, feed_id);
        let sk = format!("{}#{}", BOOKMARK_PREFIX, user_id);
        let bookmarked_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, EntityType::Bookmark)
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("FEED_BOOKMARK#{}", bookmarked_at)));

        Self {
            base,
            feed_id,
            user_id,
            user_nickname,
            bookmarked_at,
        }
    }
}

impl DynamoModel for FeedBookmark {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
