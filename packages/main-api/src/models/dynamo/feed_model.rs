use super::base_model::*;
use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Error, Result, Feed, FeedType, FeedStatus, UrlType, File, FeedBookmarkUser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoFeed {
    pub base: BaseModel,
    pub id: i64,
    pub feed_type: FeedType,
    pub user_id: i64,
    pub industry_id: i64,
    pub parent_id: Option<i64>,
    pub quote_feed_id: Option<i64>,
    pub title: Option<String>,
    pub html_contents: String,
    pub url: Option<String>,
    pub url_type: UrlType,
    pub files: Vec<File>,
    pub rewards: i64,
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
        let base = BaseModel::new(pk, sk, "FEED".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        // Base model fields
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        if let Some(ref gsi2_pk) = self.base.gsi2_pk {
            item.insert("gsi2_pk".to_string(), string_attr(gsi2_pk));
        }
        if let Some(ref gsi2_sk) = self.base.gsi2_sk {
            item.insert("gsi2_sk".to_string(), string_attr(gsi2_sk));
        }

        // Feed-specific fields
        item.insert("id".to_string(), number_attr(self.id));
        item.insert("feed_type".to_string(), number_attr(self.feed_type as i64));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("industry_id".to_string(), number_attr(self.industry_id));
        
        if let Some(parent_id) = self.parent_id {
            item.insert("parent_id".to_string(), number_attr(parent_id));
        }
        
        if let Some(quote_feed_id) = self.quote_feed_id {
            item.insert("quote_feed_id".to_string(), number_attr(quote_feed_id));
        }
        
        if let Some(ref title) = self.title {
            item.insert("title".to_string(), string_attr(title));
        }
        
        item.insert("html_contents".to_string(), string_attr(&self.html_contents));
        
        if let Some(ref url) = self.url {
            item.insert("url".to_string(), string_attr(url));
        }
        
        item.insert("url_type".to_string(), number_attr(self.url_type as i64));
        
        // Serialize files as JSON
        let files_json = serde_json::to_string(&self.files)
            .map_err(|e| Error::DynamoDbError(format!("Failed to serialize files: {}", e)))?;
        item.insert("files".to_string(), string_attr(&files_json));
        
        item.insert("rewards".to_string(), number_attr(self.rewards));
        item.insert("status".to_string(), number_attr(self.status as i64));
        item.insert("likes".to_string(), number_attr(self.likes));
        item.insert("comments".to_string(), number_attr(self.comments));
        item.insert("shares".to_string(), number_attr(self.shares));
        item.insert("is_liked".to_string(), bool_attr(self.is_liked));
        item.insert("is_bookmarked".to_string(), bool_attr(self.is_bookmarked));
        item.insert("onboard".to_string(), bool_attr(self.onboard));
        
        // Denormalized fields
        item.insert("author_nickname".to_string(), string_attr(&self.author_nickname));
        
        if let Some(ref author_profile_url) = self.author_profile_url {
            item.insert("author_profile_url".to_string(), string_attr(author_profile_url));
        }
        
        item.insert("industry_name".to_string(), string_attr(&self.industry_name));

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        let feed_type_num = extract_number(&item, "feed_type")?;
        let feed_type = match feed_type_num {
            1 => FeedType::Post,
            2 => FeedType::Reply,
            3 => FeedType::Repost,
            4 => FeedType::DocReview,
            _ => FeedType::Post,
        };

        let url_type_num = extract_number(&item, "url_type")?;
        let url_type = match url_type_num {
            0 => UrlType::None,
            1 => UrlType::Image,
            _ => UrlType::None,
        };

        let status_num = extract_number(&item, "status")?;
        let status = match status_num {
            1 => FeedStatus::Draft,
            2 => FeedStatus::Published,
            _ => FeedStatus::Published,
        };

        // Deserialize files from JSON
        let files_json = extract_string(&item, "files")?;
        let files: Vec<File> = serde_json::from_str(&files_json)
            .map_err(|e| Error::DynamoDbError(format!("Failed to deserialize files: {}", e)))?;

        Ok(Self {
            base,
            id: extract_number(&item, "id")?,
            feed_type,
            user_id: extract_number(&item, "user_id")?,
            industry_id: extract_number(&item, "industry_id")?,
            parent_id: extract_optional_number(&item, "parent_id"),
            quote_feed_id: extract_optional_number(&item, "quote_feed_id"),
            title: extract_optional_string(&item, "title"),
            html_contents: extract_string(&item, "html_contents")?,
            url: extract_optional_string(&item, "url"),
            url_type,
            files,
            rewards: extract_number(&item, "rewards")?,
            status,
            likes: extract_number(&item, "likes")?,
            comments: extract_number(&item, "comments")?,
            shares: extract_number(&item, "shares")?,
            is_liked: extract_bool(&item, "is_liked")?,
            is_bookmarked: extract_bool(&item, "is_bookmarked")?,
            onboard: extract_bool(&item, "onboard")?,
            author_nickname: extract_string(&item, "author_nickname")?,
            author_profile_url: extract_optional_string(&item, "author_profile_url"),
            industry_name: extract_string(&item, "industry_name")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedComment {
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
        
        let base = BaseModel::new(pk, sk, "FEED_COMMENT".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        
        item.insert("parent_feed_id".to_string(), number_attr(self.parent_feed_id));
        item.insert("comment_feed_id".to_string(), number_attr(self.comment_feed_id));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("html_contents".to_string(), string_attr(&self.html_contents));
        item.insert("likes".to_string(), number_attr(self.likes));
        item.insert("author_nickname".to_string(), string_attr(&self.author_nickname));
        
        if let Some(ref url) = self.author_profile_url {
            item.insert("author_profile_url".to_string(), string_attr(url));
        }

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            parent_feed_id: extract_number(&item, "parent_feed_id")?,
            comment_feed_id: extract_number(&item, "comment_feed_id")?,
            user_id: extract_number(&item, "user_id")?,
            html_contents: extract_string(&item, "html_contents")?,
            likes: extract_number(&item, "likes")?,
            author_nickname: extract_string(&item, "author_nickname")?,
            author_profile_url: extract_optional_string(&item, "author_profile_url"),
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedLike {
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
        
        let base = BaseModel::new(pk, sk, "FEED_LIKE".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        
        item.insert("feed_id".to_string(), number_attr(self.feed_id));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("user_nickname".to_string(), string_attr(&self.user_nickname));
        item.insert("liked_at".to_string(), number_attr(self.liked_at));

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            feed_id: extract_number(&item, "feed_id")?,
            user_id: extract_number(&item, "user_id")?,
            user_nickname: extract_string(&item, "user_nickname")?,
            liked_at: extract_number(&item, "liked_at")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedBookmark {
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
        
        let base = BaseModel::new(pk, sk, "FEED_BOOKMARK".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        
        item.insert("feed_id".to_string(), number_attr(self.feed_id));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("user_nickname".to_string(), string_attr(&self.user_nickname));
        item.insert("bookmarked_at".to_string(), number_attr(self.bookmarked_at));

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            feed_id: extract_number(&item, "feed_id")?,
            user_id: extract_number(&item, "user_id")?,
            user_nickname: extract_string(&item, "user_nickname")?,
            bookmarked_at: extract_number(&item, "bookmarked_at")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}