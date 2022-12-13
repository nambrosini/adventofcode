use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day23, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut computers: Vec<Computer> = Vec::new();

    // init pcs
    for i in 0..50 {
        let mut pc = Computer::new(input);
        pc.run(Some(i));
        computers.push(pc);
    }

    // init network
    let mut network_queue: Vec<Packet> = Vec::new();

    loop {
        for pc in computers.iter_mut() {
            let val = if let Some(pos) = network_queue
                .iter()
                .position(|p| p.receiver == pc.id && p.complete)
            {
                let val = network_queue[pos].take_next();
                if network_queue[pos].is_done() {
                    network_queue.remove(pos);
                }
                Some(val)
            } else {
                None
            };

            if let Some(out) = pc.run(val) {
                let mut added = false;
                for i in (0..network_queue.len()).rev() {
                    if network_queue[i].sender == pc.id && !network_queue[i].complete {
                        network_queue[i].add_value(out);
                        if network_queue[i].receiver == 255 && network_queue[i].complete {
                            return network_queue[i].y.unwrap();
                        }
                        added = true;
                        break;
                    }
                }
                if !added {
                    let packet = Packet::new(pc.id, out);
                    network_queue.push(packet);
                }
            }
        }
    }
}

#[aoc(day23, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut computers: Vec<Computer> = Vec::new();

    // init pcs
    for i in 0..50 {
        let mut pc = Computer::new(input);
        pc.run(Some(i));
        computers.push(pc);
    }

    // init network
    let mut network_queue: Vec<Packet> = Vec::new();
    let mut nat = Nat::new();
    let mut last_nat = i64::MIN;

    loop {
        for pc in computers.iter_mut() {
            let val = if let Some(pos) = network_queue
                .iter()
                .position(|p| p.receiver == pc.id && p.complete)
            {
                let val = network_queue[pos].take_next();
                if network_queue[pos].is_done() {
                    network_queue.remove(pos);
                }
                Some(val)
            } else {
                None
            };

            if let Some(out) = pc.run(val) {
                let mut added = false;
                for i in (0..network_queue.len()).rev() {
                    if network_queue[i].sender == pc.id && !network_queue[i].complete {
                        network_queue[i].add_value(out);
                        added = true;
                        break;
                    }
                }
                if !added {
                    let packet = Packet::new(pc.id, out);
                    network_queue.push(packet);
                }
            }
        }

        if let Some(pos) = network_queue
            .iter()
            .position(|p| p.receiver == 255 && p.complete)
        {
            let p = network_queue.remove(pos);

            nat.receive_new_packet(p);
        }

        if let Some(p) = nat.check_if_idle(&computers, &network_queue) {
            println!("{:?}", p);
            if p.y == Some(last_nat) {
                return p.y.unwrap();
            } else {
                last_nat = p.y.unwrap();
                network_queue.push(p);
            }
        }
    }
}

struct Nat {
    last_packet: Option<Packet>,
}

impl Nat {
    fn new() -> Self {
        Self { last_packet: None }
    }

    fn receive_new_packet(&mut self, packet: Packet) {
        self.last_packet = Some(packet);
    }

    fn check_if_idle(&mut self, computers: &[Computer], network: &[Packet]) -> Option<Packet> {
        if !network.is_empty() {
            return None;
        }

        for pc in computers {
            if !pc.incoming.is_empty() {
                return None;
            }
        }

        if let Some(p) = self.last_packet.take() {
            let p = Packet {
                sender: 255,
                receiver: 0,
                x: p.x,
                y: p.y,
                complete: true,
            };
            return Some(p);
        }

        None
    }
}

struct Computer {
    memory: HashMap<i64, i64>,
    position: i64,
    relative_base: i64,
    id: i64,
    incoming: Vec<i64>,
}

impl Computer {
    fn new(memory: &[i64]) -> Computer {
        let mut map = HashMap::new();

        for (i, e) in memory.iter().enumerate() {
            map.insert(i as i64, *e);
        }

        Self {
            memory: map,
            position: 0,
            relative_base: 0,
            id: -1,
            incoming: Vec::new(),
        }
    }

