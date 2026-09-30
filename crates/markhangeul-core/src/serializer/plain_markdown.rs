use crate::ast::{MarkHangeulDocument, MarkHangeulToken};

pub fn export_plain_markdown(document: &MarkHangeulDocument) -> String {
    document
        .nodes
        .iter()
        .map(|node| match node {
            MarkHangeulToken::Text(text) => text.text.as_str(),
            MarkHangeulToken::Markhangeul(mark) => {
                if mark.errors.is_empty() {
                    mark.text.as_str()
                } else {
                    &document.source[mark.start..mark.end]
                }
            }
        })
        .collect()
}
