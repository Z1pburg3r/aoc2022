use std::io;

fn sol1() -> io::Result<()> {

    let max_cals = include_str!("input.txt").split("\n\n")
        .map(|x| { 
            x.lines()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum::<u32>()
        }
    ).max();

    println!("{}", max_cals.unwrap());

    Ok(())
}

fn sol2() -> io::Result<()> {

    let mut cal_sums = include_str!("input.txt").split("\n\n")
        .map(|x| {
            x.lines()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum::<u32>()
        }
    ).collect::<Vec<u32>>();

    cal_sums.sort_by(|a, b| b.cmp(a));

    println!("{}", cal_sums.iter().take(3).sum::<u32>());
    
    Ok(())
}

fn main() {
    sol1();
    sol2();
}
