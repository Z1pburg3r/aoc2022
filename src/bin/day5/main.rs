use std::io;

type CrateStack<'a> = Vec::<&'a str>;

struct Crate<'a> {
    position: usize, // To be calculated as an index into `CrateStack`
    crate_id: &'a str,
}

struct Move {
    iters: usize,
    from: usize,
    to: usize,
}


fn parse_input() -> (Vec<CrateStack<'static>>, Vec<Move>) {

    // Number of stacks determined from the input file
    // Could probably parse this out, but for the sake of time...
    let mut crate_stacks = Vec::<CrateStack>::with_capacity(9);

    for _i in 0..crate_stacks.capacity() {
        crate_stacks.push(CrateStack::new());
    }

    let (start_positions, move_list) = include_str!("input.txt")
        .split_once("\n\n")
        .unwrap();

    let mut crates: Vec<Vec<Crate>> = start_positions.lines()
        .map(|x| {
            x.match_indices(|delim: char| delim.is_ascii_uppercase())
            .map(|crate_info| {
                Crate {
                    position: (crate_info.0 - 1) / 4,
                    crate_id: crate_info.1
                }
            }).collect()
        }).collect();

    crates.reverse(); // Reverse to get crates in ascending order

    // Stack those crates!
    for c in crates {
        for c_data in c {
            crate_stacks[c_data.position]
                .push(c_data.crate_id);
        }
    }

    let moves: Vec<Move> = move_list.lines()
        .map(|l| {
            let triple: Vec<usize> = l.split(' ')
                .filter(|s| s.contains(char::is_numeric))
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Move {
                iters: *triple.get(0).unwrap(),
                from: *triple.get(1).unwrap() - 1,
                to: *triple.get(2).unwrap() - 1,
            }
        }).collect();

    (crate_stacks, moves)
}

fn sol1() -> io::Result<()> {

    let (mut crate_stacks, moves) = parse_input();

    for m in moves {
        for _ in 0..m.iters {
            let sel = crate_stacks[m.from].pop().unwrap();
            crate_stacks[m.to].push(sel);
        }
    }

    for mut stack in crate_stacks {
        print!("{}", stack.pop().unwrap());
    }

    Ok(())
}

fn sol2() -> io::Result<()> {

    let (mut crate_stacks, moves) = parse_input();
    let mut substack = Vec::<&str>::new();

    for m in moves {
        for _ in 0..m.iters {
            substack.push(crate_stacks[m.from].pop().unwrap());
        }

        substack.reverse();

        for c in &substack {
            crate_stacks[m.to].push(c);
        }

        substack.clear();
    }

    for mut stack in crate_stacks {
        print!("{}", stack.pop().unwrap());
    }

    Ok(())
}

fn main() {
    sol1().unwrap();
    println!();
    sol2().unwrap();
    println!();
}
