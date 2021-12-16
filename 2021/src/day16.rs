use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    payload: Payload,
}

impl Packet {
    fn for_each(&self, f: impl Fn(&Packet)) {
        f(self);
        match self.payload {
            Lit(_) => {}
            Op { id: _, ref packets } => {
                for p in packets {
                    f(p);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Payload {
    Lit(u64),
    Op { id: u64, packets: Vec<Packet> },
}
use Payload::*;

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> Vec<u8> {
    let bytes = input.trim().as_bytes();
    let mut hex = Vec::with_capacity(bytes.len());

    for b in bytes {
        let d = match *b {
            b'0' => 0x0,
            b'1' => 0x1,
            b'2' => 0x2,
            b'3' => 0x3,
            b'4' => 0x4,
            b'5' => 0x5,
            b'6' => 0x6,
            b'7' => 0x7,
            b'8' => 0x8,
            b'9' => 0x9,
            b'a' | b'A' => 0xa,
            b'b' | b'B' => 0xb,
            b'c' | b'C' => 0xc,
            b'd' | b'D' => 0xd,
            b'e' | b'E' => 0xe,
            b'f' | b'F' => 0xf,
            _ => unreachable!(),
        };

        assert!(d < 16);
        hex.extend(format!("{:04b}", d).as_bytes())
    }

    assert_eq!(bytes.len() * 4, hex.len());

    hex
}

struct PacketParser<'a> {
    bits: &'a [u8],
    offset: usize,
    depth: usize,
}

impl<'a> PacketParser<'a> {
    fn new(bits: &'a [u8]) -> Self {
        PacketParser { bits, offset: 0, depth: 0 }
    }

    fn skip(&mut self, n: u64) {
        println!("depth={depth} Skipping {} bits", n, depth=self.depth);
        self.offset += n as usize;
        assert!(self.offset < self.bits.len());
    }

    fn fix_num(&mut self, nbits: usize) -> u64 {
        let a = self.offset;
        let b = self.offset + nbits;
        println!("depth={depth} fix_num: ({}, {})", a, b, depth=self.depth);

        let s = std::str::from_utf8(&self.bits[a..b]).unwrap();
        let num = u64::from_str_radix(s, 2).unwrap();

        self.offset += nbits;
        num
    }

    fn var_num(&mut self) -> u64 {
        let mut num = 0;
        loop {
            let x = self.fix_num(5);
            num = (num << 4) | (x & 0b1111);

            if x & 0b1_0000 == 0 {
                break;
            }
        }

        num
    }

    fn all_packets(&mut self) -> Vec<Packet> {
        let mut packets = vec![];

        while let Some(p) = self.next_packet() {
            packets.push(p);
        }

        packets
    }

    fn next_packet(&mut self) -> Option<Packet> {
        self.depth += 1;

        if self.offset == self.bits.len() {
            return None;
        }
        let version = self.fix_num(3) as u8;
        let id = self.fix_num(3);

        println!("depth={depth} version={}, id={}", version, id, depth=self.depth);

        let payload;
        if id == 4 {
            payload = self.lit_payload();
        } else {
            payload = self.op_payload(id);
        }
        self.depth -= 1;

        Some(Packet { version, payload })
    }

    fn lit_payload(&mut self) -> Payload {
        println!("depth={depth} Parsing a Literal", depth=self.depth);
        Lit(self.var_num())
    }

    fn op_payload(&mut self, id: u64) -> Payload {
        self.depth += 1;

        let length_type_id = self.fix_num(1) as u8;
        println!("depth={depth} Parsing an Op w/ length_type_id={}", length_type_id, depth=self.depth);

        if length_type_id == 0 {
            let nbits = self.fix_num(15) as usize;

            let mut sub_parser = PacketParser::new(&self.bits[..nbits]);
            sub_parser.depth = self.depth;

            let packets = sub_parser.all_packets();

            self.depth -= 1;
            Op { id, packets }
        } else if length_type_id == 1 {
            let npackets = self.fix_num(11);

            let packets: Vec<_> = (0..npackets).map(|_| self.next_packet().unwrap()).collect();

            self.depth -= 1;
            Op { id, packets }
        } else {
            unreachable!("{} is not a valid length_type_id", length_type_id);
        }
    }
}

// Part1 ======================================================================
#[aoc(day16, part1)]
#[inline(never)]
pub fn part1(bits: &[u8]) -> u64 {
    let mut parser = PacketParser::new(bits);

    let mut packets = vec![];
    while let Some(p) = parser.next_packet() {
        packets.push(p);
    }

    dbg!(packets.len());

    0
}

// Part2 ======================================================================
#[aoc(day16, part2)]
#[inline(never)]
pub fn part2(bits: &[u8]) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod t {
    use super::*;

    use pretty_assertions::assert_eq;

    fn text_to_bits(s: &str) -> Vec<u8> {
        s.as_bytes().to_owned()
    }

    #[test]
    fn check_example_1_version_sum_16() {
        assert_eq!(part1(&parse_input("8A004A801A8002F478")), 16);
    }

    #[test]
    fn check_example_1_version_sum_12() {
        assert_eq!(part1(&parse_input("620080001611562C8802118E34")), 12);
    }

    #[test]
    fn check_example_1_version_sum_23() {
        assert_eq!(part1(&parse_input("C0015000016115A2E0802F182340")), 23);
    }

    #[test]
    fn check_example_1_version_sum_31() {
        assert_eq!(part1(&parse_input("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn check_example_1_id4_literal() {
        let bits = text_to_bits("110100101111111000101000");
        let mut parser = PacketParser::new(&bits);

        let expected = Packet {
            version: 6,
            payload: Lit(2021),
        };

        assert_eq!(Some(expected), parser.next_packet());
    }

    #[test]
    fn check_example_1_id6_op_with_2_subpackets() {
        let bits = text_to_bits("00111000000000000110111101000101001010010001001000000000");
        let mut parser = PacketParser::new(&bits);

        let packets = vec![];
        let expected = Packet {
            version: 1,
            payload: Op { id: 6, packets },
        };

        assert_eq!(Some(expected), parser.next_packet());
    }

    #[test]
    fn check_example_1_id6_op_with_3_subpackets() {
        let bits = text_to_bits("11101110000000001101010000001100100000100011000001100000");
        let mut parser = PacketParser::new(&bits);

        let packets = vec![
            Packet {
                version: 2,
                payload: Lit(1),
            },
            Packet {
                version: 4,
                payload: Lit(2),
            },
            Packet {
                version: 1,
                payload: Lit(3),
            },
        ];
        let expected = Packet {
            version: 7,
            payload: Op { id: 3, packets },
        };

        assert_eq!(Some(expected), parser.next_packet());
    }
}
