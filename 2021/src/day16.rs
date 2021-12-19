use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    payload: Payload,
    offset: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Payload {
    Lit(u64),
    Op { id: u64, num_packets: u64 },
}
use Payload::*;

impl Packet {
    fn id(&self) -> u64 {
        match self.payload {
            Lit(_) => 4,
            Op { id, .. } => id,
        }
    }

    fn num_packets(&self) -> u64 {
        match self.payload {
            Lit(_) => 0,
            Op { num_packets, .. } => num_packets,
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
    packet_offsets: Vec<u64>,
}

impl<'a> PacketParser<'a> {
    fn new(bits: &'a [u8]) -> Self {
        PacketParser {
            bits,
            offset: 0,
            packet_offsets: vec![],
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

        // println!("  num: [{}, {}) -> {}", a, b, num);

        self.offset += nbits;
        num
    }

    fn var_num(&mut self) -> u64 {
        // println!("Parsing var_num...");
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
        // println!("var_num: [{}, {}) -> {}", _a, _b, num);

        num
    }

    fn all_packets(&mut self) -> Vec<Packet> {
        let mut packets = vec![];

        while let Some(p) = self.next_packet() {
            packets.push(p);
        }
        for (i, p) in packets.iter_mut().enumerate() {
            // println!("Checking packet: {:?}... ", p);

            let num_packets = p.num_packets();
            if num_packets > (u32::MAX as u64) {
                // println!("  Needs patching");
                let bit_scope: u64 = num_packets - u32::MAX as u64;

                let curr: u64 = self.packet_offsets[i];
                let real_num_packets = self.packet_offsets[i..]
                    .iter()
                    .take_while(|o| **o <= curr + bit_scope)
                    .count();

                println!(
                    "Patching {} bits into {} packets",
                    bit_scope, real_num_packets
                );

                match &mut p.payload {
                    Lit(_) => unreachable!(),
                    Op { num_packets, .. } => *num_packets = real_num_packets as u64,
                }
            } else {
                assert_ne!(num_packets, u32::MAX as u64);
                // println!("  OK");
            }
        }

        packets
    }

    fn next_packet(&mut self) -> Option<Packet> {
        let offset = self.offset();
        // println!("Starting packet at {}", offset);

        if self.offset >= self.bits.len() || self.bits[self.offset..].iter().all(|b| *b == b'0') {
            return None;
        }

        let version = self.fix_num(3) as u8;
        let id = self.fix_num(3);

        // println!("version={}, id={}", version, id);

        let payload;
        if id == 4 {
            payload = self.lit_payload();
        } else {
            payload = self.op_payload(id);
        }

        self.packet_offsets.push(offset);

        Some(Packet {
            version,
            offset,
            payload,
        })
    }

    fn lit_payload(&mut self) -> Payload {
        // println!("Parsing Lit");
        Lit(self.var_num())
    }

    fn op_payload(&mut self, id: u64) -> Payload {
        let length_type_id = self.fix_num(1) as u8;
        // println!("Parsing Op w/ length_type_id={}", length_type_id);

        if length_type_id == 0 {
            let nbits = self.fix_num(15) as usize;
            // println!("--- TODO: Handle translating nbits={} into packets", nbits);

            Op {
                id,
                num_packets: u32::MAX as u64 + nbits as u64,
            }
        } else if length_type_id == 1 {
            let num_packets = self.fix_num(11);
            // println!("Expects {} packets", num_packets);

            Op { id, num_packets }
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
    let packets = parser.all_packets();

    packets.iter().map(|p| p.version as u64).sum()
}

// Part2 ======================================================================

#[derive(Debug)]
struct PacketTree {
    p: Packet,
    args: Vec<PacketTree>,
}

impl PacketTree {
    fn eval(&self) -> i64 {
        let n = self.args.len();
        match self.p.id() {
            // sum
            0 => self.args.iter().map(PacketTree::eval).sum(),

            // product
            1 => self.args.iter().map(PacketTree::eval).product(),

            // min
            2 => self.args.iter().map(PacketTree::eval).min().unwrap(),

            // max
            3 => self.args.iter().map(PacketTree::eval).max().unwrap(),

            // lit
            4 => {
                assert_eq!(n, 0);
                match self.p.payload {
                    Lit(x) => x as i64,
                    _ => unreachable!(),
                }
            }

            // greater-than
            5 => {
                assert_eq!(n, 2);
                (self.args[0].eval() > self.args[1].eval()) as i64
            }

            // less-than
            6 => {
                assert_eq!(n, 2);
                (self.args[0].eval() < self.args[1].eval()) as i64
            }

            // equal-to
            7 => {
                assert_eq!(n, 2);
                (self.args[0].eval() == self.args[1].eval()) as i64
            }
            _ => unreachable!(),
        }
    }

    fn len(&self) -> usize {
        // count this node
        let mut n = 1;

        for arg in &self.args {
            // count the length of each argument
            n += arg.len();
        }

        n
    }
}

fn build_tree(mut packets: Vec<Packet>) -> PacketTree {
    fn helper(depth: u32, packets: &mut Vec<Packet>) -> PacketTree {
        assert!(!packets.is_empty());

        let p = packets.remove(0);

        for _ in 0..depth {
            print!("  ");
        }
        println!(
            "Eval'ing packet id={:?}, needs {} args, {} left",
            p.id(),
            p.num_packets(),
            packets.len()
        );
        assert!(p.num_packets() <= packets.len() as u64);

        let mut args = vec![];

        for _ in 0..p.num_packets() {
            args.push(helper(depth + 1, packets));
        }

        PacketTree { p, args }
    }

    helper(0, &mut packets)
}

#[aoc(day16, part2)]
#[inline(never)]
pub fn part2(bits: &[u8]) -> i64 {
    let mut parser = PacketParser::new(bits);
    let packets = parser.all_packets();
    let tree = build_tree(packets);

    dbg!(&tree.len());
    tree.eval()
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

        assert_eq!(vec![expected], parser.all_packets());
    }

    #[test]
    fn check_example_1_id6_op_with_2_subpackets() {
        let bits = text_to_bits("00111000000000000110111101000101001010010001001000000000");
        let mut parser = PacketParser::new(&bits);

        let expected = vec![
            Packet {
                offset: 0,
                version: 1,
                payload: Op {
                    id: 6,
                    num_packets: 2,
                },
            },
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
        ];

        assert_eq!(expected, parser.all_packets());
    }

    #[test]
    fn check_example_1_id6_op_with_3_subpackets() {
        let bits = text_to_bits("11101110000000001101010000001100100000100011000001100000");
        let mut parser = PacketParser::new(&bits);

        let expected = vec![
            Packet {
                offset: 0,
                version: 7,
                payload: Op {
                    id: 3,
                    num_packets: 3,
                },
            },
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
        ];

        assert_eq!(expected, parser.all_packets());
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
        let expected = vec![
            Packet {
                offset: 0,
                version: 4,
                payload: Op {
                    id: 7,
                    num_packets: 2,
                },
            },
            // {
            Packet {
                offset: 22,
                version: 2,
                payload: Op {
                    id: 0,
                    num_packets: 2,
                },
            },
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
            // }
            // {
            Packet {
                offset: 62,
                version: 6,
                payload: Op {
                    id: 1,
                    num_packets: 2,
                },
            },
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
            // }
        ];

        assert_eq!(expected, parser.all_packets());
        assert_eq!(part2(&bits), 1);
    }
}
