#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use wasm_lib;
    use test;

    #[bench]
    fn bench_ticks(b: &mut test::Bencher) {
        let mut creature = wasm_lib::Creature::new(1, 10);
        b.iter(|| creature.tick());
    }
}