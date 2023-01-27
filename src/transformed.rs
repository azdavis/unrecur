use crate::common::{Data, Event, THRESHOLD};

pub fn func(es: &mut Vec<Event>, data: Data) -> usize {
  let tmp = hunc(HuncArg::Func(es, data));
  tmp.unwrap_func()
}

pub fn gunc(es: &mut Vec<Event>, data: &mut Data) {
  let tmp = hunc(HuncArg::Gunc(es, data));
  tmp.unwrap_gunc();
}

enum HuncArg<'a> {
  Func(&'a mut Vec<Event>, Data),
  Gunc(&'a mut Vec<Event>, &'a mut Data),
}

enum HuncRet {
  Func(usize),
  Gunc,
}

impl HuncRet {
  fn unwrap_func(self) -> usize {
    match self {
      HuncRet::Func(x) => x,
      HuncRet::Gunc => unreachable!(),
    }
  }

  fn unwrap_gunc(self) {
    match self {
      HuncRet::Func(_) => unreachable!(),
      HuncRet::Gunc => {}
    }
  }
}

enum HuncCont {
  C1(Data),
}

impl HuncCont {
  fn run(self, tmp: HuncRet) -> HuncRet {
    match self {
      HuncCont::C1(data) => {
        tmp.unwrap_gunc();
        HuncRet::Func(data.num + 2)
      }
    }
  }
}

fn hunc(arg: HuncArg<'_>) -> HuncRet {
  match arg {
    HuncArg::Func(es, mut data) => {
      if data.num >= THRESHOLD {
        es.push(Event::A(data.cond));
        HuncRet::Func(es.len() + data.num)
      } else {
        data.cond = !data.cond || es.len() % 5 == 0;
        if data.cond {
          es.push(Event::B(data.num));
          data.num += 1;
          let tmp = hunc(HuncArg::Gunc(es, &mut data));
          HuncCont::C1(data).run(tmp)
        } else {
          es.push(Event::C);
          data.num += 6;
          let tmp = hunc(HuncArg::Func(es, data));
          let tmp = tmp.unwrap_func();
          HuncRet::Func(tmp + 5)
        }
      }
    }
    HuncArg::Gunc(es, data) => {
      if data.num >= THRESHOLD {
        es.push(Event::D(data.cond));
        HuncRet::Gunc
      } else {
        data.cond = !data.cond || es.len() % 7 == 0;
        if es.len() < 5 || data.cond {
          es.push(Event::E(es.len()));
          let tmp = hunc(HuncArg::Func(es, *data));
          let tmp = tmp.unwrap_func();
          data.num = tmp + 3;
          HuncRet::Gunc
        } else {
          let mut cond = es.len() % 3 > 0;
          if cond {
            let tmp = hunc(HuncArg::Func(es, *data));
            let tmp = tmp.unwrap_func();
            cond = tmp % 2 == 0;
          }
          if cond {
            es.push(Event::F);
            data.num += 4;
            let tmp = hunc(HuncArg::Gunc(es, data));
            tmp.unwrap_gunc();
            data.num += 6;
            HuncRet::Gunc
          } else {
            es.push(Event::G);
            data.num += 3;
            let tmp = hunc(HuncArg::Gunc(es, data));
            tmp.unwrap_gunc();
            data.num += 2;
            HuncRet::Gunc
          }
        }
      }
    }
  }
}
