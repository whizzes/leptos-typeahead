# leptos-typeahead

A themeable, accessible typeahead/autocomplete component for [Leptos](https://leptos.dev).

```rust
use leptos::prelude::*;
use leptos_typeahead::Typeahead;

#[component]
fn Demo() -> impl IntoView {
    let selected = RwSignal::new(Vec::<String>::new());
    view! {
        <Typeahead
            id="skills"
            options=vec!["Rust".to_string(), "TypeScript".to_string(), "Svelte".to_string()]
            selected=selected
        />
    }
}
```

## Custom option types

`TypeaheadOption` replaces upstream's runtime `labelKey`/`valueKey`:

```rust
use leptos_typeahead::TypeaheadOption;

#[derive(Clone, PartialEq)]
struct Country {
    name: String,
    code: String,
}

impl TypeaheadOption for Country {
    fn label(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> String {
        self.code.clone()
    }
}
```

## Features

- Single and multi-select (`multiple`)
- Free-text entries via `allow_new` (a `Callback<String, T>` constructor)
- Async/delegated filtering via `on_search` (debounced by `search_delay_ms`)
- Full keyboard support (arrow keys, `Enter`, `Escape`, `Backspace` to pop the last token)
- ARIA combobox semantics (`role="combobox"`, `aria-expanded`, `aria-activedescendant`, ...)
- Hidden `<input>`s via `name` so it participates in a native `<form>`
- Themeable through `TypeaheadTheme` (CSS custom properties, no rebuild required)
- Headless escape hatch: `TypeaheadController` exposes the same reactive state/logic
  without the bundled markup, for building custom UI on top of it

## License

MIT
