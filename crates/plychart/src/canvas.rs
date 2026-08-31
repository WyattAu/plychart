//! Core Canvas2D helpers — DPR, resize, chart lifecycle, rendering.

/// Get the device pixel ratio.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn device_pixel_ratio() -> f64 {
    web_sys::window()
        .map(|w| w.device_pixel_ratio())
        .unwrap_or(1.0)
}

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn device_pixel_ratio() -> f64 {
    1.0
}

/// Create a chart on a canvas element.
#[cfg(target_arch = "wasm32")]
pub fn create_chart(canvas_id: &str, width: u32, height: u32) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' is not a canvas")))?;

    canvas.set_width(width);
    canvas.set_height(height);

    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    ctx.set_fill_style(&"#0a0a0a".into());
    ctx.fill_rect(0.0, 0.0, width as f64, height as f64);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_chart(_canvas_id: &str, _width: u32, _height: u32) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Fill canvas with background color.
#[cfg(target_arch = "wasm32")]
pub fn clear_canvas(ctx: &web_sys::CanvasRenderingContext2d, bg: &str, w: f64, h: f64) {
    ctx.set_fill_style(&bg.into());
    ctx.fill_rect(0.0, 0.0, w, h);
}

/// Update chart with OHLCV candle data.
#[cfg(target_arch = "wasm32")]
pub fn update_candles(
    canvas_id: &str,
    data: &[plycore::CandleData],
) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();

    match data.len() {
        0 => {}
        1 => {
            let x = area.x + area.w / 2.0;
            let y = area.y + area.h / 2.0;
            ctx.set_fill_style(&theme.accent.into());
            ctx.begin_path();
            ctx.arc(x, y, 4.0, 0.0, std::f64::consts::TAU)
                .unwrap_or_default();
            ctx.fill();
        }
        _ => {
            let candle_w = (area.w / data.len() as f64 * 0.7).max(1.0);
            crate::charts::candlestick::draw(&ctx, data, &area, &theme, candle_w);
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_candles(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with line data.
#[cfg(target_arch = "wasm32")]
pub fn update_line(canvas_id: &str, data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::line::draw(&ctx, data, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_line(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with bar data (uses bar renderer, not candlestick).
#[cfg(target_arch = "wasm32")]
pub fn update_bar(canvas_id: &str, data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::bar::draw(&ctx, data, &area, &theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_bar(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with area data (uses area renderer, not candlestick).
#[cfg(target_arch = "wasm32")]
pub fn update_area(canvas_id: &str, data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::area::draw(&ctx, data, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_area(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with heatmap data.
#[cfg(target_arch = "wasm32")]
pub fn update_heatmap(canvas_id: &str, data_json: &str) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let matrix: Vec<Vec<f64>> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    if !matrix.is_empty() {
        let area = plycore::ChartArea {
            x: 0.0,
            y: 0.0,
            w: width,
            h: height,
        };
        crate::charts::heatmap::draw(&ctx, &matrix, &area);
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_heatmap(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with order book data.
#[cfg(target_arch = "wasm32")]
pub fn update_order_book(canvas_id: &str, data_json: &str) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let v: serde_json::Value = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let bids = v
        .get("bids")
        .and_then(|b| b.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|p| {
                    let arr = p.as_array()?;
                    Some((arr.get(0)?.as_f64()?, arr.get(1)?.as_f64()?))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let asks = v
        .get("asks")
        .and_then(|a| a.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|p| {
                    let arr = p.as_array()?;
                    Some((arr.get(0)?.as_f64()?, arr.get(1)?.as_f64()?))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::order_book::draw(&ctx, &bids, &asks, &area, &theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_order_book(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with scatter data.
#[cfg(target_arch = "wasm32")]
pub fn update_scatter(canvas_id: &str, data_json: &str) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let points: Vec<(f64, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::scatter::draw(&ctx, &points, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_scatter(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with gauge value.
#[cfg(target_arch = "wasm32")]
pub fn update_gauge(
    canvas_id: &str,
    value: f64,
    max: f64,
    color: &str,
) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::gauge::draw(&ctx, value, max, color, &area);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_gauge(
    _canvas_id: &str,
    _value: f64,
    _max: f64,
    _color: &str,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with backtest equity + drawdown data.
#[cfg(target_arch = "wasm32")]
pub fn update_backtest(
    canvas_id: &str,
    equity_json: &str,
    drawdown_json: &str,
) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let equity: Vec<f64> = serde_json::from_str(equity_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let drawdown: Vec<f64> = serde_json::from_str(drawdown_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::backtest::draw(&ctx, &equity, &drawdown, &area, &theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_backtest(
    _canvas_id: &str,
    _equity_json: &str,
    _drawdown_json: &str,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with radar data.
#[cfg(target_arch = "wasm32")]
pub fn update_radar(
    canvas_id: &str,
    values_json: &str,
    labels_json: &str,
    color: &str,
) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let values: Vec<f64> = serde_json::from_str(values_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let labels: Vec<String> = serde_json::from_str(labels_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::radar::draw(&ctx, &values, &label_refs, color, &area);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_radar(
    _canvas_id: &str,
    _values_json: &str,
    _labels_json: &str,
    _color: &str,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with treemap data.
#[cfg(target_arch = "wasm32")]
pub fn update_treemap(canvas_id: &str, data_json: &str) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let items: Vec<(String, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::treemap::draw(&ctx, &items, &area);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_treemap(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with waterfall data.
#[cfg(target_arch = "wasm32")]
pub fn update_waterfall(canvas_id: &str, data_json: &str) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let bars: Vec<(String, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    let theme = plycore::ChartTheme::dark();
    crate::charts::waterfall::draw(&ctx, &bars, &area, &theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_waterfall(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with correlation matrix.
#[cfg(target_arch = "wasm32")]
pub fn update_correlation(
    canvas_id: &str,
    matrix_json: &str,
    labels_json: &str,
) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window
        .document()
        .ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element
        .dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let matrix: Vec<Vec<f64>> = serde_json::from_str(matrix_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let labels: Vec<String> = serde_json::from_str(labels_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let area = plycore::ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::correlation::draw(&ctx, &matrix, &label_refs, &area);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_correlation(
    _canvas_id: &str,
    _matrix_json: &str,
    _labels_json: &str,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Destroy a chart and clean up resources.
#[cfg(target_arch = "wasm32")]
pub fn destroy_chart(canvas_id: &str) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(canvas_id) {
                if let Ok(canvas) = element.dyn_into::<web_sys::HtmlCanvasElement>() {
                    let width = canvas.width();
                    let height = canvas.height();
                    if let Some(ctx) = canvas
                        .get_context("2d")
                        .and_then(|c| c.dyn_into::<web_sys::CanvasRenderingContext2d>().ok())
                    {
                        ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn destroy_chart(_canvas_id: &str) -> Result<(), crate::ChartError> {
    Ok(())
}
