use criterion::{black_box, criterion_group, criterion_main, Criterion};

enum SlotSource {
    SlotSubscriber,
    OrderSubscriber,
}

impl SlotSource {
    fn get_slot(&self) -> u64 {
        match self {
            SlotSource::SlotSubscriber => 1,
            SlotSource::OrderSubscriber => 2,
        }
    }
}

struct FooSubscriber {
    slot_source: SlotSource,
}

impl FooSubscriber {
    pub fn new(slot_source: SlotSource) -> Self {
        Self { slot_source }
    }

    pub fn some_function(&self) {
        let slot = self.slot_source.get_slot();
        black_box(slot); // Use black_box to prevent optimizations
    }
}

fn benchmark_enums(c: &mut Criterion) {
    let slot_subscriber = SlotSource::SlotSubscriber;
    let order_subscriber = SlotSource::OrderSubscriber;

    let foo1 = FooSubscriber::new(slot_subscriber);
    let foo2 = FooSubscriber::new(order_subscriber);

    c.bench_function("enums_slot_subscriber", |b| b.iter(|| foo1.some_function()));

    c.bench_function("enums_order_subscriber", |b| {
        b.iter(|| foo2.some_function())
    });
}

criterion_group!(benches, benchmark_enums);
criterion_main!(benches);
