pub mod clock;
mod command;
mod component;
mod core;
mod execution;
mod inspect;
mod risk;
mod state;
mod strategy;

pub trait Engine {
    type Core;
    type Components;
    type FeedTx;

    // TODO:
    // Optional Inspector or Seperate Engine with Inspector trait
    type Inspect;

    fn shutdown() -> Self;
    fn send(&self);
    fn update_state(&self);
    // fn wire_inspector(&mut self) -> Option<&Inspect>;
}
