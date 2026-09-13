use unicode_normalization::UnicodeNormalization;

use crate::option::TypeaheadOption;

/// Case/diacritic-insensitive substring match, ported from upstream's
/// `defaultFilterBy`.
pub fn default_filter_by<T: TypeaheadOption>(option: &T, text: &str) -> bool {
    if text.is_empty() {
        return true;
    }
    normalize(&option.label()).contains(&normalize(text))
}

fn normalize(value: &str) -> String {
    value
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .collect::<String>()
        .to_lowercase()
}

fn is_combining_mark(c: char) -> bool {
    unicode_normalization::char::is_combining_mark(c)
}

pub fn is_option_selected<T: TypeaheadOption>(option: &T, selected: &[T]) -> bool {
    selected.contains(option)
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchSegment {
    pub text: String,
    pub matched: bool,
}

/// Splits `label` into matched/unmatched segments against every occurrence
/// of `search`. Case-insensitive only (no diacritic stripping), matching
/// upstream's `highlightSegments` — a diacritic-insensitive `default_filter_by`
/// match may occasionally go unhighlighted here without failing to match
/// there; that's an acceptable, non-crashing degradation.
///
/// Walks `char`s rather than byte slices so a case-fold that changes byte
/// length (e.g. Turkish `İ`) can't produce a misaligned or panicking slice.
pub fn highlight_segments(label: &str, search: &str) -> Vec<MatchSegment> {
    if search.is_empty() {
        return vec![MatchSegment {
            text: label.to_string(),
            matched: false,
        }];
    }

    let needle_lower = search.to_lowercase();
    let needle_len = needle_lower.chars().count();
    let chars: Vec<char> = label.chars().collect();

    let mut segments = Vec::new();
    let mut run_start = 0usize;
    let mut cursor = 0usize;

    while cursor < chars.len() {
        let is_match = cursor + needle_len <= chars.len()
            && chars[cursor..cursor + needle_len]
                .iter()
                .collect::<String>()
                .to_lowercase()
                == needle_lower;

        if is_match {
            if cursor > run_start {
                segments.push(MatchSegment {
                    text: chars[run_start..cursor].iter().collect(),
                    matched: false,
                });
            }
            segments.push(MatchSegment {
                text: chars[cursor..cursor + needle_len].iter().collect(),
                matched: true,
            });
            cursor += needle_len;
            run_start = cursor;
        } else {
            cursor += 1;
        }
    }

    if run_start < chars.len() {
        segments.push(MatchSegment {
            text: chars[run_start..].iter().collect(),
            matched: false,
        });
    }

    segments
}
