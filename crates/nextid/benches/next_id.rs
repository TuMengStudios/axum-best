use criterion::Criterion;

fn bench_next_id(c: &mut Criterion) {
    c.bench_function("next_id", |b| b.iter(nextid::next_id));
}

criterion::criterion_group!(benches, bench_next_id);
criterion::criterion_main!(benches);
