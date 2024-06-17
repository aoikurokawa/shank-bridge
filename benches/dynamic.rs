use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait SlotSourceTrait {
    fn get_slot(&self) -> u64;
}

struct SlotSubscriber;

impl SlotSourceTrait for SlotSubscriber {
    fn get_slot(&self) -> u64 {
        1
    }
}

struct OrderSubscriber;

impl SlotSourceTrait for OrderSubscriber {
    fn get_slot(&self) -> u64 {
        2
    }
}

struct FooSubscriberTrait {
    slot_source: Box<dyn SlotSourceTrait>,
}

impl FooSubscriberTrait {
    pub fn new(slot_source: Box<dyn SlotSourceTrait>) -> Self {
        Self { slot_source }
    }

    pub fn some_function(&self) {
        let slot = self.slot_source.get_slot();
        black_box(slot); // Use black_box to prevent optimizations
    }
}

fn benchmark_traits_dynamic(c: &mut Criterion) {
    let slot_subscriber = SlotSubscriber;
    let order_subscriber = OrderSubscriber;

    let foo1 = FooSubscriberTrait::new(Box::new(slot_subscriber));
    let foo2 = FooSubscriberTrait::new(Box::new(order_subscriber));

    c.bench_function("traits_slot_subscriber", |b| {
        b.iter(|| foo1.some_function())
    });

    c.bench_function("traits_order_subscriber", |b| {
        b.iter(|| foo2.some_function())
    });
}

criterion_group!(benches, benchmark_traits_dynamic);
criterion_main!(benches);
