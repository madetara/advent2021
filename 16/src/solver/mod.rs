mod packet;

pub fn solve_1(binary_string: &String) -> usize {
    let packet = packet::Packet::parse(&binary_string);

    packet.version_sum()
}

pub fn solve_2(binary_string: &String) -> usize {
    let packet = packet::Packet::parse(&binary_string);

    packet.value()
}
