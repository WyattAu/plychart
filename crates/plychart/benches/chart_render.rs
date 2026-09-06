//! Criterion benches for chart render-data-prep paths — the work that happens
//! per `update_*` call before any Canvas2D drawing: JSON deserialization of
//! input payloads, data extents, histogram binning, and interaction viewport
//! math. Native-only: criterion does not compile on wasm32, and the browser
//! render path itself is covered by tests/wasm.rs.

#[cfg(not(target_arch = "wasm32"))]
use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(not(target_arch = "wasm32"))]
mod prep {
    use plycore::CandleData;

    pub fn candles(n: usize) -> Vec<CandleData> {
        (0..n)
            .map(|i| CandleData {
                time: i as f64,
                open: 100.0 + (i as f64 * 0.01).sin(),
                high: 100.5 + (i as f64 * 0.01).sin(),
                low: 99.5 + (i as f64 * 0.01).sin(),
                close: 100.0 + (i as f64 * 0.01).cos(),
                volume: 1000.0,
            })
            .collect()
    }

    /// Min/max extent over OHLC — the prep every time-series chart performs
    /// to fit the price axis.
    pub fn ohlc_extent(data: &[CandleData]) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for c in data {
            min = min.min(c.low);
            max = max.max(c.high);
        }
        (min, max)
    }

    pub fn heatmap_matrix(rows: usize, cols: usize) -> String {
        let row: Vec<String> = (0..cols)
            .map(|c| (c as f64 * 0.1).sin().to_string())
            .collect();
        let body: Vec<String> = (0..rows).map(|_| format!("[{}]", row.join(","))).collect();
        format!("[{}]", body.join(","))
    }

    pub fn order_book(levels: usize) -> String {
        let side = |sign: f64| -> Vec<String> {
            (0..levels)
                .map(|i| {
                    format!(
                        "[{},{}]",
                        100.0 + sign * i as f64 * 0.5,
                        (1000.0 * (1.0 + i as f64 * 0.01)) as u64
                    )
                })
                .collect()
        };
        format!(
            r#"{{"bids": [{}], "asks": [{}]}}"#,
            side(-1.0).join(","),
            side(1.0).join(",")
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn bench_candle_render(c: &mut Criterion) {
    use plychart::ChartInteraction;
    use prep::{candles, heatmap_matrix, ohlc_extent, order_book};

    let data = candles(1000);
    let data_json = serde_json::to_string(&data).unwrap();

    c.bench_function("candle_data_serialize_1k", |b| {
        b.iter(|| serde_json::to_string(black_box(&data)).unwrap())
    });

    // Deserialize is the entry-path prep for update_candles/update_line/etc.
    c.bench_function("candle_data_deserialize_1k", |b| {
        b.iter(|| {
            let parsed: Vec<plycore::CandleData> =
                serde_json::from_str(black_box(&data_json)).unwrap();
            parsed
        })
    });

    // Price-axis fitting over OHLC.
    c.bench_function("candle_extent_10k", |b| {
        let big = candles(10_000);
        b.iter(|| ohlc_extent(black_box(&big)))
    });

    // update_heatmap prep: matrix JSON -> Vec<Vec<f64>>.
    c.bench_function("heatmap_matrix_deserialize_50x50", |b| {
        let matrix_json = heatmap_matrix(50, 50);
        b.iter(|| {
            let m: Vec<Vec<f64>> = serde_json::from_str(black_box(&matrix_json)).unwrap();
            m
        })
    });

    // update_order_book prep: JSON -> bid/ask (price, size) pairs.
    c.bench_function("order_book_parse_500_per_side", |b| {
        let ob_json = order_book(500);
        b.iter(|| {
            let v: serde_json::Value = serde_json::from_str(black_box(&ob_json)).unwrap();
            v
        })
    });

    // update_histogram prep: binning 10k values into 50 bins.
    c.bench_function("histogram_bins_10k_b50", |b| {
        let values: Vec<f64> = (0..10_000)
            .map(|i| (i as f64 * 0.37).sin() * 50.0)
            .collect();
        b.iter(|| plychart::charts::histogram::compute_bins(black_box(&values), 50).unwrap())
    });

    // Interaction state machine: a zoom + drag-pan sequence, hot on wheel/drag.
    c.bench_function("interaction_zoom_pan_1k", |b| {
        b.iter(|| {
            let mut i = ChartInteraction::new();
            for t in 0..1000 {
                i.on_wheel(if t % 2 == 0 { -10.0 } else { 10.0 }, 5000);
                i.on_mouse_down(400.0, 100.0, 5000);
                i.on_mouse_drag(400.0 + (t % 40) as f64 - 20.0, 5000);
                i.on_mouse_up();
            }
            i.viewport
        })
    });
}

#[cfg(not(target_arch = "wasm32"))]
criterion_group!(benches, bench_candle_render);
#[cfg(not(target_arch = "wasm32"))]
criterion_main!(benches);

// harness = false bench targets are plain binaries; wasm32 builds get a stub.
#[cfg(target_arch = "wasm32")]
fn main() {}
