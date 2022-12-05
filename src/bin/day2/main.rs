use std::io;

fn sol1() -> io::Result<()> {

    let rounds: Vec<Vec<&str>> = include_str!("input.txt").lines()
        .map(|x| {
            x.split(' ')
            .collect()
        }).collect();

    let mut total: i32 = 0;
    let mut diff: i32;

    for round in rounds {
        let opp_shape = match round[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("{} is an invalid play!", round[0]),
        };
        let shape = match round[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("{} is an invalid play!", round[1]),
        };

        diff = opp_shape - shape;

        match diff.rem_euclid(3) {
            0 => total += shape + 3, // Draw
            1 => total += shape,     // Loss
            2 => total += shape + 6, // Win
            _ => panic!("Error applying Euclid's algorithm!"),
        }
    }

    println!("{}", total);

    Ok(())
}

fn sol2() -> io::Result<()> {

    let rounds: Vec<Vec<&str>> = include_str!("input.txt").lines()
        .map(|x| {
            x.split(' ')
            .collect()
        }).collect();

    let mut total: i32 = 0;
    let mut diff: i32;

    for round in rounds {
        let opp_shape = match round[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("{} is an invalid play!", round[0]),
        };
        let shape = match round[1] {
            "X" => (opp_shape + 1) % 3 + 1,
            "Y" => opp_shape,
            "Z" => opp_shape % 3 + 1,
            _ => panic!("{} is an invalid play!", round[1]),
        };

        diff = opp_shape - shape;

        match diff.rem_euclid(3) {
            0 => total += shape + 3, // Draw
            1 => total += shape,     // Loss
            2 => total += shape + 6, // Win
            _ => panic!("Error applying Euclid's algorithm!"),
        }
    }

    println!("{}", total);

    Ok(())
}

fn main() {
    sol1();
    sol2();
}
