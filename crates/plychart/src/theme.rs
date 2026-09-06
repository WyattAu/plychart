//! Chart theme — CSS variable extraction, color management, theme JSON parsing.

use crate::color::parse_css_color;
use plycore::ChartTheme;

/// Background RGB of [`ChartTheme::light`] (`#ffffff`).
const LIGHT_BG: (u8, u8, u8) = (255, 255, 255);
/// Background RGB of [`ChartTheme::midnight`] (`#0c0c0c`).
const MIDNIGHT_BG: (u8, u8, u8) = (12, 12, 12);
/// Accent RGB of [`ChartTheme::midnight`] (`#00e5ff`).
const MIDNIGHT_ACCENT: (u8, u8, u8) = (0, 229, 255);

/// Extract theme colors from CSS variables at runtime.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn theme_from_css() -> ChartTheme {
    ChartTheme::dark() // Fallback; CSS variables handled at the application layer
}

/// Get the appropriate theme for the current context.
#[must_use]
pub fn get_theme(is_dark: bool) -> ChartTheme {
    if is_dark {
        ChartTheme::dark()
    } else {
        ChartTheme::light()
    }
}

/// Parse a theme JSON string (`{"bg", "accent", "up", "down"}`) into a [`ChartTheme`].
///
/// Color values accept any CSS syntax supported by
/// [`parse_css_color`](crate::color::parse_css_color): hex (`#rgb`, `#rrggbb`,
/// `#rrggbbaa`), `rgb()`/`rgba()`, and basic named colors. Built-in palettes
/// are recognized by their actual color values, so equivalent spellings select
/// the same theme — `{"bg": "white"}`, `{"bg": "#fff"}` and
/// `{"bg": "rgb(255,255,255)"}` all select the light theme, while a
/// `#0c0c0c` background or `#00e5ff` accent selects midnight.
///
/// Empty input, invalid JSON, or unrecognized colors fall back to the dark theme.
#[must_use]
pub fn theme_from_json(theme_json: &str) -> ChartTheme {
    // ChartTheme has &'static str fields, so we deserialize into an owned
    // helper struct and match parsed colors against the built-in palettes.
    // `up`/`down` are part of the wire schema but don't drive palette selection.
    #[derive(serde::Deserialize, Default)]
    #[allow(dead_code)]
    struct OwnedTheme {
        bg: Option<String>,
        accent: Option<String>,
        up: Option<String>,
        down: Option<String>,
    }

    let mut t = ChartTheme::dark();
    if theme_json.is_empty() {
        return t;
    }
    let Ok(owned) = serde_json::from_str::<OwnedTheme>(theme_json) else {
        return t;
    };

    if let Some(bg) = owned.bg.as_deref().and_then(parse_css_color) {
        match bg.rgb() {
            LIGHT_BG => t = ChartTheme::light(),
            MIDNIGHT_BG => t = ChartTheme::midnight(),
            _ => {}
        }
    }
    if let Some(accent) = owned.accent.as_deref().and_then(parse_css_color)
        && accent.rgb() == MIDNIGHT_ACCENT
    {
        t = ChartTheme::midnight();
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_defaults_to_dark() {
        assert_eq!(theme_from_json("").bg, ChartTheme::dark().bg);
        assert_eq!(theme_from_json("   ").bg, ChartTheme::dark().bg);
    }

    #[test]
    fn invalid_json_defaults_to_dark() {
        assert_eq!(theme_from_json("not json").bg, ChartTheme::dark().bg);
        assert_eq!(theme_from_json("[1,2,3]").bg, ChartTheme::dark().bg);
        assert_eq!(theme_from_json("{\"bg\": 123}").bg, ChartTheme::dark().bg);
    }

    #[test]
    fn empty_object_defaults_to_dark() {
        assert_eq!(theme_from_json("{}").bg, ChartTheme::dark().bg);
    }

    #[test]
    fn unknown_bg_defaults_to_dark() {
        assert_eq!(
            theme_from_json("{\"bg\": \"#123456\"}").bg,
            ChartTheme::dark().bg
        );
        assert_eq!(
            theme_from_json("{\"bg\": \"#0a0a0a\"}").bg,
            ChartTheme::dark().bg
        );
    }

    #[test]
    fn light_theme_exact_hex() {
        let t = theme_from_json("{\"bg\": \"#ffffff\"}");
        assert_eq!(t.bg, ChartTheme::light().bg);
        assert_eq!(t.text, ChartTheme::light().text);
    }

    #[test]
    fn light_theme_equivalent_spellings() {
        for json in [
            "{\"bg\": \"white\"}",
            "{\"bg\": \"#fff\"}",
            "{\"bg\": \"#ffffffff\"}",
            "{\"bg\": \"rgb(255,255,255)\"}",
            "{\"bg\": \"rgb(255, 255, 255)\"}",
            "{\"bg\": \"rgb(100%, 100%, 100%)\"}",
            "{\"bg\": \"WHITE\"}",
            "{\"bg\": \"  #ffffff  \"}",
        ] {
            let t = theme_from_json(json);
            assert_eq!(t.bg, ChartTheme::light().bg, "failed for {json:?}");
        }
    }

    #[test]
    fn midnight_theme_bg() {
        for json in [
            "{\"bg\": \"#0c0c0c\"}",
            "{\"bg\": \"rgb(12,12,12)\"}",
            "{\"bg\": \"#0c0c0c\", \"accent\": \"#00e5ff\"}",
        ] {
            let t = theme_from_json(json);
            assert_eq!(t.bg, ChartTheme::midnight().bg, "failed for {json:?}");
            assert_eq!(t.accent, ChartTheme::midnight().accent);
        }
    }

    #[test]
    fn midnight_theme_accent_only() {
        for json in [
            "{\"accent\": \"#00e5ff\"}",
            "{\"accent\": \"rgb(0,229,255)\"}",
            "{\"accent\": \"rgb(0 229 255)\"}",
        ] {
            let t = theme_from_json(json);
            // Accent match selects the full midnight palette (original precedence).
            assert_eq!(
                t.accent,
                ChartTheme::midnight().accent,
                "failed for {json:?}"
            );
            assert_eq!(t.bg, ChartTheme::midnight().bg, "failed for {json:?}");
        }
    }

    #[test]
    fn accent_overrides_bg_selection() {
        // Preserves the original precedence: accent is applied after bg.
        let t = theme_from_json("{\"bg\": \"#ffffff\", \"accent\": \"#00e5ff\"}");
        assert_eq!(t.accent, ChartTheme::midnight().accent);
        assert_eq!(t.bg, ChartTheme::midnight().bg);
    }

    #[test]
    fn unparseable_color_values_fall_back_to_dark() {
        assert_eq!(
            theme_from_json("{\"bg\": \"notacolor\"}").bg,
            ChartTheme::dark().bg
        );
        assert_eq!(
            theme_from_json("{\"accent\": \"#zzzzzz\"}").bg,
            ChartTheme::dark().bg
        );
    }

    #[test]
    fn up_down_fields_are_part_of_schema() {
        let t = theme_from_json("{\"up\": \"#4ade80\", \"down\": \"#f87171\"}");
        assert_eq!(t.bg, ChartTheme::dark().bg);
        let t = theme_from_json("{\"bg\": \"white\", \"up\": \"green\", \"down\": \"red\"}");
        assert_eq!(t.bg, ChartTheme::light().bg);
    }

    #[test]
    fn extra_fields_are_ignored() {
        let t = theme_from_json("{\"bg\": \"#ffffff\", \"unknown\": true}");
        assert_eq!(t.bg, ChartTheme::light().bg);
    }

    #[test]
    fn all_builtin_theme_colors_reparse_to_themselves() {
        // Every color in the three built-in palettes must round-trip through
        // the CSS parser as an exact match, so theme_from_json can never
        // misclassify a palette value.
        for theme in [
            ChartTheme::dark(),
            ChartTheme::light(),
            ChartTheme::midnight(),
        ] {
            for c in [
                theme.bg,
                theme.text,
                theme.text_muted,
                theme.grid,
                theme.up,
                theme.down,
                theme.accent,
                theme.volume,
                theme.crosshair,
            ] {
                let parsed = parse_css_color(c).unwrap_or_else(|| panic!("{c} did not parse"));
                assert_eq!(
                    (parsed.r, parsed.g, parsed.b),
                    parse_css_color(c).unwrap().rgb()
                );
            }
        }
    }
}
