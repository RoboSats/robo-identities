use robohash::*;
use std::error::Error;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_robohash(
    initial_string: &str,
    use_background: &bool,
    size: u32,
) -> Result<(), Box<dyn Error>> {
    // build
    let robo_hash: RoboHash = RoboHashBuilder::new(initial_string)
        .with_background(use_background)
        .with_size(size, size)
        .build()
        .unwrap();

    let _base64_robohash = robo_hash.assemble_base64()?;

    // Save output
    // use std::fs::File;
    // use std::io::Write;
    // let mut output = File::create("robohash.txt")?;
    // write!(output, "{}", base64_robohash)?;

    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let initial_string = black_box("test");
    let use_background = black_box(&true);
    let size = black_box(512);

    c.bench_function("Build Robohash", |b| {
        b.iter(|| build_robohash(initial_string, use_background, size))
    });

    let size = black_box(256);
    c.bench_function("Build medium size Robohash", |b| {
        b.iter(|| build_robohash(initial_string, use_background, size))
    });

    let size = black_box(64);
    c.bench_function("Build small size Robohash", |b| {
        b.iter(|| build_robohash(initial_string, use_background, size))
    });

    let size = black_box(8);
    c.bench_function("Build tiny size Robohash", |b| {
        b.iter(|| build_robohash(initial_string, use_background, size))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
