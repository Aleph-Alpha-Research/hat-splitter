use divan::Bencher;

use hat_splitter::{Splitter, HATSplitter};

use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn hamlet_text() -> &'static str {
    include_str!("../data/shakespeare.txt")
}

#[divan::bench]
fn split(bencher: Bencher) {
    let splitter = HATSplitter::new();

    bencher.bench(|| splitter.split(hamlet_text()));
}

