/// Every visual aspect of [`crate::Typeahead`] is themeable through this
/// struct, applied as CSS custom properties on the component's root
/// element — no Tailwind or external stylesheet required by the consumer.
///
/// Port of `theme.ts`'s `TypeaheadTheme`. Rust has no `Partial<T>`, so
/// override only what you need with struct update syntax:
///
/// ```
/// use leptos_typeahead::TypeaheadTheme;
///
/// let theme = TypeaheadTheme {
///     accent_color: "#059669".into(),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TypeaheadTheme {
    pub text_color: String,
    pub muted_text_color: String,
    pub placeholder_color: String,
    pub border_color: String,
    pub border_radius: String,
    pub accent_color: String,
    pub active_background: String,
    pub active_text_color: String,
    pub input_background: String,
    pub menu_background: String,
    pub menu_shadow: String,
    pub token_background: String,
    pub token_text_color: String,
    pub token_border_color: String,
    pub highlight_background: String,
    pub font_family: String,
    pub font_size: String,
}

impl Default for TypeaheadTheme {
    fn default() -> Self {
        Self {
            text_color: "#111827".into(),
            muted_text_color: "#6b7280".into(),
            placeholder_color: "#9ca3af".into(),
            border_color: "#d1d5db".into(),
            border_radius: "0.375rem".into(),
            accent_color: "#4f46e5".into(),
            active_background: "#4f46e5".into(),
            active_text_color: "#ffffff".into(),
            input_background: "#ffffff".into(),
            menu_background: "#ffffff".into(),
            menu_shadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -4px rgba(0, 0, 0, 0.1)"
                .into(),
            token_background: "#eef2ff".into(),
            token_text_color: "#4338ca".into(),
            token_border_color: "#c7d2fe".into(),
            highlight_background: "#fef08a".into(),
            font_family: "sans-serif".into(),
            font_size: "1rem".into(),
        }
    }
}

impl TypeaheadTheme {
    /// Serializes as an inline `style` string of `--ta-*` custom properties,
    /// mirroring `theme.ts`'s `themeToCssVars`.
    pub fn to_style(&self) -> String {
        format!(
            "--ta-text: {}; --ta-muted-text: {}; --ta-placeholder: {}; --ta-border: {}; \
             --ta-radius: {}; --ta-accent: {}; --ta-active-bg: {}; --ta-active-text: {}; \
             --ta-input-bg: {}; --ta-menu-bg: {}; --ta-menu-shadow: {}; --ta-token-bg: {}; \
             --ta-token-text: {}; --ta-token-border: {}; --ta-highlight-bg: {}; \
             --ta-font-family: {}; --ta-font-size: {}",
            self.text_color,
            self.muted_text_color,
            self.placeholder_color,
            self.border_color,
            self.border_radius,
            self.accent_color,
            self.active_background,
            self.active_text_color,
            self.input_background,
            self.menu_background,
            self.menu_shadow,
            self.token_background,
            self.token_text_color,
            self.token_border_color,
            self.highlight_background,
            self.font_family,
            self.font_size,
        )
    }
}
