use leptos::prelude::*;

#[component]
pub(crate) fn Menu(
    #[prop(into)] id: String,
    #[prop(into)] is_loading: Signal<bool>,
    #[prop(into)] is_empty: Signal<bool>,
    #[prop(into)] empty_label: String,
    #[prop(into)] search_text: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ul id=id role="listbox" class="ta-menu">
            {move || {
                if is_loading.get() {
                    view! {
                        <li class="ta-menu-message">
                            <svg
                                class="ta-icon ta-icon-spin"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 1 1-9-9" />
                            </svg>
                            {search_text.clone()}
                        </li>
                    }
                        .into_any()
                } else if is_empty.get() {
                    view! { <li class="ta-menu-message">{empty_label.clone()}</li> }.into_any()
                } else {
                    children().into_any()
                }
            }}
        </ul>
    }
}
