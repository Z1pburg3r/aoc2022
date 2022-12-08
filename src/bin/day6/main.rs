use std::io;
use std::collections::HashSet;

fn sol1() -> Option<(usize, &'static str)> {

    let data_stream = include_str!("input.txt");
    let final_slice = data_stream.len() - 4;

    for pos in 0..final_slice {
        let slice = data_stream[pos..pos + 4].chars();
        let set = HashSet::<char>::from_iter(slice);
        if set.len() == 4 {
            let pkt_start = pos + 4;
            return Some((pkt_start, data_stream));
        }
    }
    None
}

fn sol2(pkt_start: usize, data_stream: &str) -> Option<usize> {

    let pkt_start_idx = pkt_start - 1;
    let final_slice = data_stream.len() - 14;

    for pos in pkt_start_idx..final_slice {
        let slice = data_stream[pos..pos + 14].chars();
        let set = HashSet::<char>::from_iter(slice);
        if set.len() == 14 {
            let msg_start = pos + 14;
            return Some(msg_start);
        }
    }

    None
}


fn sol() -> io::Result<()> {

    let (pkt_start, data_stream) = match sol1() {
        Some((x, y)) => (x, y),
        None => panic!("No packet start was detected!"),
    };

    println!("Packet start: {pkt_start}");

    match sol2(pkt_start, data_stream) {
        Some(x) => println!("Message start: {x}"),
        None => panic!("No start of message was detected!"),
    }
    
    Ok(())
}

fn main() {
    sol().unwrap();
}
