use crate::common::{Data, Event};
use crate::original;

fn run(f: fn(&mut Vec<Event>, Data) -> usize, num: usize) -> (usize, Vec<Event>) {
  let mut es = Vec::<Event>::new();
  let ret = f(&mut es, Data { num, cond: false });
  (ret, es)
}

#[test]
fn smoke() {
  use Event::{A, B, C, D, E, F, G};
  let (ret, es) = run(original::func, 42);
  assert_eq!(
    es,
    [B(42), E(1), B(43), E(3), B(44), B(45), G, E(7), C, A(false), D(false), F, E(12), C, A(false)]
  );
  assert_eq!(45, ret);
}

// #[test]
// fn original_is_transformed() {
//   for num in 0usize..=THRESHOLD {
//     let (r0, e0) = run(original::func, num);
//     let (r1, e1) = run(transformed::func, num);
//     assert_eq!(r0, r1);
//     assert_eq!(e0, e1);
//   }
// }
