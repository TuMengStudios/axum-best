use std::hint::black_box;
use std::path::Path;

use axum_best::utils::file_digest;
use axum_best::utils::gen_valid_code;
use criterion::Criterion;

fn bench_gen_valid_code(c: &mut Criterion) {
    c.bench_function("gen_valid_code", |b| {
        b.iter(
            || gen_valid_code(black_box(4)), // let _ = gen_valid_code(black_box(4));
        )
    });
}

fn bench_file_digest(c: &mut Criterion) {
    use manifest_dir_macros::file_path;

    c.bench_function("file digest", |b| {
        b.iter(|| {
            let path = Path::new(file_path!("tests/index.txt"));
            file_digest(&path, axum_best::utils::HashAlgorithm::SHA1)
        });
    });

    c.bench_function("file digest md5", |b| {
        b.iter(|| {
            let path = Path::new(file_path!("tests/index.txt"));
            file_digest(&path, axum_best::utils::HashAlgorithm::MD5)
        });
    });
}

criterion::criterion_group!(benches, bench_gen_valid_code, bench_file_digest);
criterion::criterion_main!(benches);
