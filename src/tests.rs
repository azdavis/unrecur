use crate::common::{Data, Event, THRESHOLD};
use crate::{original, transformed};

fn run(f: fn(&mut Vec<Event>, Data) -> usize, num: usize) -> (usize, Vec<Event>) {
  let mut es = Vec::<Event>::new();
  let ret = f(&mut es, Data { num, cond: false });
  (ret, es)
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
