use leptos::prelude::*;

#[component]
pub(crate) fn Token(
    #[prop(into)] label: String,
    #[prop(into)] disabled: bool,
    on_remove: Callback<()>,
) -> impl IntoView {
    let aria_label = format!("Remove {label}");

    view! {
        <span class="ta-token">
            {label}
            <Show when=move || !disabled>
                <button
                    type="button"
                    class="ta-token-remove"
                    aria-label=aria_label.clone()
                    on:click=move |_| on_remove.run(())
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M18 6 6 18M6 6l12 12" />
                    </svg>
                </button>
            </Show>
        </span>
    }
}
