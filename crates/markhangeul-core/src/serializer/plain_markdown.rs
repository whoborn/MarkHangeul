use crate::ast::{MarkHangeulDocument, MarkHangeulToken};

pub fn export_plain_markdown(document: &MarkHangeulDocument) -> String {
    document
        .nodes
        .iter()
        .map(|node| match node {
            MarkHangeulToken::Text(text) => text.text.as_str(),
            MarkHangeulToken::Markhangeul(mark) => mark.text.as_str(),
        })
        .collect()
}
