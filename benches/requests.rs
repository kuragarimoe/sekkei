use sekkei::request::{Method, Request};

use criterion::{criterion_group, criterion_main, Criterion};
use serde_json::json;
use tokio::runtime::Runtime;

#[tokio::main]
async fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test http request", |b| {
        b.iter(|| async {
            let mut request = Request {
                url: "http://localhost:9898/test".to_string(),

                body: Some(json!({
                    "test": "hi"
                })),

                headers: Some(json!({
                    "test": "hi"
                })),

                method: Method::Get,
            };

            request.make().await.unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
