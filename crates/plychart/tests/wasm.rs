//! Browser tests for the WASM entry-point surface (`plychart::wasm`).
//!
//! Run with: `wasm-pack test --headless --chrome crates/plychart`
//! (CI-verified; see the `wasm-test` job in .github/workflows/ci.yml).
//!
//! Each test mounts a real `<canvas>` via web-sys, so the Canvas2D render
//! path is exercised end-to-end in headless Chromium.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Append a canvas with the given id/size to document.body and return it.
fn mount_canvas(id: &str, width: u32, height: u32) -> web_sys::HtmlCanvasElement {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let canvas = document
        .create_element("canvas")
        .expect("create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("not a canvas");
    canvas.set_id(id);
    canvas.set_width(width);
    canvas.set_height(height);
    document
        .body()
        .expect("no body")
        .append_child(&canvas)
        .expect("append canvas");
    canvas
}

fn remove_canvas(id: &str) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(el) = document.get_element_by_id(id) {
            if let Some(body) = document.body() {
                let _ = body.remove_child(&el);
            }
        }
    }
}

const CANDLE: &str =
    r#"{"time":1.0,"open":100.0,"high":105.0,"low":99.0,"close":102.0,"volume":500.0}"#;
const CANDLES_2: &str = r#"[{"time":1.0,"open":100.0,"high":105.0,"low":99.0,"close":102.0,"volume":500.0},
                            {"time":2.0,"open":102.0,"high":108.0,"low":101.0,"close":107.0,"volume":600.0}]"#;

// ---------------------------------------------------------------------------
// Theme parse -> canvas color application
// ---------------------------------------------------------------------------

/// Every theme spelling must be accepted and applied to a live canvas without
/// error. Covers the CSS syntaxes theme_from_json supports.
#[wasm_bindgen_test]
fn theme_json_spellings_apply_to_canvas() {
    mount_canvas("t-canvas", 320, 240);
    let themes = [
        "",
        "{}",
        r##"{"bg": "#ffffff"}"##,
        r##"{"bg": "white"}"##,
        r##"{"bg": "#fff"}"##,
        r##"{"bg": "rgb(255,255,255)"}"##,
        r##"{"bg": "#0c0c0c"}"##,
        r##"{"bg": "#0c0c0c", "accent": "#00e5ff"}"##,
        r##"{"accent": "rgb(0 229 255)"}"##,
        r##"{"bg": "not-a-color"}"##,
        "not json",
    ];
    for theme in themes {
        let r = plychart::wasm::update_candles("t-canvas", CANDLES_2, theme);
        assert!(
            r.is_ok(),
            "update_candles failed for theme {theme:?}: {r:?}"
        );
    }
    remove_canvas("t-canvas");
}

/// create_chart fills the canvas with the theme background.
#[wasm_bindgen_test]
fn create_chart_with_theme_succeeds() {
    mount_canvas("t-create", 100, 80);
    assert!(plychart::wasm::create_chart("t-create", 100, 80).is_ok());
    assert!(plychart::wasm::destroy_chart("t-create").is_ok());
    remove_canvas("t-create");
}

/// A missing canvas must return Err, never panic.
#[wasm_bindgen_test]
fn missing_canvas_is_err_not_panic() {
    let r = plychart::wasm::update_candles("no-such-canvas", CANDLES_2, "");
    assert!(r.is_err(), "expected CanvasNotFound, got {r:?}");
    assert!(plychart::wasm::create_chart("no-such-canvas", 10, 10).is_err());
}

