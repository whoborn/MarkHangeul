use crate::ast::MarkHangeulDocument;

pub fn export_json_ast(document: &MarkHangeulDocument) -> String {
    serde_json::to_string_pretty(document)
        .unwrap_or_else(|error| format!("{{\"error\":\"failed to serialize AST: {error}\"}}"))
}
