//! Chart theme — CSS variable extraction and color management.

use plycore::ChartTheme;

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