// ---------------------------------------------------------------------------
// Render entry points with empty / small data
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn render_entry_points_handle_empty_data() {
    mount_canvas("e-canvas", 320, 240);
    assert!(plychart::wasm::create_chart("e-canvas", 320, 240).is_ok());
    let theme = r#"{"bg": "white"}"#;

    assert!(plychart::wasm::update_candles("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_line("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_area("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_bar("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_heatmap("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_heatmap_div("e-canvas", "[]", 0.0, theme).is_ok());
    assert!(plychart::wasm::update_order_book("e-canvas", "{}", theme).is_ok());
    assert!(plychart::wasm::update_scatter("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_scatter_multi("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_radar("e-canvas", "[]", "[]", "#fff", theme).is_ok());
    assert!(plychart::wasm::update_radar_multi("e-canvas", "[]", "[]", theme).is_ok());
    assert!(plychart::wasm::update_treemap("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_waterfall("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_correlation("e-canvas", "[]", "[]", theme).is_ok());
    assert!(plychart::wasm::update_stacked_bar("e-canvas", "[]", "[]", theme).is_ok());
    assert!(plychart::wasm::update_pie("e-canvas", "[]", theme).is_ok());
    assert!(plychart::wasm::update_histogram("e-canvas", "[]", 5, theme).is_ok());
    assert!(plychart::wasm::update_histogram_log("e-canvas", "[]", 5, theme).is_ok());
    assert!(plychart::wasm::update_sparkline("e-canvas", "[]", "#fff", theme).is_ok());
    assert!(plychart::wasm::update_backtest("e-canvas", "[]", "[]", theme).is_ok());
    assert!(plychart::wasm::update_gauge("e-canvas", 0.0, 100.0, "#00e5ff", theme).is_ok());
    assert!(
        plychart::wasm::update_gauge_zoned("e-canvas", 0.0, 100.0, "#00e5ff", "[]", theme).is_ok()
    );

    remove_canvas("e-canvas");
}

#[wasm_bindgen_test]
fn render_entry_points_handle_single_point() {
    mount_canvas("s-canvas", 320, 240);
    assert!(plychart::wasm::create_chart("s-canvas", 320, 240).is_ok());
    let theme = r##"{"bg": "#0c0c0c"}"##;

    // Single candle hits the special single-point render path.
    assert!(plychart::wasm::update_candles("s-canvas", &format!("[{CANDLE}]"), theme).is_ok());
    assert!(plychart::wasm::update_line("s-canvas", &format!("[{CANDLE}]"), theme).is_ok());
    assert!(plychart::wasm::update_bar("s-canvas", &format!("[{CANDLE}]"), theme).is_ok());
    assert!(plychart::wasm::update_area("s-canvas", &format!("[{CANDLE}]"), theme).is_ok());
    assert!(plychart::wasm::update_heatmap("s-canvas", "[[1.0]]", theme).is_ok());
    assert!(plychart::wasm::update_heatmap_div("s-canvas", "[[1.0,-1.0]]", 0.0, theme).is_ok());
    assert!(
        plychart::wasm::update_scatter("s-canvas", "[[1.0,2.0]]", theme).is_ok(),
        "scatter with one point"
    );
    assert!(plychart::wasm::update_sparkline("s-canvas", "[42.0]", "#fff", theme).is_ok());
    assert!(plychart::wasm::update_histogram("s-canvas", "[1.0]", 3, theme).is_ok());
    assert!(
        plychart::wasm::update_gauge("s-canvas", 50.0, 100.0, "#00e5ff", theme).is_ok(),
        "gauge mid-value"
    );

    remove_canvas("s-canvas");
}

#[wasm_bindgen_test]
fn multi_series_line_and_area_render() {
    mount_canvas("m-canvas", 320, 240);
    let theme = r#"{"bg": "white"}"#;
    // Two series: one with an explicit color, one falling back to theme accent.
    let two_series =
        &format!(r##"[{{"color": "#ff0000", "data": [{CANDLE}]}}, {{"data": [{CANDLE}]}}]"##);
    assert!(plychart::wasm::update_line("m-canvas", two_series, theme).is_ok());
    assert!(plychart::wasm::update_area("m-canvas", two_series, theme).is_ok());
    remove_canvas("m-canvas");
}

#[wasm_bindgen_test]
fn invalid_data_json_is_err_not_panic() {
    mount_canvas("i-canvas", 320, 240);
    assert!(plychart::wasm::update_candles("i-canvas", "not json", "").is_err());
    assert!(plychart::wasm::update_heatmap("i-canvas", "not json", "").is_err());
    assert!(plychart::wasm::update_pie("i-canvas", "not json", "").is_err());
    remove_canvas("i-canvas");
}

// ---------------------------------------------------------------------------
// Interaction state transitions
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn interaction_zoom_pan_reset_transitions() {
    use plychart::ChartInteraction;

    let mut i = ChartInteraction::new();
    assert_eq!(i.viewport.count, 100);

    // Zoom in: 100 -> 90 candles.
    i.on_wheel(-10.0, 500);
    assert_eq!(i.viewport.count, 90);

    // Drag start inside the price area begins a pan.
    i.on_mouse_down(400.0, 100.0, 500);
    assert!(i.dragging);
    let start_before = i.viewport.start;

    // Dragging left pans forward through the data.
    let moved = i.on_mouse_drag(300.0, 500);
    assert!(moved || i.viewport.start == start_before);
    assert!(i.viewport.start <= 500 - i.viewport.count);

    // Mouse up ends the drag; further drags are no-ops.
    i.on_mouse_up();
    assert!(!i.dragging);
    assert!(!i.on_mouse_drag(200.0, 500));

    // Reset restores the full view.
    i.reset_viewport(500);
    assert_eq!((i.viewport.start, i.viewport.count), (0, 500));
}

#[wasm_bindgen_test]
fn interaction_touch_and_pinch() {
    use plychart::ChartInteraction;

    let mut i = ChartInteraction::new();
    i.on_touch_start(400.0, 100.0, 500);
    assert!(i.dragging);
    i.on_touch_move(350.0, 100.0, 500);
    assert!(i.mouse.in_chart);
    i.on_touch_end();
    assert!(!i.dragging);

    // Pinch zoom-in reduces visible count; pinch out enlarges it, clamped to total.
    i.on_pinch(2.0, 500);
    let after_zoom_in = i.viewport.count;
    assert!(after_zoom_in <= 100);
    i.on_pinch(0.5, 500);
    assert!(i.viewport.count >= after_zoom_in);
    assert!(i.viewport.count <= 500);

    // Keyboard navigation.
    let start = i.viewport.start;
    i.on_key_pan(10, 500);
    assert!(i.viewport.start >= start);
    i.on_key_zoom(true, 500);
    assert!(i.viewport.count <= 100);
    i.toggle_log_scale();
    assert!(i.viewport.log_scale);
}

// ---------------------------------------------------------------------------
// Tooltip / click hit-testing (canvas-size fallback paths)
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn tooltip_data_empty_returns_empty_object() {
    let r = plychart::wasm::get_tooltip_data("no-canvas", 10.0, 10.0, "[]", 0)
        .expect("tooltip on empty data");
    assert_eq!(r, "{}");
}

#[wasm_bindgen_test]
fn tooltip_data_nearest_index() {
    let r = plychart::wasm::get_tooltip_data(
        "no-canvas",
        780.0,
        10.0,
        &format!("[{CANDLE},{CANDLE}]"),
        0,
    )
    .expect("tooltip");
    assert!(r.contains("\"index\":1"), "expected last index, got {r}");
}

#[wasm_bindgen_test]
fn click_data_pie_and_default() {
    // data_len=4 pie: a point to the right of centre is in slice 0.
    let r = plychart::wasm::get_click_data("no-canvas", 700.0, 300.0, 4, "pie").expect("pie click");
    assert!(r.contains("\"index\":0"), "got {r}");

    // Default nearest-index: far right of an 800-wide canvas -> last index.
    let r = plychart::wasm::get_click_data("no-canvas", 795.0, 300.0, 4, "line").expect("click");
    assert!(r.contains("\"index\":3"), "got {r}");

    // Empty data -> empty object.
    let r = plychart::wasm::get_click_data("no-canvas", 1.0, 1.0, 0, "pie").expect("empty click");
    assert_eq!(r, "{}");
}

#[wasm_bindgen_test]
fn click_data_scatter_nearest() {
    let r =
        plychart::wasm::get_click_data("no-canvas", 400.0, 300.0, 10, "scatter").expect("scatter");
    assert!(r.contains("\"distance\""), "got {r}");
}
