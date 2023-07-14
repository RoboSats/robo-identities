use robohash::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_robohash(
    initial_string: &str,
    set: &str,
    color: &str,
    background_set: &str,
    size: u32,
) -> Result<(), Box<dyn Error>> {
    // build
    let robo_hash: RoboHash = RoboHashBuilder::new(initial_string)
        .with_set(set)
        .with_color(&color)
        .with_background_set(background_set)
        .with_size(size, size)
        .build()
        .unwrap();

    let base64_robohash = robo_hash.assemble_base64()?;

    // Save output
    // let mut output = File::create("robohash.txt")?;
    // write!(output, "{}", base64_robohash)?;

    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let initial_string = black_box("test");
    let set = black_box("set1");
    let color = black_box(String::from("red"));
    let background_set = black_box("bg1");
    let size = black_box(512);

    c.bench_function("Build Robohash", |b| {
        b.iter(|| build_robohash(initial_string, set, &color, background_set, size))
    });

    let size = black_box(256);
    c.bench_function("Build medium size Robohash", |b| {
        b.iter(|| build_robohash(initial_string, set, &color, background_set, size))
    });

    let size = black_box(64);
    c.bench_function("Build small size Robohash", |b| {
        b.iter(|| build_robohash(initial_string, set, &color, background_set, size))
    });

    let size = black_box(8);
    c.bench_function("Build tiny size Robohash", |b| {
        b.iter(|| build_robohash(initial_string, set, &color, background_set, size))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
