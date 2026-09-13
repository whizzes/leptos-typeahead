use leptos::prelude::*;

use crate::option::TypeaheadOption;
use crate::utils::{default_filter_by, is_option_selected};

/// One rendered menu row: an existing option, or (with `allow_new`) the
/// free-text query offered as a new one — mirrors upstream's
/// `TypeaheadItem { option, isNew }`, as a Rust enum instead of a flag.
#[derive(Clone, PartialEq)]
pub enum TypeaheadItem<T: TypeaheadOption> {
    Existing(T),
    New(String),
}

/// Reactive inputs the view passes down — see `Typeahead.ts`'s `TypeaheadConfig`.
/// Every field is itself reactive (a `Signal` or `Callback`, both `Copy`),
/// so unlike upstream there is no need to re-push this on every render.
pub struct TypeaheadConfig<T: TypeaheadOption> {
    pub options: Signal<Vec<T>>,
    pub multiple: Signal<bool>,
    pub filter_by: Option<Callback<(T, String), bool>>,
    pub min_length: usize,
    /// `Some(constructor)` enables `allowNew`: builds the `T` inserted into
    /// `selected` when the free-text row is chosen.
    pub allow_new: Option<Callback<String, T>>,
    pub disabled: Signal<bool>,
    pub clear_button: bool,
    pub is_loading: Signal<bool>,
    pub on_search: Option<Callback<String>>,
    pub search_delay_ms: u64,
}

// Manual `Clone`/`Copy` impls: every field is `Copy` regardless of whether
// `T` itself is (a `Signal<Vec<T>>` or `Callback<(T, ..), ..>` is a `Copy`
// handle no matter what it's parameterized over), but `#[derive]` would add
// an incorrect `T: Copy` bound since `T` appears as a generic parameter.
impl<T: TypeaheadOption> Clone for TypeaheadConfig<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: TypeaheadOption> Copy for TypeaheadConfig<T> {}

/// Holds all `Typeahead` state as Leptos signals and the logic to mutate it.
/// The bundled `Typeahead` component wires its markup to these signals and
/// calls the methods in response to user interaction; it never mutates
/// state directly. Exposed publicly for headless use — build your own
/// markup around it the way upstream's `Typeahead` class allows.
pub struct TypeaheadController<T: TypeaheadOption> {
    pub query: RwSignal<String>,
    pub show_menu: RwSignal<bool>,
    pub active_index: RwSignal<i32>,
    pub selected: RwSignal<Vec<T>>,
    config: TypeaheadConfig<T>,
    search_generation: RwSignal<u64>,

    pub filtered_options: Memo<Vec<T>>,
    pub items: Memo<Vec<TypeaheadItem<T>>>,
    pub effective_active_index: Memo<i32>,
    pub is_empty: Memo<bool>,
    pub menu_open: Memo<bool>,
    pub show_clear: Memo<bool>,
}

// See the matching note on `TypeaheadConfig` above: every field here is
// `Copy` independent of `T`, so `#[derive(Clone, Copy)]`'s implicit
// `T: Copy` bound would be wrong.
impl<T: TypeaheadOption> Clone for TypeaheadController<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: TypeaheadOption> Copy for TypeaheadController<T> {}

