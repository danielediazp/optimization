use rum::{rumdecoder, rumload, states::State};
use std::env;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut state = State::new();
    state.boot_up_instructions(instructions);
    rumdecoder::run(&mut state)
}
