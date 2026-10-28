#[cfg(test)]
mod tests {
    use zed_rag_service::db::Database;
    use zed_rag_service::models::{GuardrailConfig, PathRule};

    #[test]
    fn test_guardrails_crud() {
        let db = Database::in_memory().expect("Failed to create in-memory db");
        let workspace_id = "test-repo";

        let config = GuardrailConfig {
            workspace_id: workspace_id.to_string(),
            version: "1.0".to_string(),
            deny_paths: vec!["common-grpc/**".to_string()],
            path_rules: vec![PathRule {
                pattern: "common-grpc/**".to_string(),
                action: "block".to_string(),
                reason: "Direct edits forbidden".to_string(),
            }],
            protected_branches: vec!["main".to_string()],
            allowed_commands: vec!["cargo test".to_string()],
            updated_at: "2026-10-28T00:00:00Z".to_string(),
        };

        db.save_guardrails(&config).expect("Failed to save guardrails");
        let fetched = db.get_guardrails(workspace_id).expect("Failed to fetch");
        assert!(fetched.is_some());
        let fetched_config = fetched.unwrap();
        assert_eq!(fetched_config.deny_paths.len(), 1);
        assert_eq!(fetched_config.deny_paths[0], "common-grpc/**");
    }

    #[test]
    fn test_document_ingestion_and_search() {
        let db = Database::in_memory().expect("Failed to create in-memory db");
        let workspace_id = "test-repo";

        db.insert_document(
            "doc-1",
            workspace_id,
            "Controller Development Guide",
            "controller",
            "Controllers handle incoming HTTP JSON payloads and invoke service layer.",
            &["controller".to_string(), "http".to_string()],
        )
        .expect("Failed to insert doc");

        let matches = db
            .search_documents(workspace_id, "HTTP JSON controller", None, 5)
            .expect("Search failed");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].1, "Controller Development Guide");
    }
}
