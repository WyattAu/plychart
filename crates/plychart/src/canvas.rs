//! Core Canvas2D helpers — DPR, resize, chart lifecycle, rendering.

#[cfg(target_arch = "wasm32")]
use plycore::ChartArea;
use plycore::ChartTheme;

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

/// Look up a canvas element by ID and return its 2D rendering context,
/// logical width, and logical height.
///
/// Applies DPR scaling to the context so callers can work in logical pixels.
#[cfg(target_arch = "wasm32")]
pub(crate) fn get_canvas_context(
    canvas_id: &str,
) -> Result<(web_sys::CanvasRenderingContext2d, f64, f64), crate::ChartError> {
    use wasm_bindgen::JsCast;
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

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let dpr = device_pixel_ratio();
    let ctx = canvas
        .get_context("2d")
        .map_err(|_| crate::ChartError::RenderError("get_context failed".into()))?
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| crate::ChartError::RenderError("Not a CanvasRenderingContext2d".into()))?;
    ctx.scale(dpr, dpr).unwrap_or_default();

    Ok((ctx, width, height))
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
        .map_err(|_| crate::ChartError::RenderError("get_context failed".into()))?
        .ok_or(crate::ChartError::RenderError("No 2D context".into()))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| crate::ChartError::RenderError("Not a CanvasRenderingContext2d".into()))?;
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

/// Public wrapper for WASM multi-series rendering.
#[cfg(target_arch = "wasm32")]
pub fn get_canvas_context_wasm(
    canvas_id: &str,
) -> Result<(web_sys::CanvasRenderingContext2d, f64, f64), crate::ChartError> {
    get_canvas_context(canvas_id)
}

