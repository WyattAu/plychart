//! WASM entry points for plychart.

use wasm_bindgen::prelude::*;
use plycore::CandleData;

/// Create a chart bound to a canvas element.
#[wasm_bindgen]
pub fn create_chart(canvas_id: &str, width: u32, height: u32) -> Result<(), JsValue> {
    crate::canvas::create_chart(canvas_id, width, height)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Update chart with OHLCV candle data.
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

/// Update chart with heatmap data.
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

/// Destroy a chart and clean up resources.
#[wasm_bindgen]
pub fn destroy_chart(canvas_id: &str) -> Result<(), JsValue> {
    crate::canvas::destroy_chart(canvas_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
