//! Core Canvas2D helpers — DPR, resize, chart lifecycle, rendering.

/// Get the device pixel ratio.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn device_pixel_ratio() -> f64 {
    web_sys::window().map(|w| w.device_pixel_ratio()).unwrap_or(1.0)
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
    let document = window.document().ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document.get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element.dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' is not a canvas")))?;

    canvas.set_width(width);
    canvas.set_height(height);

    let dpr = device_pixel_ratio();
    let ctx = canvas.get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    // Clear with dark background
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
pub fn update_candles(canvas_id: &str, data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window.document().ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document.get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element.dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas.get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea { x: 0.0, y: 0.0, w: width, h: height };
    let theme = plycore::ChartTheme::dark();

    match data.len() {
        0 => {}
        1 => {
            let x = area.x + area.w / 2.0;
            let y = area.y + area.h / 2.0;
            ctx.set_fill_style(&theme.accent.into());
            ctx.begin_path();
            ctx.arc(x, y, 4.0, 0.0, std::f64::consts::TAU).unwrap_or_default();
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
pub fn update_candles(_canvas_id: &str, _data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with line data.
#[cfg(target_arch = "wasm32")]
pub fn update_line(canvas_id: &str, data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    let window = web_sys::window().ok_or(crate::ChartError::CanvasNotFound("No window".into()))?;
    let document = window.document().ok_or(crate::ChartError::CanvasNotFound("No document".into()))?;
    let element = document.get_element_by_id(canvas_id)
        .ok_or_else(|| crate::ChartError::CanvasNotFound(format!("'{canvas_id}' not found")))?;
    let canvas: web_sys::HtmlCanvasElement = element.dyn_into()
        .map_err(|_| crate::ChartError::CanvasNotFound("Not a canvas".into()))?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas.get_context("2d")
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    clear_canvas(&ctx, "#0a0a0a", width, height);

    let area = plycore::ChartArea { x: 0.0, y: 0.0, w: width, h: height };
    let theme = plycore::ChartTheme::dark();
    crate::charts::line::draw(&ctx, data, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_line(_canvas_id: &str, _data: &[plycore::CandleData]) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with heatmap data.
pub fn update_heatmap(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with order book data.
pub fn update_order_book(_canvas_id: &str, _data_json: &str) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Destroy a chart and clean up resources.
pub fn destroy_chart(_canvas_id: &str) -> Result<(), crate::ChartError> {
    Ok(())
}
