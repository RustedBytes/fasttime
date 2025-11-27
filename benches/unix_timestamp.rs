use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fasttime::DateTime;
use time::OffsetDateTime;

fn unix_samples() -> Vec<(i64, i32)> {
    (0..1024)
        .map(|i| {
            let secs = (i as i64 * 250_000) - 125_000_000;
            let nanos = ((i as i64 * 97_921) % 2_000_000_000) as i32 - 1_000_000_000;
            (secs, nanos)
        })
        .collect()
}

fn bench_from_unix_timestamp(c: &mut Criterion) {
    let samples = unix_samples();

    let mut group = c.benchmark_group("from_unix_timestamp");
    group.bench_function("fasttime::DateTime", |b| {
        b.iter(|| {
            for &(secs, nanos) in &samples {
                black_box(DateTime::from_unix_timestamp(secs, nanos).unwrap());
            }
        });
    });

    group.bench_function("time::OffsetDateTime", |b| {
        b.iter(|| {
            for &(secs, nanos) in &samples {
                let total_nanos = secs as i128 * 1_000_000_000 + nanos as i128;
                black_box(OffsetDateTime::from_unix_timestamp_nanos(total_nanos).unwrap());
            }
        });
    });
    group.finish();
}

criterion_group!(benches, bench_from_unix_timestamp);
criterion_main!(benches);