impl<T: TypeaheadOption> TypeaheadController<T> {
    pub fn new(selected: RwSignal<Vec<T>>, config: TypeaheadConfig<T>) -> Self {
        let query = RwSignal::new(String::new());
        let show_menu = RwSignal::new(false);
        let active_index = RwSignal::new(-1i32);
        let search_generation = RwSignal::new(0u64);

        if !config.multiple.get_untracked()
            && let Some(first) = selected.get_untracked().first()
        {
            query.set(first.label());
        }

        let filtered_options = Memo::new(move |_| {
            let cfg = config;
            let q = query.get();
            if q.len() < cfg.min_length {
                return Vec::new();
            }
            let current_selected = selected.get();
            let mut list: Vec<T> = cfg
                .options
                .get()
                .into_iter()
                .filter(|option| match cfg.filter_by {
                    Some(filter) => filter.run((option.clone(), q.clone())),
                    None => default_filter_by(option, &q),
                })
                .collect();
            if cfg.multiple.get() {
                list.retain(|option| !is_option_selected(option, &current_selected));
            }
            list
        });

        let items = Memo::new(move |_| {
            let cfg = config;
            let q = query.get();
            let trimmed = q.trim();
            let options = filtered_options.get();
            let current_selected = selected.get();

            let trimmed_lower = trimmed.to_lowercase();
            let has_exact_match = options
                .iter()
                .any(|option| option.label().to_lowercase() == trimmed_lower);
            let query_matches_selected = current_selected
                .iter()
                .any(|option| option.label().to_lowercase() == trimmed_lower);
            let show_new = cfg.allow_new.is_some()
                && !trimmed.is_empty()
                && !has_exact_match
                && !query_matches_selected;

            let mut list: Vec<TypeaheadItem<T>> =
                options.into_iter().map(TypeaheadItem::Existing).collect();
            if show_new {
                list.push(TypeaheadItem::New(trimmed.to_string()));
            }
            list
        });

        let effective_active_index = Memo::new(move |_| {
            let index = active_index.get();
            if index == -1 && items.get().len() == 1 {
                0
            } else {
                index
            }
        });

        let is_empty = Memo::new(move |_| {
            let cfg = config;
            !cfg.is_loading.get() && query.get().len() >= cfg.min_length && items.get().is_empty()
        });

        let menu_open = Memo::new(move |_| {
            let cfg = config;
            show_menu.get() && !cfg.disabled.get() && query.get().len() >= cfg.min_length
        });

        let show_clear = Memo::new(move |_| {
            let cfg = config;
            if !cfg.clear_button || cfg.disabled.get() {
                return false;
            }
            if cfg.multiple.get() {
                !selected.get().is_empty()
            } else {
                !selected.get().is_empty() || !query.get().is_empty()
            }
        });

        Self {
            query,
            show_menu,
            active_index,
            selected,
            config,
            search_generation,
            filtered_options,
            items,
            effective_active_index,
            is_empty,
            menu_open,
            show_clear,
        }
    }

    /// While the query isn't being actively edited (menu closed) and
    /// selection is single, keep the input text following external
    /// `selected` writes — mirrors upstream's `syncSelected`. Call from an
    /// `Effect` watching `selected` for changes made outside the controller.
    pub fn sync_query_with_selection(&self) {
        if !self.config.multiple.get_untracked() && !self.show_menu.get_untracked() {
            let label = self.selected.get_untracked().first().map(|o| o.label());
            self.query.set(label.unwrap_or_default());
        }
    }

    pub fn open_menu(&self) {
        self.show_menu.set(true);
    }

    pub fn close_menu(&self) {
        self.show_menu.set(false);
        self.active_index.set(-1);
    }

    pub fn move_active_down(&self) {
        if !self.show_menu.get_untracked() {
            self.open_menu();
            return;
        }
        let last_index = self.items.get_untracked().len() as i32 - 1;
        self.active_index
            .update(|index| *index = (*index + 1).min(last_index));
    }

    pub fn move_active_up(&self) {
        self.active_index
            .update(|index| *index = (*index - 1).max(-1));
    }

    pub fn handle_input(&self, value: String) {
        self.query.set(value.clone());
        self.active_index.set(-1);
        self.show_menu.set(true);

        let Some(on_search) = self.config.on_search else {
            return;
        };
        let min_length = self.config.min_length;
        let generation = self.search_generation.get_untracked() + 1;
        self.search_generation.set(generation);
        let search_generation = self.search_generation;
        let delay = self.config.search_delay_ms;

        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(delay as u32).await;
            if search_generation.try_get_untracked() == Some(generation)
                && value.len() >= min_length
            {
                on_search.run(value);
            }
        });
    }

    /// Applies the selection to `selected` — call `on_change` yourself
    /// afterwards, the same way the bundled component does, since this only
    /// fires for user interaction, never for external `selected` writes.
    pub fn select_item(&self, item: TypeaheadItem<T>) {
        let option = match item {
            TypeaheadItem::Existing(option) => option,
            TypeaheadItem::New(text) => match self.config.allow_new {
                Some(constructor) => constructor.run(text),
                None => return,
            },
        };

        let label = option.label();
        if self.config.multiple.get_untracked() {
            self.selected.update(|current| current.push(option));
            self.query.set(String::new());
        } else {
            self.selected.set(vec![option]);
            self.query.set(label);
        }
        self.close_menu();
    }

    pub fn remove_token(&self, index: usize) {
        self.selected.update(|current| {
            if index < current.len() {
                current.remove(index);
            }
        });
    }

    pub fn clear(&self) {
        self.selected.update(|current| current.clear());
        self.query.set(String::new());
        self.close_menu();
    }
}
