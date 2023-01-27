use crate::common::{Data, Event, THRESHOLD};

pub fn func(es: &mut Vec<Event>, data: Data) -> usize {
  hunc(es, Arg::Func(data)).unwrap_func()
}

pub fn gunc(es: &mut Vec<Event>, num: usize) -> Data {
  hunc(es, Arg::Gunc(num)).unwrap_gunc()
}

enum Arg {
  Func(Data),
  Gunc(usize),
}

enum Ret {
  Func(usize),
  Gunc(Data),
}

impl Ret {
  fn unwrap_func(self) -> usize {
    match self {
      Ret::Func(ret) => ret,
      Ret::Gunc(_) => unreachable!(),
    }
  }

  fn unwrap_gunc(self) -> Data {
    match self {
      Ret::Func(_) => unreachable!(),
      Ret::Gunc(ret) => ret,
    }
  }
}

enum Cont {}

fn hunc(es: &mut Vec<Event>, arg: Arg) -> Ret {
  let mut cs = Vec::<Cont>::new();
  let ret = match arg {
    Arg::Func(mut data) => {
      if data.num >= THRESHOLD {
        es.push(Event::A(data.cond));
        Ret::Func(es.len() + data.num)
      } else {
        data.cond = !data.cond;
        if data.cond {
          es.push(Event::B(data.num));
          data.num += 1;
          let tmp = hunc(es, Arg::Gunc(data.num)).unwrap_gunc().num;
          Ret::Func(tmp + 2)
        } else {
          es.push(Event::C);
          data.num += 6;
          let tmp = hunc(es, Arg::Func(data)).unwrap_func();
          Ret::Func(tmp + 3)
        }
      }
    }
    Arg::Gunc(num) => {
      let cond = es.len() % 2 == 0;
      if num >= THRESHOLD {
        es.push(Event::D);
        Ret::Gunc(Data { num: es.len() | num, cond })
      } else {
        let data = Data { num: num + 2, cond };
        if es.len() < 5 {
          es.push(Event::E(es.len()));
          let tmp = hunc(es, Arg::Func(data)).unwrap_func();
          Ret::Gunc(Data { num: tmp + 3, cond })
        } else {
          let mut cond = es.len() % 3 > 0;
          if cond {
            let tmp = hunc(es, Arg::Func(data.clone())).unwrap_func();
            cond = tmp % 2 == 0;
          }
          if cond {
            es.push(Event::F);
            let mut tmp = hunc(es, Arg::Gunc(num + 4)).unwrap_gunc();
            tmp.cond = !tmp.cond;
            Ret::Gunc(tmp)
          } else {
            es.push(Event::G);
            let fst = hunc(es, Arg::Func(data)).unwrap_func();
            let mut tmp = hunc(es, Arg::Gunc(fst)).unwrap_gunc();
            tmp.num += fst;
            Ret::Gunc(tmp)
          }
        }
      }
    }
  };
  while let Some(cont) = cs.pop() {
    match cont {}
  }
  ret
}
