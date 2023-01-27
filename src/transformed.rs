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

fn hunc(arg: HuncArg<'_>) -> HuncRet {
  match arg {
    HuncArg::Func(es, mut data) => {
      let tmp = if data.num >= THRESHOLD {
        es.push(Event::A(data.cond));
        es.len() + data.num
      } else {
        data.cond = !data.cond || es.len() % 5 == 0;
        if data.cond {
          es.push(Event::B(data.num));
          data.num += 1;
          gunc(es, &mut data);
          data.num + 2
        } else {
          es.push(Event::C);
          data.num += 6;
          let tmp = func(es, data);
          tmp + 5
        }
      };
      HuncRet::Func(tmp)
    }
    HuncArg::Gunc(es, data) => {
      if data.num >= THRESHOLD {
        es.push(Event::D(data.cond));
      } else {
        data.cond = !data.cond || es.len() % 7 == 0;
        if es.len() < 5 || data.cond {
          es.push(Event::E(es.len()));
          let tmp = func(es, *data);
          data.num = tmp + 3;
        } else {
          let mut cond = es.len() % 3 > 0;
          if cond {
            let tmp = func(es, *data);
            cond = tmp % 2 == 0;
          }
          if cond {
            es.push(Event::F);
            data.num += 4;
            gunc(es, data);
            data.num += 6;
          } else {
            es.push(Event::G);
            data.num += 3;
            gunc(es, data);
            data.num += 2;
          }
        }
      }
      HuncRet::Gunc
    }
  }
}
