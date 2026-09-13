/// Anything that can be offered/selected by [`crate::Typeahead`].
pub trait TypeaheadOption: Clone + PartialEq + Send + Sync + 'static {
    /// Text shown in the input, the menu, and matched against the query.
    fn label(&self) -> String;

    /// Value used for the hidden `<input>` emitted when `name` is set, and
    /// as the list key. Defaults to [`TypeaheadOption::label`], mirroring
    /// upstream's `valueKey` fallback.
    fn value(&self) -> String {
        self.label()
    }
}

impl TypeaheadOption for String {
    fn label(&self) -> String {
        self.clone()
    }
}

impl TypeaheadOption for &'static str {
    fn label(&self) -> String {
        (*self).to_string()
    }
}
