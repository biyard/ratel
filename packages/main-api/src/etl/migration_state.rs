use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationState {
    pub migration_id: String,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub status: MigrationStatus,
    pub table_states: HashMap<String, TableMigrationState>,
    pub total_records: u64,
    pub migrated_records: u64,
    pub error_count: u64,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMigrationState {
    pub table_name: String,
    pub total_records: u64,
    pub migrated_records: u64,
    pub last_migrated_id: Option<i64>,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub status: MigrationStatus,
    pub error_count: u64,
    pub last_error: Option<String>,
}

impl MigrationState {
    pub fn new(migration_id: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            migration_id,
            started_at: now,
            completed_at: None,
            status: MigrationStatus::Pending,
            table_states: HashMap::new(),
            total_records: 0,
            migrated_records: 0,
            error_count: 0,
            last_error: None,
        }
    }

    pub fn add_table(&mut self, table_name: String, total_records: u64) {
        let now = chrono::Utc::now().timestamp();
        self.table_states.insert(
            table_name.clone(),
            TableMigrationState {
                table_name,
                total_records,
                migrated_records: 0,
                last_migrated_id: None,
                started_at: now,
                completed_at: None,
                status: MigrationStatus::Pending,
                error_count: 0,
                last_error: None,
            },
        );
        self.total_records += total_records;
    }

    pub fn start_migration(&mut self) {
        self.status = MigrationStatus::InProgress;
    }

    pub fn complete_migration(&mut self) {
        self.status = MigrationStatus::Completed;
        self.completed_at = Some(chrono::Utc::now().timestamp());
    }

    pub fn fail_migration(&mut self, error: String) {
        self.status = MigrationStatus::Failed;
        self.last_error = Some(error);
        self.completed_at = Some(chrono::Utc::now().timestamp());
    }

    pub fn update_table_progress(&mut self, table_name: &str, migrated_count: u64, last_id: Option<i64>) {
        if let Some(table_state) = self.table_states.get_mut(table_name) {
            let prev_migrated = table_state.migrated_records;
            table_state.migrated_records = migrated_count;
            table_state.last_migrated_id = last_id;
            
            // Update global progress
            self.migrated_records = self.migrated_records - prev_migrated + migrated_count;
            
            if migrated_count >= table_state.total_records {
                table_state.status = MigrationStatus::Completed;
                table_state.completed_at = Some(chrono::Utc::now().timestamp());
            } else {
                table_state.status = MigrationStatus::InProgress;
            }
        }
    }

    pub fn record_table_error(&mut self, table_name: &str, error: String) {
        if let Some(table_state) = self.table_states.get_mut(table_name) {
            table_state.error_count += 1;
            table_state.last_error = Some(error.clone());
        }
        self.error_count += 1;
        self.last_error = Some(error);
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.total_records == 0 {
            return 0.0;
        }
        (self.migrated_records as f64 / self.total_records as f64) * 100.0
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, MigrationStatus::Completed)
    }

    pub fn has_failed(&self) -> bool {
        matches!(self.status, MigrationStatus::Failed)
    }

    pub fn get_incomplete_tables(&self) -> Vec<String> {
        if self.table_states.is_empty() {
            // If no tables are registered, return default list
            vec![
                "users".to_string(),
                "spaces".to_string(),
                "feeds".to_string(),
                "discussions".to_string(),
                "groups".to_string(),
                "followers".to_string(),
                "space_members".to_string(),
                "feed_bookmark_users".to_string(),
                "discussion_comments".to_string(),
            ]
        } else {
            // Return tables that are not completed
            self.table_states
                .values()
                .filter(|state| !matches!(state.status, MigrationStatus::Completed))
                .map(|state| state.table_name.clone())
                .collect()
        }
    }

    pub fn mark_table_complete(&mut self, table_name: &str) {
        if let Some(table_state) = self.table_states.get_mut(table_name) {
            table_state.status = MigrationStatus::Completed;
            table_state.completed_at = Some(chrono::Utc::now().timestamp());
        } else {
            // If table wasn't registered, add it as completed
            let now = chrono::Utc::now().timestamp();
            self.table_states.insert(
                table_name.to_string(),
                TableMigrationState {
                    table_name: table_name.to_string(),
                    total_records: 0,
                    migrated_records: 0,
                    last_migrated_id: None,
                    started_at: now,
                    completed_at: Some(now),
                    status: MigrationStatus::Completed,
                    error_count: 0,
                    last_error: None,
                },
            );
        }
    }
}