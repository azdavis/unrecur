use crate::common::{Data, Event};
use crate::original;

fn run(f: fn(&mut Vec<Event>, Data) -> usize, num: usize) -> (usize, Vec<Event>) {
  let mut es = Vec::<Event>::new();
  let ret = f(&mut es, Data { num, cond: false });
  (ret, es)
}

#[test]
fn original_37() {
  use Event::{A, B, C, E, F, G};
  let (ret, es) = run(original::func, 37);
  assert_eq!(
    es,
    [B(37), E(1), B(38), E(3), B(39), B(40), G, E(7), C, A(false), F, E(11), C, A(false)]
  );
  assert_eq!(ret, 90);
}
