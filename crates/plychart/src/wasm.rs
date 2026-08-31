//! WASM entry points for plychart.

use wasm_bindgen::prelude::*;
use plycore::CandleData;

/// Create a chart bound to a canvas element.
#[wasm_bindgen]
pub fn create_chart(canvas_id: &str, width: u32, height: u32) -> Result<(), JsValue> {
    crate::canvas::create_chart(canvas_id, width, height)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with OHLCV candle data (candlestick, line, area, bar).
#[wasm_bindgen]
pub fn update_candles(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::update_candles(canvas_id, &data)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with line data.
#[wasm_bindgen]
pub fn update_line(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::update_line(canvas_id, &data)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with heatmap matrix data.
#[wasm_bindgen]
pub fn update_heatmap(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_heatmap(canvas_id, data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with order book data.
#[wasm_bindgen]
pub fn update_order_book(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_order_book(canvas_id, data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with scatter data.
#[wasm_bindgen]
pub fn update_scatter(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_scatter(canvas_id, data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with gauge value.
#[wasm_bindgen]
pub fn update_gauge(canvas_id: &str, value: f64, max: f64, color: &str) -> Result<(), JsValue> {
    crate::canvas::update_gauge(canvas_id, value, max, color)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with bar data.
#[wasm_bindgen]
pub fn update_bar(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::update_bar(canvas_id, &data)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with backtest equity + drawdown data.
#[wasm_bindgen]
pub fn update_backtest(canvas_id: &str, equity_json: &str, drawdown_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_backtest(canvas_id, equity_json, drawdown_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with area data.
#[wasm_bindgen]
pub fn update_area(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    let data: Vec<CandleData> = serde_json::from_str(data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::canvas::update_area(canvas_id, &data)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with radar data.
#[wasm_bindgen]
pub fn update_radar(canvas_id: &str, values_json: &str, labels_json: &str, color: &str) -> Result<(), JsValue> {
    crate::canvas::update_radar(canvas_id, values_json, labels_json, color)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with treemap data.
#[wasm_bindgen]
pub fn update_treemap(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_treemap(canvas_id, data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with waterfall data.
#[wasm_bindgen]
pub fn update_waterfall(canvas_id: &str, data_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_waterfall(canvas_id, data_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with correlation matrix.
#[wasm_bindgen]
pub fn update_correlation(canvas_id: &str, matrix_json: &str, labels_json: &str) -> Result<(), JsValue> {
    crate::canvas::update_correlation(canvas_id, matrix_json, labels_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Destroy a chart and clean up resources.
#[wasm_bindgen]
pub fn destroy_chart(canvas_id: &str) -> Result<(), JsValue> {
    crate::canvas::destroy_chart(canvas_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
