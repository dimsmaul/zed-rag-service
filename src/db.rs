use anyhow::Result;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use crate::models::GuardrailConfig;

struct DocRecord {
    id: String,
    title: String,
    category: String,
    content: String,
    tags_json: String,
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS guardrails (
                workspace_id TEXT PRIMARY KEY,
                config_json TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                workspace_id TEXT NOT NULL,
                title TEXT NOT NULL,
                category TEXT NOT NULL,
                content TEXT NOT NULL,
                tags_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_docs_workspace ON documents(workspace_id);
            CREATE INDEX IF NOT EXISTS idx_docs_category ON documents(category);
            ",
        )?;
        Ok(())
    }

    pub fn get_guardrails(&self, workspace_id: &str) -> Result<Option<GuardrailConfig>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT config_json FROM guardrails WHERE workspace_id = ?1")?;
        let mut rows = stmt.query(params![workspace_id])?;

        if let Some(row) = rows.next()? {
            let json_str: String = row.get(0)?;
            let config: GuardrailConfig = serde_json::from_str(&json_str)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    pub fn save_guardrails(&self, config: &GuardrailConfig) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let json_str = serde_json::to_string(config)?;
        conn.execute(
            "INSERT INTO guardrails (workspace_id, config_json, updated_at) 
             VALUES (?1, ?2, ?3)
             ON CONFLICT(workspace_id) DO UPDATE SET config_json = ?2, updated_at = ?3",
            params![config.workspace_id, json_str, config.updated_at],
        )?;
        Ok(())
    }

    pub fn insert_document(
        &self,
        id: &str,
        workspace_id: &str,
        title: &str,
        category: &str,
        content: &str,
        tags: &[String],
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let tags_json = serde_json::to_string(tags)?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO documents (id, workspace_id, title, category, content, tags_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, workspace_id, title, category, content, tags_json, now],
        )?;
        Ok(())
    }

    pub fn search_documents(
        &self,
        workspace_id: &str,
        query: &str,
        category: Option<&str>,
        limit: usize,
    ) -> Result<Vec<(String, String, String, String, Vec<String>, f32)>> {
        let conn = self.conn.lock().unwrap();
        let query_lower = query.to_lowercase();
        let terms: Vec<&str> = query_lower.split_whitespace().collect();

        let mut records: Vec<DocRecord> = Vec::new();

        if let Some(cat) = category {
            let mut stmt = conn.prepare(
                "SELECT id, title, category, content, tags_json FROM documents WHERE workspace_id = ?1 AND category = ?2"
            )?;
            let mut rows = stmt.query(params![workspace_id, cat])?;
            while let Some(row) = rows.next()? {
                records.push(DocRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    category: row.get(2)?,
                    content: row.get(3)?,
                    tags_json: row.get(4)?,
                });
            }
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, title, category, content, tags_json FROM documents WHERE workspace_id = ?1"
            )?;
            let mut rows = stmt.query(params![workspace_id])?;
            while let Some(row) = rows.next()? {
                records.push(DocRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    category: row.get(2)?,
                    content: row.get(3)?,
                    tags_json: row.get(4)?,
                });
            }
        }

        let mut results = Vec::new();
        for doc in records {
            let tags: Vec<String> = serde_json::from_str(&doc.tags_json).unwrap_or_default();
            let text_to_search = format!("{} {} {}", doc.title.to_lowercase(), doc.category.to_lowercase(), doc.content.to_lowercase());
            let mut score = 0.0f32;
            for term in &terms {
                if doc.title.to_lowercase().contains(term) {
                    score += 2.0;
                }
                if text_to_search.contains(term) {
                    score += 1.0;
                }
            }

            if score > 0.0 || terms.is_empty() {
                results.push((doc.id, doc.title, doc.category, doc.content, tags, score));
            }
        }

        results.sort_by(|a, b| b.5.partial_cmp(&a.5).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        Ok(results)
    }
}
