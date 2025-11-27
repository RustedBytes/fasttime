use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fasttime::DateTime;
use time::OffsetDateTime;

// const SAMPLE_SIZES: &[(usize, &str)] = &[(64, "small"), (1024, "default"), (16_384, "large")];
const SAMPLE_SIZES: &[(usize, &str)] = &[(1024, "default")];

fn unix_samples(len: usize) -> Vec<(i64, i32)> {
    (0..len)
        .map(|i| {
            let secs = (i as i64 * 250_000) - 125_000_000;
            let nanos = ((i as i64 * 97_921) % 2_000_000_000) as i32 - 1_000_000_000;
            (secs, nanos)
        })
        .collect()
}

fn datetime_samples(len: usize) -> (Vec<DateTime>, Vec<OffsetDateTime>) {
    let raw = unix_samples(len);
    let mut fast = Vec::with_capacity(raw.len());
    let mut time = Vec::with_capacity(raw.len());
    for (secs, nanos) in raw {
        fast.push(DateTime::from_unix_timestamp(secs, nanos).unwrap());
        let total_nanos = secs as i128 * 1_000_000_000 + nanos as i128;
        time.push(OffsetDateTime::from_unix_timestamp_nanos(total_nanos).unwrap());
    }
    (fast, time)
}

fn bench_from_unix_timestamp(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_unix_timestamp");
    for &(len, label) in SAMPLE_SIZES {
        let samples = unix_samples(len);
        let fast_samples = samples.clone();
        let time_samples = samples;

        let fast_name = format!("fasttime::{label}-n={len}");
        group.bench_function(fast_name, move |b| {
            b.iter(|| {
                for &(secs, nanos) in &fast_samples {
                    black_box(DateTime::from_unix_timestamp(secs, nanos).unwrap());
                }
            });
        });

        let time_name = format!("time::{label}-n={len}");
        group.bench_function(time_name, move |b| {
            b.iter(|| {
                for &(secs, nanos) in &time_samples {
                    let total_nanos = secs as i128 * 1_000_000_000 + nanos as i128;
                    black_box(OffsetDateTime::from_unix_timestamp_nanos(total_nanos).unwrap());
                }
            });
        });
    }
    group.finish();
}

fn bench_to_unix_timestamp(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_unix_timestamp");
    for &(len, label) in SAMPLE_SIZES {
        let (fast_samples, time_samples) = datetime_samples(len);

        let fast_name = format!("fasttime::{label}-n={len}");
        group.bench_function(fast_name, move |b| {
            b.iter(|| {
                for dt in &fast_samples {
                    black_box(dt.unix_timestamp());
                }
            });
        });

        let time_name = format!("time::{label}-n={len}");
        group.bench_function(time_name, move |b| {
            b.iter(|| {
                for dt in &time_samples {
                    black_box(dt.unix_timestamp());
                }
            });
        });
    }
    group.finish();
}

fn bench_to_unix_timestamp_nanos(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_unix_timestamp_nanos");
    for &(len, label) in SAMPLE_SIZES {
        let (fast_samples, time_samples) = datetime_samples(len);

        let fast_name = format!("fasttime::{label}-n={len}");
        group.bench_function(fast_name, move |b| {
            b.iter(|| {
                for dt in &fast_samples {
                    black_box(dt.unix_timestamp_nanos());
                }
            });
        });

        let time_name = format!("time::{label}-n={len}");
        group.bench_function(time_name, move |b| {
            b.iter(|| {
                for dt in &time_samples {
                    black_box(dt.unix_timestamp_nanos());
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_from_unix_timestamp,
    bench_to_unix_timestamp,
    bench_to_unix_timestamp_nanos
);
criterion_main!(benches);
