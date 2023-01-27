use crate::common::{Data, Event, THRESHOLD};

pub fn func(es: &mut Vec<Event>, mut data: Data) -> usize {
  if data.num >= THRESHOLD {
    es.push(Event::A(data.cond));
    return es.len() + data.num;
  }
  data.cond = !data.cond || es.len() % 5 == 0;
  if data.cond {
    es.push(Event::B(data.num));
    data.num += 1;
    gunc(es, data);
    data.num + 2
  } else {
    es.push(Event::C);
    data.num += 6;
    func(es, data) + 5
  }
}

pub fn gunc(es: &mut Vec<Event>, mut data: Data) -> usize {
  if data.num >= THRESHOLD {
    es.push(Event::D(data.cond));
    return es.len() | data.num;
  }
  data.cond = !data.cond || es.len() % 7 == 0;
  if es.len() < 5 || data.cond {
    es.push(Event::E(es.len()));
    func(es, data) + 3
  } else if es.len() % 3 > 0 && func(es, data) % 2 == 0 {
    es.push(Event::F);
    data.num += 4;
    gunc(es, data) + 6
  } else {
    es.push(Event::G);
    data.num += 3;
    let fst = gunc(es, data);
    data.num += 2;
    let snd = gunc(es, data);
    fst + snd
  }
}
