
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sgbf_client::parsing::Parser;

pub fn criterion_benchmark(c: &mut Criterion) {
    let parser = Parser::default();
    let document = include_str!("../tests/data/calendar.html");
    c.bench_function("bench parse_calendar", |b| b.iter(|| parser.parse_calendar(black_box(String::from(document)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);