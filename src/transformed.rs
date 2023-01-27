use crate::common::{Data, Event, THRESHOLD};

pub fn func(es: &mut Vec<Event>, mut data: Data) -> usize {
  if data.num >= THRESHOLD {
    es.push(Event::A(data.cond));
    es.len() + data.num
  } else {
    data.cond = !data.cond;
    if data.cond {
      es.push(Event::B(data.num));
      data.num += 1;
      let tmp = gunc(es, data.num).num;
      tmp + 2
    } else {
      es.push(Event::C);
      data.num += 6;
      let tmp = func(es, data);
      tmp + 3
    }
  }
}

pub fn gunc(es: &mut Vec<Event>, num: usize) -> Data {
  let cond = es.len() % 2 == 0;
  if num >= THRESHOLD {
    es.push(Event::D);
    Data { num: es.len() | num, cond }
  } else {
    let data = Data { num: num + 2, cond };
    if es.len() < 5 {
      es.push(Event::E(es.len()));
      let tmp = func(es, data);
      Data { num: tmp + 3, cond }
    } else {
      let mut cond = es.len() % 3 > 0;
      if cond {
        let tmp = func(es, data.clone());
        cond = tmp % 2 == 0;
      }
      if cond {
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
  }
}
