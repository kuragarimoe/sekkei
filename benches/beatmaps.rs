use criterion::*;
use sekkei::parser::beatmap::BeatmapFile;
use std::{borrow::Borrow, env, fs, time::Duration};

#[tokio::main]
async fn criterion_benchmark(c: &mut Criterion) {
    // preconfigure
    let mut b = c.benchmark_group("sekkai-parser");
    let path = env::current_dir().unwrap().to_str().unwrap().to_string()
        + "/tests/files/IMAGINARY LIKE THE JUSTICE.osu";

    b.warm_up_time(Duration::new(10, 0));
    b.throughput(Throughput::Bytes(fs::metadata(path).unwrap().len() as u64));
    
    // run benches
    b.bench_function("test unpack", |b| {
        b.iter(|| {
            let path2 = env::current_dir().unwrap().to_str().unwrap().to_string()
                + "/tests/files/IMAGINARY LIKE THE JUSTICE.osu";
            let bm = BeatmapFile::from_file(&path2); // unwrap
        })
    });

    // done
    b.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
