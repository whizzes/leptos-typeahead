use leptos::prelude::*;
use leptos::web_sys::MouseEvent;

use super::highlighter::Highlighter;

#[component]
pub(crate) fn MenuItem(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into)] search: String,
    #[prop(into)] active: bool,
    #[prop(into)] is_new: bool,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <li
            id=id
            role="option"
            aria-selected=active
            class="ta-menu-item"
            class:ta-menu-item-active=active
            // Keyboard selection is handled by the combobox input's `Enter` key,
            // not here; `mousedown` is prevented so the input never loses focus
            // before `click` fires the selection.
            on:mousedown=|ev: MouseEvent| ev.prevent_default()
            on:click=move |_| on_select.run(())
        >
            {if is_new {
                view! { <span class="ta-menu-item-new">{label}</span> }.into_any()
            } else {
                view! { <Highlighter text=label search=search /> }.into_any()
            }}
        </li>
    }
}
