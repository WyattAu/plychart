//! Criterion benches for chart data-prep paths. Native-only: criterion does
//! not compile on wasm32, and benches are never run by the wasm browser job.

#[cfg(not(target_arch = "wasm32"))]
use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(not(target_arch = "wasm32"))]
fn bench_candle_render(c: &mut Criterion) {
    let data: Vec<plycore::CandleData> = (0..1000)
        .map(|i| plycore::CandleData {
            time: i as f64,
            open: 100.0 + (i as f64 * 0.01).sin(),
            high: 100.5 + (i as f64 * 0.01).sin(),
            low: 99.5 + (i as f64 * 0.01).sin(),
            close: 100.0 + (i as f64 * 0.01).cos(),
            volume: 1000.0,
        })
        .collect();

    c.bench_function("candle_data_serialize_1k", |b| {
        b.iter(|| serde_json::to_string(black_box(&data)).unwrap())
    });
}

#[cfg(not(target_arch = "wasm32"))]
criterion_group!(benches, bench_candle_render);
#[cfg(not(target_arch = "wasm32"))]
criterion_main!(benches);

// harness = false bench targets are plain binaries; wasm32 builds get a stub.
#[cfg(target_arch = "wasm32")]
fn main() {}
