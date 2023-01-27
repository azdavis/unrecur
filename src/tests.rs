use crate::common::{Data, Event, THRESHOLD};
use crate::{original, transformed};

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

#[test]
fn original_is_transformed() {
  for num in 0usize..=THRESHOLD {
    let (r0, e0) = run(original::func, num);
    let (r1, e1) = run(transformed::func, num);
    assert_eq!(r0, r1);
    assert_eq!(e0, e1);
  }
}
