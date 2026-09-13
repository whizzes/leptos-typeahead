//! A themeable, accessible typeahead/autocomplete component for
//! [Leptos](https://leptos.dev).
//!
//! ```ignore
//! use leptos::prelude::*;
//! use leptos_typeahead::Typeahead;
//!
//! #[component]
//! fn Demo() -> impl IntoView {
//!     let selected = RwSignal::new(Vec::<String>::new());
//!     view! {
//!         <Typeahead
//!             id="skills"
//!             options=vec!["Rust".to_string(), "TypeScript".to_string(), "Svelte".to_string()]
//!             selected=selected
//!         />
//!     }
//! }
//! ```

mod components;
mod controller;
mod option;
mod theme;
mod utils;

pub use components::Typeahead;
pub use controller::{TypeaheadConfig, TypeaheadController, TypeaheadItem};
pub use option::TypeaheadOption;
pub use theme::TypeaheadTheme;