    fn run(&mut self, input: Option<i64>) -> Option<i64> {
        if let Some(x) = input {
            self.incoming.push(x);
        }
        let op: Operation = self.memory[&self.position].into();
        match op {
            Operation::Add(m1, m2, m3) => {
                let v1 = self.get_mem(1, m1);
                let v2 = self.get_mem(2, m2);
                self.set_mem(3, v1 + v2, m3);
                self.position += 4;
            }
            Operation::Mul(m1, m2, m3) => {
                let v1 = self.get_mem(1, m1);
                let v2 = self.get_mem(2, m2);
                self.set_mem(3, v1 * v2, m3);
                self.position += 4;
            }
            Operation::Save(m1) => {
                let v = if !self.incoming.is_empty() {
                    self.incoming.remove(0)
                } else {
                    -1
                };
                if self.id == -1 {
                    self.id = v;
                }
                self.set_mem(1, v, m1);
                self.position += 2;
            }
            Operation::Out(m1) => {
                let output = self.get_mem(1, m1);
                self.position += 2;
                return Some(output);
            }
            Operation::Jit(m1, m2) => {
                let p1 = self.get_mem(1, m1);
                let p2 = self.get_mem(2, m2);

                if p1 != 0 {
                    self.position = p2;
                } else {
                    self.position += 3;
                }
            }
            Operation::Jif(m1, m2) => {
                let p1 = self.get_mem(1, m1);
                let p2 = self.get_mem(2, m2);

                if p1 == 0 {
                    self.position = p2;
                } else {
                    self.position += 3;
                }
            }
            Operation::Lt(m1, m2, m3) => {
                let p1 = self.get_mem(1, m1);
                let p2 = self.get_mem(2, m2);

                if p1 < p2 {
                    self.set_mem(3, 1, m3);
                } else {
                    self.set_mem(3, 0, m3);
                }
                self.position += 4;
            }
            Operation::Eq(m1, m2, m3) => {
                let p1 = self.get_mem(1, m1);
                let p2 = self.get_mem(2, m2);

                if p1 == p2 {
                    self.set_mem(3, 1, m3);
                } else {
                    self.set_mem(3, 0, m3);
                }
                self.position += 4;
            }
            Operation::Rb(m1) => {
                let p1 = self.get_mem(1, m1);
                self.relative_base += p1;
                self.position += 2;
            }
            Operation::Exit => {
                self.position += 1;
            }
        }

        None
    }

    fn get_mem(&mut self, offset: i64, mode: Mode) -> i64 {
        let index = self.get_index(offset, mode);
        let entry = self.memory.entry(index).or_insert(0);
        *entry
    }

    fn set_mem(&mut self, offset: i64, value: i64, mode: Mode) {
        let index = self.get_index(offset, mode);
        let entry = self.memory.entry(index).or_insert(0);
        *entry = value
    }

    fn get_index(&self, offset: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Position => self.memory[&(self.position + offset)],
            Mode::Immediate => self.position + offset,
            Mode::Relative => self.memory[&(self.position + offset)] + self.relative_base,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save(Mode),
    Out(Mode),
    Exit,
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    Rb(Mode),
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Operation {
    fn from(i: i64) -> Self {
        let code = i % 100;
        let m1 = (i / 100) % 10;
        let m2 = (i / 1000) % 10;
        let m3 = (i / 10000) % 10;

        match code {
            1 => Self::Add(m1.into(), m2.into(), m3.into()),
            2 => Self::Mul(m1.into(), m2.into(), m3.into()),
            3 => Self::Save(m1.into()),
            4 => Self::Out(m1.into()),
            5 => Self::Jit(m1.into(), m2.into()),
            6 => Self::Jif(m1.into(), m2.into()),
            7 => Self::Lt(m1.into(), m2.into(), m3.into()),
            8 => Self::Eq(m1.into(), m2.into(), m3.into()),
            9 => Self::Rb(m1.into()),
            99 => Self::Exit,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for Mode {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Packet {
    sender: i64,
    receiver: i64,
    x: Option<i64>,
    y: Option<i64>,
    complete: bool,
}

impl Packet {
    fn new(sender: i64, receiver: i64) -> Self {
        Self {
            sender,
            receiver,
            x: None,
            y: None,
            complete: false,
        }
    }

    fn take_next(&mut self) -> i64 {
        if let Some(x) = self.x.take() {
            return x;
        }
        if let Some(y) = self.y.take() {
            return y;
        }

        unreachable!()
    }

    fn is_done(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    fn add_value(&mut self, value: i64) {
        if self.x.is_none() {
            self.x = Some(value);
            return;
        }

        if self.y.is_none() {
            self.y = Some(value);
            self.complete = true;
            return;
        }

        unreachable!();
    }
}
