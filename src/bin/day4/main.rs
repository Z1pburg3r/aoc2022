use std::io;
use std::ops::RangeInclusive;

struct Sections {
    section1: RangeInclusive<usize>,
    section2: RangeInclusive<usize>,
}

fn parse_input() -> Vec<Sections> {

    include_str!("input.txt").lines()
        .map(|x| {
            let (s1, s2) = x.split_once(',').unwrap();
            let (s1_begin, s1_end) = s1.split_once('-').unwrap();
            let (s2_begin, s2_end) = s2.split_once('-').unwrap();

            let s1_parsed = s1_begin.parse::<usize>().unwrap()
                ..=s1_end.parse::<usize>().unwrap();
            let s2_parsed = s2_begin.parse::<usize>().unwrap()
                ..=s2_end.parse::<usize>().unwrap();

            Sections {
                section1: s1_parsed,
                section2: s2_parsed,
            }
        }).collect::<Vec<Sections>>()

}

fn sol1() -> io::Result<()> {

    let sections_vec = parse_input();
    
    let partially_contains = sections_vec.into_iter()
        .filter(|sections| {
            let s1_contain_lower = sections.section1
                .contains(sections.section2.start());
            let s1_contain_upper = sections.section1
                .contains(sections.section2.end());
            let s2_contain_lower = sections.section2
                .contains(sections.section1.start());
            let s2_contain_upper = sections.section2
                .contains(sections.section1.end());

            (s1_contain_lower && s1_contain_upper)
            || (s2_contain_lower && s2_contain_upper)
        })
        .collect::<Vec<Sections>>()
        .len();

    println!("{partially_contains}");

    Ok(())
}

fn sol2() -> io::Result<()> {

    let sections_vec = parse_input();

    let fully_contains = sections_vec.into_iter()
        .filter(|sections| {
            let s1_contain_lower = sections.section1
                .contains(sections.section2.start());
            let s1_contain_upper = sections.section1
                .contains(sections.section2.end());
            let s2_contain_lower = sections.section2
                .contains(sections.section1.start());
            let s2_contain_upper = sections.section2
                .contains(sections.section1.end());

            s1_contain_lower || s1_contain_upper
            || s2_contain_lower || s2_contain_upper
        })
        .collect::<Vec<Sections>>()
        .len();

    println!("{fully_contains}");


    Ok(())
}

fn main() {
    sol1().unwrap();
    sol2().unwrap();
}
