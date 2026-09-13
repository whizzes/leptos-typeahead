use leptos::prelude::*;

use crate::utils::highlight_segments;

#[component]
pub(crate) fn Highlighter(
    #[prop(into)] text: String,
    #[prop(into)] search: String,
) -> impl IntoView {
    let segments = highlight_segments(&text, &search);

    segments
        .into_iter()
        .map(|segment| {
            if segment.matched {
                view! { <mark class="ta-highlight">{segment.text}</mark> }.into_any()
            } else {
                segment.text.into_any()
            }
        })
        .collect_view()
}
