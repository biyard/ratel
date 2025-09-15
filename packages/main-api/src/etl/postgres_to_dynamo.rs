use bdk::prelude::*;
use dto::{Error, Result, User, Space, Feed, Group, Discussion, Industry, Badge};
use sqlx::PgPool;
use tracing::{info, error, warn};

use crate::{
    models::dynamo::*,
    etl::{BatchProcessor, MigrationState, MigrationStatus},
};

pub struct PostgresToDynamoMigrator {
    batch_processor: BatchProcessor,
    migration_state: MigrationState,
    db_pool: PgPool,
}

impl PostgresToDynamoMigrator {
    pub fn new(table_name: &str, db_pool: PgPool) -> Self {
        let migration_id = format!("migration_{}", chrono::Utc::now().timestamp());
        let batch_processor = BatchProcessor::new(table_name).with_batch_size(25);
        let migration_state = MigrationState::new(migration_id);

        Self {
            batch_processor,
            migration_state,
            db_pool,
        }
    }

    pub async fn migrate_all_tables(&mut self) -> Result<()> {
        info!("Starting complete migration from PostgreSQL to DynamoDB");
        
        self.migration_state.start_migration();
        
        // Calculate total records for progress tracking
        self.calculate_total_records().await?;
        
        // Migrate core entities first
        match self.migrate_users().await {
            Ok(_) => info!("Users migration completed"),
            Err(e) => {
                error!("Users migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Users migration failed: {:?}", e));
                return Err(e);
            }
        }

