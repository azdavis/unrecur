use crate::common::{Data, Event, THRESHOLD};

pub fn func(es: &mut Vec<Event>, mut data: Data) -> usize {
  if data.num >= THRESHOLD {
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
  }
}

pub fn gunc(es: &mut Vec<Event>, data: &mut Data) {
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
}

enum HuncArg<'a> {
  Func(&'a mut Vec<Event>, Data),
  Gunc(&'a mut Vec<Event>, &'a mut Data),
}

enum HuncRet {
  Func(usize),
  Gunc,
}

fn hunc(arg: HuncArg<'_>) -> HuncRet {
  match arg {
    HuncArg::Func(es, data) => {
      let tmp = func(es, data);
      HuncRet::Func(tmp)
    }
    HuncArg::Gunc(es, data) => {
      gunc(es, data);
      HuncRet::Gunc
    }
  }
}
