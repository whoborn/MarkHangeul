//! Browser API using exactly the same parser and renderer as the editor.
use markhangeul_core::{export_json_ast, export_plain_markdown, parse_markhangeul};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = renderMarkdown)]
pub fn render_markdown(source: &str) -> String {
    markhangeul_render::render_preview_html(&parse_markhangeul(source), None)
        .replace("role=\"button\" tabindex=\"0\"", "role=\"img\"")
}
#[wasm_bindgen(js_name = parse)]
pub fn parse(source: &str) -> String {
    export_json_ast(&parse_markhangeul(source))
}
#[wasm_bindgen(js_name = plainMarkdown)]
pub fn plain_markdown(source: &str) -> String {
    export_plain_markdown(&parse_markhangeul(source))
}
#[wasm_bindgen(js_name = exportHtml)]
pub fn export_html(source: &str) -> String {
    markhangeul_render::export_html(&parse_markhangeul(source))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_api_preserves_markdown_and_escapes_html() {
        let source = "# 공개 예제\n\n**마{T2}**\n\n<script>alert(1)</script>";
        let html = render_markdown(source);
        assert!(html.contains("<h1>공개 예제</h1>"));
        assert!(html.contains("<strong><span"));
        assert!(!html.contains("<script>"));
        assert!(!html.contains("tabindex=\"0\""));
        assert!(html.contains("role=\"img\""));
        assert!(plain_markdown(source).contains("**마**"));
        assert!(parse("마{toneContour=9}").contains("INVALID"));
        assert!(export_html(source).starts_with("<!doctype html>"));
    }
}
