//! WASM entry points for plychart.

use wasm_bindgen::prelude::*;
use plycore::{CandleData, ChartTheme};

/// Parse a theme JSON string. Empty string or invalid JSON defaults to dark theme.
fn parse_theme(theme_json: &str) -> ChartTheme {
    if theme_json.is_empty() {
        return ChartTheme::dark();
    }
    serde_json::from_str(theme_json).unwrap_or_default()
}

/// Input format for multi-series data.
#[derive(serde::Deserialize)]
struct MultiSeriesInput {
    color: Option<String>,
    data: Option<Vec<CandleData>>,
}

/// Create a chart bound to a canvas element.
#[wasm_bindgen]
pub fn create_chart(canvas_id: &str, width: u32, height: u32) -> Result<(), JsValue> {
    crate::canvas::create_chart(canvas_id, width, height)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with OHLCV candle data (candlestick, line, area, bar).
#[wasm_bindgen]
pub fn update_candles(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let theme = parse_theme(theme_json);
    crate::canvas::update_candles(canvas_id, &data, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with line data.
/// Single series: `[{time,open,high,low,close,volume}]`
/// Multi series: `[{color:"#ff0000", data:[{time,open,high,low,close,volume}]}, ...]`
#[wasm_bindgen]
pub fn update_line(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);

    // Try multi-series first: [{color, data: [...]}]
    if let Ok(multi) = serde_json::from_str::<Vec<MultiSeriesInput>>(data_json) {
        if !multi.is_empty() && multi[0].data.is_some() {
            let series: Vec<crate::charts::multiline::DataSeries<'_>> = multi
                .iter()
                .filter_map(|m| {
                    Some(crate::charts::multiline::DataSeries {
                        color: m.color.as_deref().unwrap_or(theme.accent),
                        data: m.data.as_deref()?,
                    })
                })
                .collect();
            let (ctx, width, height) = crate::canvas::get_canvas_context_wasm(canvas_id)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            crate::canvas::clear_canvas(&ctx, theme.bg, width, height);
            let area = plycore::ChartArea { x: 0.0, y: 0.0, w: width, h: height };
            crate::charts::multiline::draw_lines(&ctx, &series, &area);
            return Ok(());
        }
    }

    // Fallback: single series [{time,open,high,low,close,volume}]
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::update_line(canvas_id, &data, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with heatmap matrix data.
#[wasm_bindgen]
pub fn update_heatmap(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_heatmap(canvas_id, data_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with order book data.
#[wasm_bindgen]
pub fn update_order_book(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_order_book(canvas_id, data_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with scatter data.
#[wasm_bindgen]
pub fn update_scatter(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_scatter(canvas_id, data_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with multi-series scatter data.
/// data_json: `[{color: "#ff0000", points: [[x,y], [x,y], ...]}, ...]`
#[wasm_bindgen]
pub fn update_scatter_multi(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    #[derive(serde::Deserialize)]
    struct ScatterSeries {
        color: String,
        points: Vec<Vec<f64>>,
    }

    let theme = parse_theme(theme_json);
    let series: Vec<ScatterSeries> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let (ctx, width, height) = crate::canvas::get_canvas_context_wasm(canvas_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::clear_canvas(&ctx, theme.bg, width, height);

    let area = plycore::ChartArea { x: 0.0, y: 0.0, w: width, h: height };
    let series_refs: Vec<(&[(f64, f64)], &str)> = series
        .iter()
        .map(|s| {
            let pts: Vec<(f64, f64)> = s.points.iter()
                .filter_map(|p| {
                    if p.len() >= 2 {
                        Some((p[0], p[1]))
                    } else {
                        None
                    }
                })
                .collect();
            (pts.leak() as &[(f64, f64)], s.color.as_str())
        })
        .collect();
    crate::charts::scatter::draw_multi(&ctx, &series_refs, &area);

    Ok(())
}

/// Update chart with gauge value.
#[wasm_bindgen]
pub fn update_gauge(canvas_id: &str, value: f64, max: f64, color: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_gauge(canvas_id, value, max, color, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with bar data.
#[wasm_bindgen]
pub fn update_bar(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let theme = parse_theme(theme_json);
    crate::canvas::update_bar(canvas_id, &data, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with backtest equity + drawdown data.
#[wasm_bindgen]
pub fn update_backtest(canvas_id: &str, equity_json: &str, drawdown_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_backtest(canvas_id, equity_json, drawdown_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with area data.
/// Single series: `[{time,open,high,low,close,volume}]`
/// Multi series: `[{color:"#ff0000", data:[{time,open,high,low,close,volume}]}, ...]`
#[wasm_bindgen]
pub fn update_area(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);

    // Try multi-series first
    if let Ok(multi) = serde_json::from_str::<Vec<MultiSeriesInput>>(data_json) {
        if !multi.is_empty() && multi[0].data.is_some() {
            let series: Vec<crate::charts::multiline::DataSeries<'_>> = multi
                .iter()
                .filter_map(|m| {
                    Some(crate::charts::multiline::DataSeries {
                        color: m.color.as_deref().unwrap_or(theme.accent),
                        data: m.data.as_deref()?,
                    })
                })
                .collect();
            let (ctx, width, height) = crate::canvas::get_canvas_context_wasm(canvas_id)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            crate::canvas::clear_canvas(&ctx, theme.bg, width, height);
            let area = plycore::ChartArea { x: 0.0, y: 0.0, w: width, h: height };
            crate::charts::multiline::draw_areas(&ctx, &series, &area);
            return Ok(());
        }
    }

    // Fallback: single series
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::update_area(canvas_id, &data, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with radar data.
#[wasm_bindgen]
pub fn update_radar(canvas_id: &str, values_json: &str, labels_json: &str, color: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_radar(canvas_id, values_json, labels_json, color, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with multi-series radar data.
/// data_json: `[{color: "#ff0000", values: [v1, v2, ...]}, ...]`
#[wasm_bindgen]
pub fn update_radar_multi(canvas_id: &str, data_json: &str, labels_json: &str, theme_json: &str) -> Result<(), JsValue> {
    #[derive(serde::Deserialize)]
    struct RadarSeries {
        color: String,
        values: Vec<f64>,
    }

    let theme = parse_theme(theme_json);
    let series: Vec<RadarSeries> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let labels: Vec<String> = serde_json::from_str(labels_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let (ctx, width, height) = crate::canvas::get_canvas_context_wasm(canvas_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::clear_canvas(&ctx, theme.bg, width, height);

    let area = plycore::ChartArea { x: 0.0, y: 0.0, w: width, h: height };
    let series_refs: Vec<(&[f64], &str)> = series
        .iter()
        .map(|s| (s.values.as_slice(), s.color.as_str()))
        .collect();
    crate::charts::radar::draw_multi(&ctx, &series_refs, &label_refs, &area);

    Ok(())
}

/// Update chart with treemap data.
#[wasm_bindgen]
pub fn update_treemap(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_treemap(canvas_id, data_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with waterfall data.
#[wasm_bindgen]
pub fn update_waterfall(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_waterfall(canvas_id, data_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with correlation matrix.
#[wasm_bindgen]
pub fn update_correlation(canvas_id: &str, matrix_json: &str, labels_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_correlation(canvas_id, matrix_json, labels_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Destroy a chart and clean up resources.
#[wasm_bindgen]
pub fn destroy_chart(canvas_id: &str) -> Result<(), JsValue> {
    crate::canvas::destroy_chart(canvas_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with pie/donut data.
/// data_json: `[["label", value], ...]`
#[wasm_bindgen]
pub fn update_pie(canvas_id: &str, data_json: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_pie(canvas_id, data_json, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with histogram data.
/// data_json: `[value1, value2, ...]`
#[wasm_bindgen]
pub fn update_histogram(canvas_id: &str, data_json: &str, bin_count: usize, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_histogram(canvas_id, data_json, bin_count, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with sparkline data.
/// data_json: `[value1, value2, ...]`
#[wasm_bindgen]
pub fn update_sparkline(canvas_id: &str, data_json: &str, color: &str, theme_json: &str) -> Result<(), JsValue> {
    let theme = parse_theme(theme_json);
    crate::canvas::update_sparkline(canvas_id, data_json, color, &theme)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get tooltip data for a mouse position over a candlestick/line/area/bar chart.
/// Returns JSON: `{index, time, open, high, low, close, volume}` or `{}` if no data.
#[wasm_bindgen]
pub fn get_tooltip_data(canvas_id: &str, x: f64, y: f64, data_json: &str) -> Result<String, JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    if data.is_empty() {
        return Ok("{}".to_string());
    }

    // Simple nearest-index lookup based on x position
    #[cfg(target_arch = "wasm32")]
    let width = {
        use wasm_bindgen::JsCast;
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(canvas_id))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .map(|c| c.width() as f64)
            .unwrap_or(800.0)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let width = 800.0;

    let idx = ((x / width) * data.len() as f64).round() as usize;
    let idx = idx.min(data.len() - 1);
    let c = &data[idx];

    serde_json::to_string(&serde_json::json!({
        "index": idx,
        "time": c.time,
        "open": c.open,
        "high": c.high,
        "low": c.low,
        "close": c.close,
        "volume": c.volume,
    }))
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get click data for a mouse position.
/// Returns JSON: `{index, x, y}` with the nearest data index.
#[wasm_bindgen]
pub fn get_click_data(canvas_id: &str, x: f64, y: f64, data_len: usize) -> Result<String, JsValue> {
    if data_len == 0 {
        return Ok("{}".to_string());
    }

    #[cfg(target_arch = "wasm32")]
    let width = {
        use wasm_bindgen::JsCast;
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(canvas_id))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .map(|c| c.width() as f64)
            .unwrap_or(800.0)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let width = 800.0;

    let idx = ((x / width) * data_len as f64).round() as usize;
    let idx = idx.min(data_len - 1);

    serde_json::to_string(&serde_json::json!({
        "index": idx,
        "x": x,
        "y": y,
    }))
    .map_err(|e| JsValue::from_str(&e.to_string()))
}
