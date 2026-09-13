use std::sync::Once;

use leptos::ev::{Event, KeyboardEvent};
use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use wasm_bindgen::JsCast;

use crate::controller::{TypeaheadConfig, TypeaheadController, TypeaheadItem};
use crate::option::TypeaheadOption;
use crate::theme::TypeaheadTheme;

use super::menu::Menu;
use super::menu_item::MenuItem;
use super::token::Token;

const STYLE_ELEMENT_ID: &str = "leptos-typeahead-styles";

/// Ships the component's CSS by injecting a single `<style>` element into
/// `<head>` the first time a `Typeahead` mounts — no Tailwind or external
/// stylesheet required by the consumer, matching upstream's promise.
fn inject_stylesheet() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        if document.get_element_by_id(STYLE_ELEMENT_ID).is_some() {
            return;
        }
        let Ok(style) = document.create_element("style") else {
            return;
        };
        style.set_id(STYLE_ELEMENT_ID);
        style.set_text_content(Some(include_str!("../style.css")));
        if let Some(head) = document.head() {
            let _ = head.append_child(&style);
        }
    });
}

/// A themeable, accessible typeahead/autocomplete combobox — a Leptos Typeahead.
/// Own the selection yourself and pass it in as `selected`, the same way you'd
/// use any other `RwSignal` with a Leptos form control:
///
/// ```ignore
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <Typeahead
///         id="skills"
///         options=vec!["Rust".to_string(), "TypeScript".to_string()]
///         selected=selected
///         on_change=move |values| leptos::logging::log!("{values:?}")
///     />
/// }
/// ```
#[component]
pub fn Typeahead<T>(
    /// Element id; also derives the input, listbox, and option element ids.
    #[prop(into)]
    id: String,
    /// Options to filter from. Accepts a plain `Vec<T>` or a `Signal<Vec<T>>`
    /// for async/reactive sources.
    #[prop(into, default = Signal::from(Vec::new()))]
    options: Signal<Vec<T>>,
    /// Owns the current selection — read and written directly, the way you
    /// already own any other `RwSignal` bound to a form control.
    selected: RwSignal<Vec<T>>,
    #[prop(into, default = Signal::from(false))] multiple: Signal<bool>,
    /// Overrides the default case/diacritic-insensitive substring match.
    #[prop(optional)]
    filter_by: Option<Callback<(T, String), bool>>,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(default = true)] clear_button: bool,
    #[prop(into, default = Signal::from(false))] is_loading: Signal<bool>,
    #[prop(default = 0)] min_length: usize,
    /// `Some(constructor)` enables free-text entries: builds the `T` to
    /// insert into `selected` from the typed query when its menu row is
    /// picked, mirroring upstream's `allowNew`/`customOption`.
    #[prop(optional)]
    allow_new: Option<Callback<String, T>>,
    #[prop(into, default = "New option: ".to_string())] new_selection_prefix: String,
    #[prop(into, default = "No results found.".to_string())] empty_label: String,
    #[prop(into, default = "Searching…".to_string())] search_text: String,
    /// When set, filtering is delegated to the consumer (async mode) —
    /// `options` is trusted as-is and this fires (debounced by
    /// `search_delay_ms`) as the query changes.
    #[prop(optional)]
    on_search: Option<Callback<String>>,
    #[prop(default = 200)] search_delay_ms: u64,
    /// Renders `<input type="hidden">`s so this participates in a native
    /// `<form>` like any other control.
    #[prop(into, optional)]
    name: Option<String>,
    /// Fires on selection/removal/clear caused by user interaction — not on
    /// external `selected` writes.
    #[prop(optional)]
    on_change: Option<Callback<Vec<T>>>,
    #[prop(optional)] theme: Option<TypeaheadTheme>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView
