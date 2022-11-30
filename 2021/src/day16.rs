#![allow(clippy::needless_late_init)]

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    payload: Payload,
    offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Payload {
    Lit(u64),
    Op { id: u64, args: Vec<Packet> },
}
use Payload::*;

impl Packet {
    fn visit(&self, mut f: impl FnMut(&Packet)) {
        fn helper(p: &Packet, ff: &mut impl FnMut(&Packet)) {
            ff(p);
            match &p.payload {
                Lit(_) => {}
                Op { args, .. } => {
                    for arg in args.iter() {
                        helper(arg, ff);
                    }
                }
            }
        }

        helper(self, &mut f);
    }

    fn eval(&self) -> i64 {
        match &self.payload {
            Lit(num) => *num as i64,
            Op { id, args, .. } => {
                match id {
                    // sum
                    0 => args.iter().map(Self::eval).sum(),

                    // product
                    1 => args.iter().map(Self::eval).product(),

                    // min
                    2 => args.iter().map(Self::eval).min().unwrap(),

                    // max
                    3 => args.iter().map(Self::eval).max().unwrap(),

                    // greater-than
                    5 => {
                        assert_eq!(args.len(), 2);
                        (args[0].eval() > args[1].eval()) as i64
                    }

                    // less-than
                    6 => {
                        assert_eq!(args.len(), 2);
                        (args[0].eval() < args[1].eval()) as i64
                    }

                    // equal-to
                    7 => {
                        assert_eq!(args.len(), 2);
                        (args[0].eval() == args[1].eval()) as i64
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

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

    depth: u64,
}

impl<'a> PacketParser<'a> {
    fn new(bits: &'a [u8]) -> Self {
        PacketParser {
            bits,
            offset: 0,

            depth: 0,
        }
    }

    fn offset(&self) -> u64 {
        self.offset as u64
    }

    fn fix_num(&mut self, nbits: usize) -> u64 {
        let a = self.offset;
        let b = self.offset + nbits;
        let s = std::str::from_utf8(&self.bits[a..b]).unwrap();
        let num = u64::from_str_radix(s, 2).unwrap();

        self.offset += nbits;
        num
    }

    fn var_num(&mut self) -> u64 {
        let _a = self.offset();

        let mut num = 0;
        loop {
            let x = self.fix_num(5);
            num = (num << 4) | (x & 0b1111);

            if x & 0b1_0000 == 0 {
                break;
            }
        }

        let _b = self.offset();

        num
    }

    fn parse(&mut self) -> Packet {
        self.depth += 1;
        let offset = self.offset();

        let version = self.fix_num(3) as u8;
        let id = self.fix_num(3);

        let payload;
        if id == 4 {
            payload = self.parse_lit();
        } else {
            payload = self.parse_op(id);
        }

        self.depth -= 1;

        Packet {
            version,
            offset,
            payload,
        }
    }

    fn parse_lit(&mut self) -> Payload {
        Lit(self.var_num())
    }

    fn parse_op(&mut self, id: u64) -> Payload {
        let length_type_id = self.fix_num(1) as u8;

        if length_type_id == 0 {
            let nbits = self.fix_num(15) as usize;

            let mut args = vec![];
            let target = self.offset + nbits;
            while self.offset < target {
                args.push(self.parse());
            }

            Op { id, args }
        } else if length_type_id == 1 {
            let num_packets = self.fix_num(11);

            let mut args = vec![];
            for _ in 0..num_packets {
                args.push(self.parse());
            }

            Op { id, args }
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
    let packet = parser.parse();

    let mut sum = 0;
    packet.visit(|p| sum += p.version as u64);

    sum
}

// Part2 ======================================================================
#[aoc(day16, part2)]
#[inline(never)]
pub fn part2(bits: &[u8]) -> i64 {
    let mut parser = PacketParser::new(bits);
    let packet = parser.parse();

    packet.eval()
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
            offset: 0,
            version: 6,
            payload: Lit(2021),
        };

        assert_eq!(expected, parser.parse());
    }

    #[test]
    fn check_example_1_id6_op_with_2_subpackets() {
        let bits = text_to_bits("00111000000000000110111101000101001010010001001000000000");
        let mut parser = PacketParser::new(&bits);

        let expected = Packet {
            offset: 0,
            version: 1,
            payload: Op {
                id: 6,
                args: vec![
                    Packet {
                        version: 6,
                        payload: Lit(10),
                        offset: 22,
                    },
                    Packet {
                        version: 2,
                        payload: Lit(20),
                        offset: 33,
                    },
                ],
            },
        };

        assert_eq!(expected, parser.parse());
    }

    #[test]
    fn check_example_1_id6_op_with_3_subpackets() {
        let bits = text_to_bits("11101110000000001101010000001100100000100011000001100000");
        let mut parser = PacketParser::new(&bits);

        let expected = Packet {
            offset: 0,
            version: 7,
            payload: Op {
                id: 3,
                args: vec![
                    Packet {
                        offset: 18,
                        version: 2,
                        payload: Lit(1),
                    },
                    Packet {
                        offset: 29,
                        version: 4,
                        payload: Lit(2),
                    },
                    Packet {
                        offset: 40,
                        version: 1,
                        payload: Lit(3),
                    },
                ],
            },
        };

        assert_eq!(expected, parser.parse());
    }

    #[test]
    fn check_example_2_eval_a() {}

    #[test]
    fn check_example_2_eval_0() {
        assert_eq!(part2(&parse_input("C200B40A82")), 3);
    }

    #[test]
    fn check_example_2_eval_1() {
        assert_eq!(part2(&parse_input("04005AC33890")), 54);
    }

    #[test]
    fn check_example_2_eval_2() {
        assert_eq!(part2(&parse_input("880086C3E88112")), 7);
    }

    #[test]
    fn check_example_2_eval_3() {
        assert_eq!(part2(&parse_input("CE00C43D881120")), 9);
    }

    #[test]
    fn check_example_2_eval_4() {
        assert_eq!(part2(&parse_input("D8005AC2A8F0")), 1);
    }

    #[test]
    fn check_example_2_eval_5() {
        assert_eq!(part2(&parse_input("F600BC2D8F")), 0);
    }

    #[test]
    fn check_example_2_eval_6() {
        assert_eq!(part2(&parse_input("9C005AC2F8F0")), 0);
    }

    #[test]
    fn check_example_2_eval_7() {
        let bits = parse_input("9C0141080250320F1802104A08");
        let mut parser = PacketParser::new(&bits);

        // 1 + 3 == 2 * 2
        let expected = Packet {
            offset: 0,
            version: 4,
            payload: Op {
                id: 7,
                args: vec![
                    Packet {
                        offset: 22,
                        version: 2,
                        payload: Op {
                            id: 0,
                            args: vec![
                                Packet {
                                    offset: 40,
                                    version: 2,
                                    payload: Lit(1),
                                },
                                Packet {
                                    offset: 51,
                                    version: 4,
                                    payload: Lit(3),
                                },
                            ],
                        },
                    },
                    Packet {
                        offset: 62,
                        version: 6,
                        payload: Op {
                            id: 1,
                            args: vec![
                                Packet {
                                    offset: 80,
                                    version: 0,
                                    payload: Lit(2),
                                },
                                Packet {
                                    offset: 91,
                                    version: 2,
                                    payload: Lit(2),
                                },
                            ],
                        },
                    },
                ],
            },
        };

        assert_eq!(expected, parser.parse());
        assert_eq!(part2(&bits), 1);
    }
}
