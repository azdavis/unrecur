use crate::common::{Data, Event, THRESHOLD};

pub fn func(es: &mut Vec<Event>, mut data: Data) -> usize {
  if data.num >= THRESHOLD {
    es.push(Event::A(data.cond));
    return es.len() + data.num;
  }
  data.cond = !data.cond;
  if data.cond {
    es.push(Event::B(data.num));
    data.num += 1;
    gunc(es, data.num).num + 2
  } else {
    es.push(Event::C);
    data.num += 6;
    func(es, data) + 3
  }
}

pub fn gunc(es: &mut Vec<Event>, num: usize) -> Data {
  let cond = es.len() % 2 == 0;
  if num >= THRESHOLD {
    es.push(Event::D);
    return Data { num: es.len() | num, cond };
  }
  let data = Data { num: num + 2, cond };
  if es.len() < 5 {
    es.push(Event::E(es.len()));
    Data { num: func(es, data) + 3, cond }
  } else if es.len() % 3 > 0 && func(es, data) % 2 == 0 {
    es.push(Event::F);
    let mut tmp = gunc(es, num + 4);
    tmp.cond = !tmp.cond;
    tmp
  } else {
    es.push(Event::G);
    let fst = func(es, data);
    let mut tmp = gunc(es, fst);
    tmp.num += fst;
    tmp
  }
}