where
    T: TypeaheadOption,
{
    inject_stylesheet();

    let config = TypeaheadConfig {
        options,
        multiple,
        filter_by,
        min_length,
        allow_new,
        disabled,
        clear_button,
        is_loading,
        on_search,
        search_delay_ms,
    };
    let controller = TypeaheadController::new(selected, config);

    // Keeps the input text following `selected` while it isn't being
    // actively edited — e.g. a parent resetting the selection externally.
    Effect::new(move |_| {
        selected.track();
        controller.sync_query_with_selection();
    });

    let root_ref = NodeRef::<html::Div>::new();
    let input_ref = NodeRef::<html::Input>::new();

    let outside_click_handle = window_event_listener(leptos::ev::click, move |ev| {
        if !controller.menu_open.get_untracked() {
            return;
        }
        let target = ev.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let clicked_inside = root_ref
            .get_untracked()
            .zip(target.as_ref())
            .map(|(root, target)| root.contains(Some(target)))
            .unwrap_or(false);
        if !clicked_inside {
            controller.close_menu();
        }
    });
    on_cleanup(move || outside_click_handle.remove());

    let focus_input = move || {
        if let Some(el) = input_ref.get_untracked() {
            let _ = el.focus();
        }
    };
    let notify_change = move || {
        if let Some(on_change) = on_change {
            on_change.run(selected.get_untracked());
        }
    };

    let select_item = move |item: TypeaheadItem<T>| {
        controller.select_item(item);
        focus_input();
        notify_change();
    };
    let remove_token = move |index: usize| {
        controller.remove_token(index);
        focus_input();
        notify_change();
    };
    let clear = move |_: web_sys::MouseEvent| {
        controller.clear();
        focus_input();
        notify_change();
    };

    let on_input = move |ev: Event| controller.handle_input(event_target_value(&ev));

    let on_keydown = move |ev: KeyboardEvent| match ev.key().as_str() {
        "ArrowDown" => {
            ev.prevent_default();
            controller.move_active_down();
        }
        "ArrowUp" => {
            ev.prevent_default();
            controller.move_active_up();
        }
        "Enter" => {
            let items = controller.items.get_untracked();
            let index = controller.effective_active_index.get_untracked();
            if controller.menu_open.get_untracked() && index >= 0 && (index as usize) < items.len()
            {
                ev.prevent_default();
                select_item(items[index as usize].clone());
            }
        }
        "Escape" => {
            if controller.menu_open.get_untracked() {
                ev.prevent_default();
                controller.close_menu();
            }
        }
        "Backspace" if multiple.get_untracked() && controller.query.get_untracked().is_empty() => {
            let len = selected.get_untracked().len();
            if len > 0 {
                remove_token(len - 1);
            }
        }
        _ => {}
    };

    let listbox_id = format!("{id}-listbox");
    let root_style = theme.unwrap_or_default().to_style();
    let root_class = format!("ta-root {class}");

    let hidden_inputs = move || {
        let name = name.clone()?;
        Some(if multiple.get() {
            selected
                .get()
                .into_iter()
                .map(|option| {
                    let name = name.clone();
                    view! { <input type="hidden" name=name value=option.value() /> }
                })
                .collect_view()
                .into_any()
        } else {
            match selected.get().into_iter().next() {
                Some(option) => {
                    view! { <input type="hidden" name=name.clone() value=option.value() /> }
                        .into_any()
                }
                None => ().into_any(),
            }
        })
    };

    let multi_input_id = format!("{id}-input");
    let single_input_id = multi_input_id.clone();
    let multi_listbox_id = listbox_id.clone();
    let single_listbox_id = listbox_id.clone();
    let multi_item_id = id.clone();
    let single_item_id = id.clone();
    let multi_placeholder = placeholder.clone();
    // `StoredValue` (`Copy`, unlike `String`) is used here rather than more
    // `.clone()`s — the menu row list is itself passed as a component's
    // `ChildrenFn` (an `Arc<dyn Fn>`), and a plain `String` captured through
    // that extra layer of closure indirection can't satisfy `Fn` (only
    // `FnOnce`), even when cloned at every level.
    let menu_item_id = StoredValue::new(id.clone());
    let menu_new_selection_prefix = StoredValue::new(new_selection_prefix.clone());

    view! {
        <div node_ref=root_ref class=root_class style=root_style>
            {hidden_inputs}

            {move || {
                if multiple.get() {
                    let multi_input_id = multi_input_id.clone();
                    let multi_listbox_id = multi_listbox_id.clone();
                    let multi_item_id = multi_item_id.clone();
                    let multi_placeholder = multi_placeholder.clone();
                    view! {
                        <div class="ta-control ta-control-multi" class:ta-disabled=move || disabled.get()>
                            <For
                                each={move || {
                                    let items: Vec<(usize, T)> =
                                        selected.get().into_iter().enumerate().collect();
                                    items
                                }}
                                key={|(_, option): &(usize, T)| option.value()}
                                children={move |(index, option): (usize, T)| {
                                    view! {
                                        <Token
                                            label=option.label()
                                            disabled=disabled.get()
                                            on_remove=Callback::new(move |_| remove_token(index))
                                        />
                                    }
                                }}
                            />
                            <input
                                node_ref=input_ref
                                id=multi_input_id
                                role="combobox"
                                aria-autocomplete="list"
                                aria-expanded=move || controller.menu_open.get()
                                aria-controls=multi_listbox_id
                                aria-activedescendant=move || {
                                    let index = controller.effective_active_index.get();
                                    (index >= 0).then(|| format!("{multi_item_id}-item-{index}"))
                                }
                                disabled=move || disabled.get()
                                placeholder=move || {
                                    if selected.get().is_empty() { multi_placeholder.clone() } else { String::new() }
                                }
                                prop:value=move || controller.query.get()
                                on:input=on_input
                                on:keydown=on_keydown
                                on:focus=move |_| controller.open_menu()
                                autocomplete="off"
                                class="ta-input ta-input-multi"
                            />
                            {move || {
                                if is_loading.get() {
                                    view! {
                                        <svg
                                            class="ta-icon ta-icon-spin"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 1 1-9-9" />
                                        </svg>
                                    }
                                        .into_any()
                                } else if controller.show_clear.get() {
                                    view! {
                                        <button type="button" on:click=clear class="ta-clear-btn" aria-label="Clear">
                                            <svg class="ta-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M18 6 6 18M6 6l12 12" />
                                            </svg>
                                        </button>
                                    }
                                        .into_any()
                                } else {
                                    ().into_any()
                                }
                            }}
                        </div>
                    }
                        .into_any()
                } else {
                    let single_input_id = single_input_id.clone();
                    let single_listbox_id = single_listbox_id.clone();
                    let single_item_id = single_item_id.clone();
                    let placeholder = placeholder.clone();
                    view! {
                        <div class="ta-control-wrap">
                            <input
                                node_ref=input_ref
                                id=single_input_id
                                role="combobox"
                                aria-autocomplete="list"
                                aria-expanded=move || controller.menu_open.get()
                                aria-controls=single_listbox_id
                                aria-activedescendant=move || {
                                    let index = controller.effective_active_index.get();
                                    (index >= 0).then(|| format!("{single_item_id}-item-{index}"))
                                }
                                disabled=move || disabled.get()
                                placeholder=placeholder
                                prop:value=move || controller.query.get()
                                on:input=on_input
                                on:keydown=on_keydown
                                on:focus=move |_| controller.open_menu()
                                autocomplete="off"
                                class="ta-input"
                                class:ta-disabled=move || disabled.get()
                            />
                            {move || {
                                if is_loading.get() {
                                    view! {
                                        <svg
                                            class="ta-icon ta-icon-spin ta-icon-inline"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                        >
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 1 1-9-9" />
                                        </svg>
                                    }
                                        .into_any()
                                } else if controller.show_clear.get() {
                                    view! {
                                        <button
                                            type="button"
                                            on:click=clear
                                            class="ta-clear-btn ta-clear-btn-inline"
                                            aria-label="Clear"
                                        >
                                            <svg class="ta-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M18 6 6 18M6 6l12 12" />
                                            </svg>
                                        </button>
                                    }
                                        .into_any()
                                } else {
                                    ().into_any()
                                }
                            }}
                        </div>
                    }
                        .into_any()
                }
            }}

            {move || {
                if !controller.menu_open.get() {
                    return ().into_any();
                }
                view! {
                    <Menu
                        id=listbox_id.clone()
                        is_loading=is_loading
                        is_empty=controller.is_empty
                        empty_label=empty_label.clone()
                        search_text=search_text.clone()
                    >
                        {move || {
                            controller
                                .items
                                .get()
                                .into_iter()
                                .enumerate()
                                .map(move |(index, item)| {
                                    let (label, search, is_new) = match &item {
                                        TypeaheadItem::Existing(option) => {
                                            (option.label(), controller.query.get(), false)
                                        }
                                        TypeaheadItem::New(text) => {
                                            let prefix = menu_new_selection_prefix.get_value();
                                            (format!("{prefix}{text}"), String::new(), true)
                                        }
                                    };
                                    let active = controller.effective_active_index.get() == index as i32;
                                    view! {
                                        <MenuItem
                                            id=format!("{}-item-{index}", menu_item_id.get_value())
                                            label=label
                                            search=search
                                            active=active
                                            is_new=is_new
                                            on_select=Callback::new(move |_| select_item(item.clone()))
                                        />
                                    }
                                })
                                .collect_view()
                        }}
                    </Menu>
                }
                    .into_any()
            }}
        </div>
    }
}
