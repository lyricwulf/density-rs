use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn roundtrip(c: &mut Criterion) {
    let data_len: usize = 1_048_576;
    let mut text = vec![0u8; data_len];
    let mut seeded_rng = StdRng::seed_from_u64(510152025);

    for i in text.iter_mut() {
        *i = seeded_rng.gen();
    }

    let text_bytes = &text[..];
    let text_length = text_bytes.len();

    let num_chunks_256 = text_length >> 8;
    let compress_safe_size = (num_chunks_256 + 1) * 320;

    let mut compressed_output = vec![0u8; compress_safe_size as usize];

    c.bench_function("Chameleon Compression", |b| {
        b.iter(|| {
            dens::compress_slice(
                black_box(text_bytes),
                black_box(&mut compressed_output),
                black_box(dens::Algorithm::Chameleon),
            );
        })
    });
}

criterion_group!(benches, roundtrip);
criterion_main!(benches);
