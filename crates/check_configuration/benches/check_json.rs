use check_configuration::Configuration;
use check_deserialize::json::deserialize_from_json_str;
use check_json_parser::JsonParserOptions;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;
fn bench_check_json(criterion: &mut Criterion) {
    let code = include_str!("check_bench.json");

    let mut group = criterion.benchmark_group("check_json");
    group.throughput(Throughput::Bytes(code.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("deserialize_from_json_str", "check.json"),
        &code,
        |b, _| {
            b.iter(|| {
                black_box(deserialize_from_json_str::<Configuration>(
                    code,
                    JsonParserOptions::default(),
                    "check.json",
                ));
            })
        },
    );

    group.finish();
}

criterion_group!(check_json, bench_check_json);
criterion_main!(check_json);
