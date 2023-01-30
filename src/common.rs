#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
  A(bool),
  B(usize),
  C,
  D,
  E(usize),
  F,
  G,
}

#[derive(Debug, Clone)]
pub struct Data {
  pub num: usize,
  pub cond: bool,
}

pub const THRESHOLD: usize = 50;
