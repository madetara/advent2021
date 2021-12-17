use std::sync::Arc;

#[derive(Clone)]
pub struct Packet {
    pub version: u8,
    pub packet_value: Arc<dyn Operator>,
}

pub trait Operator {
    fn value(&self) -> usize;
    fn version_sum(&self) -> usize;
}

struct Literal {
    x: usize,
}

struct Vector<F>
where
    F: Fn(&Vec<usize>) -> usize,
{
    v: Vec<Packet>,
    f: F,
}

struct Pair<F>
where
    F: Fn(usize, usize) -> usize,
{
    a: Packet,
    b: Packet,
    f: F,
}

impl Operator for Literal {
    fn value(&self) -> usize {
        self.x
    }

    fn version_sum(&self) -> usize {
        0
    }
}

impl<F: Fn(&Vec<usize>) -> usize> Operator for Vector<F> {
    fn value(&self) -> usize {
        (self.f)(&self.v.iter().map(|op| op.value()).collect())
    }

    fn version_sum(&self) -> usize {
        self.v.iter().map(|p| p.version_sum()).sum()
    }
}

impl<F: Fn(usize, usize) -> usize> Operator for Pair<F> {
    fn value(&self) -> usize {
        (self.f)(self.a.value(), self.b.value())
    }

    fn version_sum(&self) -> usize {
        self.a.version_sum() + self.b.version_sum()
    }
}

impl Packet {
    pub fn parse(s: &str) -> Packet {
        let (packet, _) = Packet::parse_internal(s);
        packet
    }

    fn parse_internal(s: &str) -> (Packet, usize) {
        let version = u8::from_str_radix(&s[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&s[3..6], 2).unwrap();
        let mut read = 6;

        let packet_value: Arc<dyn Operator> = if type_id == 4 {
            // literal
            let mut i = 6usize;
            let mut binary_value = String::new();

            loop {
                binary_value += &s[(i + 1)..(i + 5)];
                read += 5;

                if s[i..(i + 1)].eq("0") {
                    break;
                }

                i += 5;
            }

            Arc::new(Literal {
                x: usize::from_str_radix(&binary_value, 2).unwrap(),
            })
        } else {
            // operator
            let length_type_id = &s[6..7];
            read += 1;

            let packets = match length_type_id {
                "0" => {
                    let subpacket_length = usize::from_str_radix(&s[7..22], 2).unwrap();
                    read += 15;

                    let mut result = vec![];
                    let mut i = 22;
                    let end = 22 + subpacket_length;

                    loop {
                        let (next_packet, packet_size) = Packet::parse_internal(&s[i..]);
                        result.push(next_packet);
                        read += packet_size;

                        if packet_size == end - i {
                            break;
                        }

                        i += packet_size;
                    }

                    result
                }
                "1" => {
                    let subpacket_count = usize::from_str_radix(&s[7..18], 2).unwrap();
                    read += 11;

                    let mut result = vec![];
                    let mut i = 18;
                    let mut packets_read = 0;

                    loop {
                        let (next_packet, packet_size) = Packet::parse_internal(&s[i..]);
                        result.push(next_packet);
                        read += packet_size;
                        packets_read += 1;

                        if packets_read == subpacket_count {
                            break;
                        }

                        i += packet_size;
                    }

                    result
                }
                _ => panic!("incorrect length type id"),
            };

            match type_id {
                0 => Arc::new(Vector {
                    f: |vec| vec.iter().sum(),
                    v: packets,
                }),
                1 => Arc::new(Vector {
                    f: |vec| vec.iter().product(),
                    v: packets,
                }),
                2 => Arc::new(Vector {
                    f: |vec| *vec.iter().min().unwrap(),
                    v: packets,
                }),
                3 => Arc::new(Vector {
                    f: |vec| *vec.iter().max().unwrap(),
                    v: packets,
                }),
                5 => Arc::new(Pair {
                    f: |a, b| if a > b { 1 } else { 0 },
                    a: packets[0].clone(),
                    b: packets[1].clone(),
                }),
                6 => Arc::new(Pair {
                    f: |a, b| if a < b { 1 } else { 0 },
                    a: packets[0].clone(),
                    b: packets[1].clone(),
                }),
                7 => Arc::new(Pair {
                    f: |a, b| if a == b { 1 } else { 0 },
                    a: packets[0].clone(),
                    b: packets[1].clone(),
                }),
                _ => panic!("incompatible type id"),
            }
        };

        (
            Packet {
                version,
                packet_value,
            },
            read,
        )
    }

    pub fn version_sum(&self) -> usize {
        self.version as usize + self.packet_value.version_sum()
    }

    pub fn value(&self) -> usize {
        self.packet_value.value()
    }
}
