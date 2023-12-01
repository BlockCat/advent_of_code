use std::iter::{repeat, repeat_with};

type Input = String;

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));
    // let input = input.trim_end_matches('0').to_string();
    println!("Ex1: {:?}", exercise_12(&input));
}

fn parse_input(input: &str) -> String {
    input
        .chars()
        .map(|x| match x {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' | 'a' => "1010",
            'B' | 'b' => "1011",
            'C' | 'c' => "1100",
            'D' | 'd' => "1101",
            'E' | 'e' => "1110",
            'F' | 'f' => "1111",
            _ => unreachable!(),
        })
        .collect()
}

fn exercise_12(input: &Input) -> (usize, usize) {
    let mut iterator = input.chars();

    let packet = decode_packet(&mut iterator).unwrap();
    // unreachable!()
    (sum_packet_version(&packet), execute(&packet))
}

fn decode_packet(iterator: &mut dyn Iterator<Item = char>) -> Option<Packet> {
    let version = iterator.take_number(3)?;
    let type_id = iterator.take_number(3).unwrap();

    if type_id == 4 {
        return Some(Packet::Literal(version, decode_literal(iterator)));
    } else {
        let packets = match iterator.take_type_length() {
            TypeLength::TotalBits(bits) => {
                let mut constrained_iterator = iterator.take(bits);
                repeat_with(|| decode_packet(&mut constrained_iterator))
                    .take_while(Option::is_some)
                    .filter_map(|x| x)
                    .collect()
            }
            TypeLength::TotalPackets(packets) => (0..packets)
                .map(|_| decode_packet(iterator).unwrap())
                .collect(),
        };

        return Some(Packet::Operator(version, type_id, packets));
    }
}

fn decode_literal(iterator: &mut dyn Iterator<Item = char>) -> usize {
    let mut iterator = iterator.chain(repeat('0'));
    let mut bits = Vec::with_capacity(4);
    while let Some('1') = iterator.next() {
        bits.extend(iterator.by_ref().take(4));
    }
    bits.extend(iterator.take(4));

    let bit_size = bits.len();
    bits.into_iter().take_number(bit_size).unwrap()
}

fn sum_packet_version(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(version, _) => *version,
        Packet::Operator(version, _, ch) => {
            *version + ch.iter().map(sum_packet_version).sum::<usize>()
        }
    }
}

fn execute(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(_, val) => *val,
        Packet::Operator(_, typ, ch) => match *typ {
            0 => ch.iter().map(execute).sum::<usize>(),
            1 => ch.iter().map(execute).product::<usize>(),
            2 => ch.iter().map(execute).min().unwrap(),
            3 => ch.iter().map(execute).max().unwrap(),
            5 => {
                let a = execute(&ch[0]);
                let b = execute(&ch[1]);
                usize::from(a > b)
            }
            6 => {
                let a = execute(&ch[0]);
                let b = execute(&ch[1]);
                usize::from(a < b)
            }
            7 => {
                let a = execute(&ch[0]);
                let b = execute(&ch[1]);
                
                usize::from(a == b)
            }
            _ => unreachable!(),
        },
    }
}

enum TypeLength {
    TotalBits(usize),
    TotalPackets(usize),
}

enum Packet {
    Literal(usize, usize),
    Operator(usize, usize, Vec<Packet>),
}

trait PacketIter: Iterator<Item = char> {
    fn take_number(self, bit_size: usize) -> Option<usize>
    where
        Self: Sized,
    {
        let mut bits = self.take(bit_size);
        if let Some(x) = bits.next() {
            let start = match x {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            };
            return Some(bits.fold(start, |acc, x| match x {
                '0' => acc * 2,
                '1' => acc * 2 + 1,
                _ => unreachable!(),
            }));
        } else {
            None
        }
    }

    fn take_type_length(mut self) -> TypeLength
    where
        Self: Sized,
    {
        match self.next() {
            Some('0') => TypeLength::TotalBits(self.take_number(15).unwrap()),
            Some('1') => TypeLength::TotalPackets(self.take_number(11).unwrap()),
            _ => unreachable!(),
        }
    }
}

impl<T> PacketIter for T where T: Iterator<Item = char> {}