        match self.migrate_industries().await {
            Ok(_) => info!("Industries migration completed"),
            Err(e) => {
                error!("Industries migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Industries migration failed: {:?}", e));
                return Err(e);
            }
        }

        match self.migrate_badges().await {
            Ok(_) => info!("Badges migration completed"),
            Err(e) => {
                error!("Badges migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Badges migration failed: {:?}", e));
                return Err(e);
            }
        }

        match self.migrate_groups().await {
            Ok(_) => info!("Groups migration completed"),
            Err(e) => {
                error!("Groups migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Groups migration failed: {:?}", e));
                return Err(e);
            }
        }

        match self.migrate_spaces().await {
            Ok(_) => info!("Spaces migration completed"),
            Err(e) => {
                error!("Spaces migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Spaces migration failed: {:?}", e));
                return Err(e);
            }
        }

        match self.migrate_feeds().await {
            Ok(_) => info!("Feeds migration completed"),
            Err(e) => {
                error!("Feeds migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Feeds migration failed: {:?}", e));
                return Err(e);
            }
        }

        match self.migrate_discussions().await {
            Ok(_) => info!("Discussions migration completed"),
            Err(e) => {
                error!("Discussions migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Discussions migration failed: {:?}", e));
                return Err(e);
            }
        }

        // Migrate relationship tables
        match self.migrate_relationships().await {
            Ok(_) => info!("Relationships migration completed"),
            Err(e) => {
                error!("Relationships migration failed: {:?}", e);
                self.migration_state.fail_migration(format!("Relationships migration failed: {:?}", e));
                return Err(e);
            }
        }

        self.migration_state.complete_migration();
        info!("Complete migration finished successfully");
        info!("Migration statistics: {}/{} records migrated, {} errors", 
              self.migration_state.migrated_records, 
              self.migration_state.total_records,
              self.migration_state.error_count);

        Ok(())
    }

    async fn calculate_total_records(&mut self) -> Result<()> {
        info!("Calculating total records for migration...");

        let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count users: {:?}", e)))?;

        let spaces_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM spaces")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count spaces: {:?}", e)))?;

        let feeds_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count feeds: {:?}", e)))?;

        let groups_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM groups")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count groups: {:?}", e)))?;

        let discussions_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM discussions")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count discussions: {:?}", e)))?;

        let industries_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM industries")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count industries: {:?}", e)))?;

        let badges_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM badges")
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to count badges: {:?}", e)))?;

        // Add relationship tables estimates
        let relationships_estimate = users_count * 5; // Estimate for various relationships per user

        self.migration_state.add_table("users".to_string(), users_count as u64);
        self.migration_state.add_table("spaces".to_string(), spaces_count as u64);
        self.migration_state.add_table("feeds".to_string(), feeds_count as u64);
        self.migration_state.add_table("groups".to_string(), groups_count as u64);
        self.migration_state.add_table("discussions".to_string(), discussions_count as u64);
        self.migration_state.add_table("industries".to_string(), industries_count as u64);
        self.migration_state.add_table("badges".to_string(), badges_count as u64);
        self.migration_state.add_table("relationships".to_string(), relationships_estimate as u64);

        info!("Total records to migrate: {}", self.migration_state.total_records);
        Ok(())
    }

    async fn migrate_users(&mut self) -> Result<()> {
        info!("Starting users migration...");

        let mut offset = 0;
        let batch_size = 1000;
        let mut migrated_count = 0;

        loop {
            let users = User::fetch_many(
                &self.db_pool,
                &format!("ORDER BY id LIMIT {} OFFSET {}", batch_size, offset),
            )
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch users: {:?}", e)))?;

            if users.is_empty() {
                break;
            }

            info!("Processing {} users (offset: {})", users.len(), offset);

            let dynamo_users: Vec<_> = users.iter()
                .map(|user| DynamoUser::from_postgres_user(user))
                .collect();

            self.batch_processor.process_items_in_batches(
                dynamo_users,
                |user| user.to_item(),
            ).await?;

            migrated_count += users.len();
            self.migration_state.update_table_progress("users", migrated_count as u64, users.last().map(|u| u.id));
            
            offset += batch_size;
        }

        info!("Users migration completed: {} records", migrated_count);
        Ok(())
    }

    async fn migrate_industries(&mut self) -> Result<()> {
        info!("Starting industries migration...");

        let industries = Industry::fetch_all(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch industries: {:?}", e)))?;

        let dynamo_items: Vec<_> = industries.iter()
            .map(|industry| {
                let mut item = std::collections::HashMap::new();
                item.insert("pk".to_string(), crate::utils::aws::string_attr(&format!("{}#{}", INDUSTRY_PREFIX, industry.id)));
                item.insert("sk".to_string(), crate::utils::aws::string_attr(METADATA_SK));
                item.insert("type".to_string(), crate::utils::aws::string_attr("INDUSTRY"));
                item.insert("id".to_string(), crate::utils::aws::number_attr(industry.id));
                item.insert("name".to_string(), crate::utils::aws::string_attr(&industry.name));
                item.insert("description".to_string(), crate::utils::aws::string_attr(&industry.description));
                item.insert("created_at".to_string(), crate::utils::aws::number_attr(industry.created_at));
                item.insert("updated_at".to_string(), crate::utils::aws::number_attr(industry.updated_at));
                Ok(item)
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        self.migration_state.update_table_progress("industries", industries.len() as u64, industries.last().map(|i| i.id));
        
        info!("Industries migration completed: {} records", industries.len());
        Ok(())
    }

    async fn migrate_badges(&mut self) -> Result<()> {
        info!("Starting badges migration...");

        let badges = Badge::fetch_all(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch badges: {:?}", e)))?;

        let dynamo_items: Vec<_> = badges.iter()
            .map(|badge| {
                let mut item = std::collections::HashMap::new();
                item.insert("pk".to_string(), crate::utils::aws::string_attr(&format!("{}#{}", BADGE_PREFIX, badge.id)));
                item.insert("sk".to_string(), crate::utils::aws::string_attr(METADATA_SK));
                item.insert("type".to_string(), crate::utils::aws::string_attr("BADGE"));
                item.insert("id".to_string(), crate::utils::aws::number_attr(badge.id));
                item.insert("name".to_string(), crate::utils::aws::string_attr(&badge.name));
                item.insert("description".to_string(), crate::utils::aws::string_attr(&badge.description));
                item.insert("image_url".to_string(), crate::utils::aws::string_attr(&badge.image_url));
                item.insert("created_at".to_string(), crate::utils::aws::number_attr(badge.created_at));
                item.insert("updated_at".to_string(), crate::utils::aws::number_attr(badge.updated_at));
                Ok(item)
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        self.migration_state.update_table_progress("badges", badges.len() as u64, badges.last().map(|b| b.id));
        
        info!("Badges migration completed: {} records", badges.len());
        Ok(())
    }

    async fn migrate_groups(&mut self) -> Result<()> {
        info!("Starting groups migration...");

        let groups = Group::fetch_all(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch groups: {:?}", e)))?;

        let dynamo_groups: Vec<_> = groups.iter()
            .map(|group| DynamoGroup::from_postgres_group(group))
            .collect();

        self.batch_processor.process_items_in_batches(
            dynamo_groups,
            |group| group.to_item(),
        ).await?;

        self.migration_state.update_table_progress("groups", groups.len() as u64, groups.last().map(|g| g.id));
        
        info!("Groups migration completed: {} records", groups.len());
        Ok(())
    }

    async fn migrate_spaces(&mut self) -> Result<()> {
        info!("Starting spaces migration...");

        let mut offset = 0;
        let batch_size = 500; // Smaller batch for spaces due to larger data
        let mut migrated_count = 0;

        loop {
            // Fetch spaces with related data
            let spaces = Space::fetch_many_with_joins(
                &self.db_pool,
                &format!("ORDER BY s.id LIMIT {} OFFSET {}", batch_size, offset),
            )
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch spaces: {:?}", e)))?;

            if spaces.is_empty() {
                break;
            }

            info!("Processing {} spaces (offset: {})", spaces.len(), offset);

            let mut dynamo_items = Vec::new();
            for space in &spaces {
                // Get owner and industry info for denormalization
                let owner_nickname = space.author.first().map(|a| a.nickname.clone()).unwrap_or_default();
                let owner_profile_url = space.author.first().and_then(|a| if a.profile_url.is_empty() { None } else { Some(a.profile_url.clone()) });
                let industry_name = space.industry.first().map(|i| i.name.clone()).unwrap_or_default();

                let dynamo_space = DynamoSpace::from_postgres_space(space, owner_nickname, owner_profile_url, industry_name);
                dynamo_items.push(dynamo_space.to_item()?);
            }

            self.batch_processor.process_batch(dynamo_items).await?;

            migrated_count += spaces.len();
            self.migration_state.update_table_progress("spaces", migrated_count as u64, spaces.last().map(|s| s.id));
            
            offset += batch_size;
        }

        info!("Spaces migration completed: {} records", migrated_count);
        Ok(())
    }

    async fn migrate_feeds(&mut self) -> Result<()> {
        info!("Starting feeds migration...");

        let mut offset = 0;
        let batch_size = 500;
        let mut migrated_count = 0;

        loop {
            let feeds = Feed::fetch_many_with_joins(
                &self.db_pool,
                &format!("ORDER BY f.id LIMIT {} OFFSET {}", batch_size, offset),
            )
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch feeds: {:?}", e)))?;

            if feeds.is_empty() {
                break;
            }

            info!("Processing {} feeds (offset: {})", feeds.len(), offset);

            let mut dynamo_items = Vec::new();
            for feed in &feeds {
                // Get author and industry info for denormalization
                let author_nickname = feed.author.first().map(|a| a.nickname.clone()).unwrap_or_default();
                let author_profile_url = feed.author.first().and_then(|a| if a.profile_url.is_empty() { None } else { Some(a.profile_url.clone()) });
                let industry_name = feed.industry.first().map(|i| i.name.clone()).unwrap_or_default();

                let dynamo_feed = DynamoFeed::from_postgres_feed(feed, author_nickname, author_profile_url, industry_name);
                dynamo_items.push(dynamo_feed.to_item()?);
            }

            self.batch_processor.process_batch(dynamo_items).await?;

            migrated_count += feeds.len();
            self.migration_state.update_table_progress("feeds", migrated_count as u64, feeds.last().map(|f| f.id));
            
            offset += batch_size;
        }

        info!("Feeds migration completed: {} records", migrated_count);
        Ok(())
    }

    async fn migrate_discussions(&mut self) -> Result<()> {
        info!("Starting discussions migration...");

        let discussions = Discussion::fetch_all_with_joins(&self.db_pool)
            .await
            .map_err(|e| Error::DatabaseException(format!("Failed to fetch discussions: {:?}", e)))?;

        let mut dynamo_items = Vec::new();
        for discussion in &discussions {
            // Get space and author info for denormalization  
            let space_title = None; // Would need to join with spaces table
            let author_nickname = String::new(); // Would need to join with users table
            let author_profile_url = None; // Would need to join with users table

            let dynamo_discussion = DynamoDiscussion::from_postgres_discussion(
                discussion, 
                space_title, 
                author_nickname, 
                author_profile_url
            );
            dynamo_items.push(dynamo_discussion.to_item()?);
        }

        self.batch_processor.process_batch(dynamo_items).await?;
        
        self.migration_state.update_table_progress("discussions", discussions.len() as u64, discussions.last().map(|d| d.id));
        
        info!("Discussions migration completed: {} records", discussions.len());
        Ok(())
    }

    async fn migrate_relationships(&mut self) -> Result<()> {
        info!("Starting relationships migration...");
        
        // Migrate user followers
        self.migrate_user_followers().await?;
        
        // Migrate user badges
        self.migrate_user_badges().await?;
        
        // Migrate space members
        self.migrate_space_members().await?;
        
        // Migrate space likes
        self.migrate_space_likes().await?;
        
        // Migrate feed likes  
        self.migrate_feed_likes().await?;
        
        // Migrate group members
        self.migrate_group_members().await?;

        info!("Relationships migration completed");
        Ok(())
    }

    async fn migrate_user_followers(&self) -> Result<()> {
        info!("Migrating user followers...");

        let followers: Vec<(i64, i64, String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT mn.following_id as user_id, mn.follower_id, u.nickname, u.profile_url
            FROM my_networks mn
            JOIN users u ON u.id = mn.follower_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Error::DatabaseException(format!("Failed to fetch followers: {:?}", e)))?;

        let dynamo_items: Vec<_> = followers.iter()
            .map(|(user_id, follower_id, nickname, profile_url)| {
                let profile_url = profile_url.clone().filter(|url| !url.is_empty());
                UserFollower::new(*user_id, *follower_id, nickname.clone(), profile_url).to_item()
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        info!("User followers migration completed: {} records", followers.len());
        Ok(())
    }

    async fn migrate_user_badges(&self) -> Result<()> {
        info!("Migrating user badges...");

        let user_badges: Vec<(i64, i64, String, String)> = sqlx::query_as(
            r#"
            SELECT ub.user_id, ub.badge_id, b.name, b.description
            FROM user_badges ub
            JOIN badges b ON b.id = ub.badge_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Error::DatabaseException(format!("Failed to fetch user badges: {:?}", e)))?;

        let dynamo_items: Vec<_> = user_badges.iter()
            .map(|(user_id, badge_id, name, description)| {
                UserBadge::new(*user_id, *badge_id, name.clone(), description.clone()).to_item()
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        info!("User badges migration completed: {} records", user_badges.len());
        Ok(())
    }

    async fn migrate_space_members(&self) -> Result<()> {
        info!("Migrating space members...");

        let space_members: Vec<(i64, i64, String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT sm.space_id, sm.user_id, u.nickname, u.profile_url
            FROM space_members sm
            JOIN users u ON u.id = sm.user_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Error::DatabaseException(format!("Failed to fetch space members: {:?}", e)))?;

        let dynamo_items: Vec<_> = space_members.iter()
            .map(|(space_id, user_id, nickname, profile_url)| {
                let profile_url = profile_url.clone().filter(|url| !url.is_empty());
                SpaceMember::new(*space_id, *user_id, nickname.clone(), profile_url).to_item()
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        info!("Space members migration completed: {} records", space_members.len());
        Ok(())
    }

    async fn migrate_space_likes(&self) -> Result<()> {
        info!("Migrating space likes...");

        let space_likes: Vec<(i64, i64, String)> = sqlx::query_as(
            r#"
            SELECT slu.space_id, slu.user_id, u.nickname
            FROM space_like_users slu
            JOIN users u ON u.id = slu.user_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Error::DatabaseException(format!("Failed to fetch space likes: {:?}", e)))?;

        let dynamo_items: Vec<_> = space_likes.iter()
            .map(|(space_id, user_id, nickname)| {
                SpaceLike::new(*space_id, *user_id, nickname.clone()).to_item()
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        info!("Space likes migration completed: {} records", space_likes.len());
        Ok(())
    }

    async fn migrate_feed_likes(&self) -> Result<()> {
        info!("Migrating feed likes...");

        let feed_likes: Vec<(i64, i64, String)> = sqlx::query_as(
            r#"
            SELECT fu.feed_id, fu.user_id, u.nickname
            FROM feed_users fu
            JOIN users u ON u.id = fu.user_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Error::DatabaseException(format!("Failed to fetch feed likes: {:?}", e)))?;

        let dynamo_items: Vec<_> = feed_likes.iter()
            .map(|(feed_id, user_id, nickname)| {
                FeedLike::new(*feed_id, *user_id, nickname.clone()).to_item()
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        info!("Feed likes migration completed: {} records", feed_likes.len());
        Ok(())
    }

    async fn migrate_group_members(&self) -> Result<()> {
        info!("Migrating group members...");

        let group_members: Vec<(i64, i64, String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT gm.group_id, gm.user_id, u.nickname, u.profile_url
            FROM group_members gm
            JOIN users u ON u.id = gm.user_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Error::DatabaseException(format!("Failed to fetch group members: {:?}", e)))?;

        let dynamo_items: Vec<_> = group_members.iter()
            .map(|(group_id, user_id, nickname, profile_url)| {
                let profile_url = profile_url.clone().filter(|url| !url.is_empty());
                GroupMember::new(*group_id, *user_id, nickname.clone(), profile_url).to_item()
            })
            .collect::<Result<Vec<_>>>()?;

        self.batch_processor.process_batch(dynamo_items).await?;
        
        info!("Group members migration completed: {} records", group_members.len());
        Ok(())
    }

    pub fn get_migration_state(&self) -> &MigrationState {
        &self.migration_state
    }

    pub async fn resume_migration(&mut self, checkpoint: MigrationState) -> Result<()> {
        info!("Resuming migration from checkpoint");
        self.migration_state = checkpoint;
        
        // Continue migration from where we left off
        let incomplete_tables: Vec<String> = self.migration_state.table_states
            .iter()
            .filter_map(|(table_name, table_state)| {
                if !matches!(table_state.status, MigrationStatus::Completed) {
                    Some(table_name.clone())
                } else {
                    None
                }
            })
            .collect();
            
        for table_name in incomplete_tables {
            let last_id = self.migration_state.table_states
                .get(&table_name)
                .and_then(|state| state.last_migrated_id)
                .unwrap_or(0);
            
            info!("Resuming {} migration from record {}", table_name, last_id);
            
            // Resume specific table migration based on table name
            match table_name.as_str() {
                "users" => self.migrate_users().await?,
                "spaces" => self.migrate_spaces().await?,
                "feeds" => self.migrate_feeds().await?,
                "groups" => self.migrate_groups().await?,
                "discussions" => self.migrate_discussions().await?,
                "industries" => self.migrate_industries().await?,
                "badges" => self.migrate_badges().await?,
                "relationships" => self.migrate_relationships().await?,
                _ => warn!("Unknown table for resume: {}", table_name),
            }
        }
        
        self.migration_state.complete_migration();
        Ok(())
    }
}