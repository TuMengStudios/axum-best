use axum_best::models;
use criterion::Criterion;

fn bench_random_user(c: &mut Criterion) {
    c.bench_function("random user", |b| {
        b.iter(
            || models::user::UserInfo::random(), // let _ = gen_valid_code(black_box(4));
        )
    });
}

criterion::criterion_group!(benches, bench_random_user);
criterion::criterion_main!(benches);
