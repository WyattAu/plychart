use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

criterion_group!(benches, bench_candle_render);
criterion_main!(benches);
