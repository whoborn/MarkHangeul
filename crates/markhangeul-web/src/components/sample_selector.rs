use yew::prelude::*;

use crate::app::SampleDocument;

#[derive(Properties, PartialEq)]
pub struct SampleSelectorProps {
    pub samples: Vec<SampleDocument>,
    pub on_select: Callback<String>,
}

#[function_component(SampleSelector)]
pub fn sample_selector(props: &SampleSelectorProps) -> Html {
    html! {
        <div class="sample-bar" aria-label="samples">
            {for props.samples.iter().map(|sample| {
                let on_select = props.on_select.clone();
                let source = sample.source.clone();
                html! {
                    <button
                        class="sample-button"
                        type="button"
                        onclick={Callback::from(move |_| on_select.emit(source.clone()))}
                    >
                        <span aria-hidden="true">{"▶"}</span>
                        {sample.label}
                    </button>
                }
            })}
        </div>
    }
}
