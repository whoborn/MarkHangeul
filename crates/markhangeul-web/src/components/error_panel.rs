use markhangeul_core::ParseError;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorPanelProps {
    pub errors: Vec<ParseError>,
}

#[function_component(ErrorPanel)]
pub fn error_panel(props: &ErrorPanelProps) -> Html {
    html! {
        <section class="panel error-panel" aria-labelledby="error-title" data-empty={props.errors.is_empty().to_string()}>
            <div class="panel-header">
                <div class="panel-title">
                    <span class="panel-icon">{"!"}</span>
                    <h2 id="error-title">{"Errors"}</h2>
                </div>
                <span class="counter">{props.errors.len()}</span>
            </div>
            <div class="error-list">
                if props.errors.is_empty() {
                    <div class="empty-state">{"No errors"}</div>
                }
                {for props.errors.iter().map(|error| html! {
                    <div class="error-row">
                        <strong>{error.code.clone()}</strong>
                        <span>{error.message.clone()}</span>
                        <code>{format!("@{}", error.index)}</code>
                    </div>
                })}
            </div>
        </section>
    }
}
