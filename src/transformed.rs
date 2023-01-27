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

enum Cont {
  C1,
  C2,
  C3(bool),
  C4(Data, usize),
  C5,
  C6,
  C7(usize),
}

fn hunc(es: &mut Vec<Event>, mut arg: Arg) -> Ret {
  let mut cs = Vec::<Cont>::new();
  'outer: loop {
    let mut ret = match arg {
      Arg::Func(mut data) => {
        if data.num >= THRESHOLD {
          es.push(Event::A(data.cond));
          Ret::Func(es.len() + data.num)
        } else {
          data.cond = !data.cond;
          if data.cond {
            es.push(Event::B(data.num));
            data.num += 1;
            cs.push(Cont::C1);
            arg = Arg::Gunc(data.num);
            continue;
          }
          es.push(Event::C);
          data.num += 6;
          cs.push(Cont::C2);
          arg = Arg::Func(data);
          continue;
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
            cs.push(Cont::C3(cond));
            arg = Arg::Func(data);
            continue;
          }
          let cond = es.len() % 3 > 0;
          if cond {
            cs.push(Cont::C4(data.clone(), num));
            arg = Arg::Func(data);
            continue;
          }
          arg = post_if_c4(&mut cs, es, data, num, cond);
          continue;
        }
      }
    };
    while let Some(cont) = cs.pop() {
      match cont {
        Cont::C1 => {
          let tmp = ret.unwrap_gunc().num;
          ret = Ret::Func(tmp + 2);
        }
        Cont::C2 => {
          let tmp = ret.unwrap_func();
          ret = Ret::Func(tmp + 3);
        }
        Cont::C3(cond) => {
          let tmp = ret.unwrap_func();
          ret = Ret::Gunc(Data { num: tmp + 3, cond });
        }
        Cont::C4(data, num) => {
          let tmp = ret.unwrap_func();
          let cond = tmp % 2 == 0;
          arg = post_if_c4(&mut cs, es, data, num, cond);
          continue 'outer;
        }
        Cont::C5 => {
          let mut tmp = ret.unwrap_gunc();
          tmp.cond = !tmp.cond;
          ret = Ret::Gunc(tmp);
        }
        Cont::C6 => {
          let fst = ret.unwrap_func();
          cs.push(Cont::C7(fst));
          arg = Arg::Gunc(fst);
          continue 'outer;
        }
        Cont::C7(fst) => {
          let mut tmp = ret.unwrap_gunc();
          tmp.num += fst;
          ret = Ret::Gunc(tmp);
        }
      }
    }
    return ret;
  }
}

fn post_if_c4(cs: &mut Vec<Cont>, es: &mut Vec<Event>, data: Data, num: usize, cond: bool) -> Arg {
  if cond {
    es.push(Event::F);
    cs.push(Cont::C5);
    Arg::Gunc(num + 4)
  } else {
    es.push(Event::G);
    cs.push(Cont::C6);
    Arg::Func(data)
  }
}