/// Update chart with OHLCV candle data.
#[cfg(target_arch = "wasm32")]
pub fn update_candles(
    canvas_id: &str,
    data: &[plycore::CandleData],
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };

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
            crate::charts::candlestick::draw(&ctx, data, &area, theme, candle_w);
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_candles(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with line data.
#[cfg(target_arch = "wasm32")]
pub fn update_line(
    canvas_id: &str,
    data: &[plycore::CandleData],
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::line::draw(&ctx, data, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_line(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with bar data.
#[cfg(target_arch = "wasm32")]
pub fn update_bar(
    canvas_id: &str,
    data: &[plycore::CandleData],
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::bar::draw(&ctx, data, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_bar(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with area data.
#[cfg(target_arch = "wasm32")]
pub fn update_area(
    canvas_id: &str,
    data: &[plycore::CandleData],
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::area::draw(&ctx, data, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_area(
    _canvas_id: &str,
    _data: &[plycore::CandleData],
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with heatmap data.
#[cfg(target_arch = "wasm32")]
pub fn update_heatmap(
    canvas_id: &str,
    data_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let matrix: Vec<Vec<f64>> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    if !matrix.is_empty() {
        let area = ChartArea {
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
pub fn update_heatmap(
    _canvas_id: &str,
    _data_json: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with order book data.
#[cfg(target_arch = "wasm32")]
pub fn update_order_book(
    canvas_id: &str,
    data_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

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

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::order_book::draw(&ctx, &bids, &asks, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_order_book(
    _canvas_id: &str,
    _data_json: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with scatter data.
#[cfg(target_arch = "wasm32")]
pub fn update_scatter(
    canvas_id: &str,
    data_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let points: Vec<(f64, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::scatter::draw(&ctx, &points, &area, theme.accent);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_scatter(
    _canvas_id: &str,
    _data_json: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with gauge value.
#[cfg(target_arch = "wasm32")]
pub fn update_gauge(
    canvas_id: &str,
    value: f64,
    max: f64,
    color: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let area = ChartArea {
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
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with backtest equity + drawdown data.
#[cfg(target_arch = "wasm32")]
pub fn update_backtest(
    canvas_id: &str,
    equity_json: &str,
    drawdown_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let equity: Vec<f64> = serde_json::from_str(equity_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let drawdown: Vec<f64> = serde_json::from_str(drawdown_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::backtest::draw(&ctx, &equity, &drawdown, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_backtest(
    _canvas_id: &str,
    _equity_json: &str,
    _drawdown_json: &str,
    _theme: &ChartTheme,
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
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let values: Vec<f64> = serde_json::from_str(values_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let labels: Vec<String> = serde_json::from_str(labels_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let area = ChartArea {
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
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with treemap data.
#[cfg(target_arch = "wasm32")]
pub fn update_treemap(
    canvas_id: &str,
    data_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let items: Vec<(String, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::treemap::draw(&ctx, &items, &area);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_treemap(
    _canvas_id: &str,
    _data_json: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with waterfall data.
#[cfg(target_arch = "wasm32")]
pub fn update_waterfall(
    canvas_id: &str,
    data_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let raw_bars: Vec<(String, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let bars: Vec<plycore::BarData> = raw_bars
        .into_iter()
        .map(|(label, value)| plycore::BarData {
            label,
            value,
            color: None,
        })
        .collect();

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::waterfall::draw(&ctx, &bars, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_waterfall(
    _canvas_id: &str,
    _data_json: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with correlation matrix.
#[cfg(target_arch = "wasm32")]
pub fn update_correlation(
    canvas_id: &str,
    matrix_json: &str,
    labels_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let matrix: Vec<Vec<f64>> = serde_json::from_str(matrix_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let labels: Vec<String> = serde_json::from_str(labels_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let area = ChartArea {
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
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with pie/donut data.
#[cfg(target_arch = "wasm32")]
pub fn update_pie(
    canvas_id: &str,
    data_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let items: Vec<(String, f64)> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::pie::draw(&ctx, &items, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_pie(
    _canvas_id: &str,
    _data_json: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with histogram data.
#[cfg(target_arch = "wasm32")]
pub fn update_histogram(
    canvas_id: &str,
    data_json: &str,
    bin_count: usize,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let values: Vec<f64> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::histogram::draw(&ctx, &values, bin_count, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_histogram(
    _canvas_id: &str,
    _data_json: &str,
    _bin_count: usize,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with sparkline data.
#[cfg(target_arch = "wasm32")]
pub fn update_sparkline(
    canvas_id: &str,
    data_json: &str,
    color: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let values: Vec<f64> = serde_json::from_str(data_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;

    let area = ChartArea {
        x: 0.0,
        y: 0.0,
        w: width,
        h: height,
    };
    crate::charts::sparkline::draw(&ctx, &values, &area, color);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_sparkline(
    _canvas_id: &str,
    _data_json: &str,
    _color: &str,
    _theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    Ok(())
}

/// Update chart with stacked bar data.
#[cfg(target_arch = "wasm32")]
pub fn update_stacked_bar(
    canvas_id: &str,
    matrix_json: &str,
    labels_json: &str,
    theme: &ChartTheme,
) -> Result<(), crate::ChartError> {
    let (ctx, width, height) = get_canvas_context(canvas_id)?;
    clear_canvas(&ctx, theme.bg, width, height);

    let matrix: Vec<Vec<f64>> = serde_json::from_str(matrix_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let labels: Vec<String> = serde_json::from_str(labels_json)
        .map_err(|e| crate::ChartError::DataParseError(e.to_string()))?;
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let area = ChartArea {
        x: 40.0,
        y: 10.0,
        w: width - 50.0,
        h: height - 30.0,
    };
    crate::charts::stacked_bar::draw(&ctx, &matrix, &label_refs, &area, theme);

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_stacked_bar(
    _canvas_id: &str,
    _matrix_json: &str,
    _labels_json: &str,
    _theme: &ChartTheme,
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
                    if let Ok(Some(ctx_obj)) = canvas.get_context("2d") {
                        if let Ok(ctx) = ctx_obj.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                            ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
                        }
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
