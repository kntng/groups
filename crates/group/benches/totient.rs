use criterion::{Criterion, black_box, criterion_group, criterion_main};
use group::const_utils::const_totient;
use group::utils::euler_totient;

fn bench_totient(c: &mut Criterion) {
    let mut totient = c.benchmark_group("euler_totient");
    totient.bench_function("runtime", move |b| {
        b.iter(|| {
            for n in 1..=10_000 {
                euler_totient(black_box(n));
            }
        });
    });

    totient.bench_function("compiletime", move |b| {
        b.iter(|| {
            for n in 1..=10_000 {
                const_totient(n);
            }
        });
    });

    totient.finish();
}

criterion_group!(benches, bench_totient);
criterion_main!(benches);
