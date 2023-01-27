use crate::common::{Data, Event, THRESHOLD};
use crate::{original, transformed};
use std::time::{Duration, Instant};

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

fn with_duration<F, T>(f: F) -> (T, Duration)
where
  F: FnOnce() -> T,
{
  let start = Instant::now();
  let ret = f();
  (ret, Instant::now().duration_since(start))
}

#[test]
fn original_is_faster() {
  let ((r0, e0), t0) = with_duration(|| run(original::func, 0));
  let ((r1, e1), t1) = with_duration(|| run(transformed::func, 0));
  assert_eq!(r0, r1);
  assert_eq!(e0, e1);
  assert!(t0 < t1);
}
