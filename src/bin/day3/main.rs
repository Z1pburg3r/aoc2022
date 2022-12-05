use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

fn sol1(priorities: &HashMap<char, u32>) -> io::Result<()> {

    let mut priority_total: u32 = 0;

    let rucksacks: Vec<(&str, &str)> = include_str!("input.txt").lines()
        .map(|x| x.split_at(x.len() / 2))
        .collect();

    for compartments in rucksacks {
        let compartment1: HashSet::<char> = compartments.0.chars().collect();
        let compartment2: HashSet::<char> = compartments.1.chars().collect();
        for x in compartment1.intersection(&compartment2) {
            priority_total += priorities.get(&x).unwrap();
        }
    }

    println!("{priority_total}");

    Ok(())
}

fn sol2(priorities: &HashMap<char, u32>) -> io::Result<()> {

    let mut priority_total: u32 = 0;
    let mut grouped_rucksacks = Vec::<(&str, &str, &str)>::new();
    let mut tmp = Vec::<&str>::new();

    for line in include_str!("input.txt").lines().enumerate() {
        tmp.push(line.1);
        if line.0 % 3 == 2 {
            grouped_rucksacks.push((tmp[0], tmp[1], tmp[2]));
            tmp.clear();
            continue;
        }
    }

    for group in grouped_rucksacks {
        let rucksack1: HashSet::<char> = group.0.chars().collect();
        let rucksack2: HashSet::<char> = group.1.chars().collect();
        let rucksack3: HashSet::<char> = group.2.chars().collect();
        
        let intersection1 = rucksack1.intersection(&rucksack2)
        .collect::<HashSet<&char>>();
        let intersection2 = rucksack2.intersection(&rucksack3)
        .collect::<HashSet<&char>>();
        let final_intersection = intersection1.intersection(&intersection2);
        for x in final_intersection {
            priority_total += priorities.get(x).unwrap();
        }
    }

    println!("{priority_total}");

    Ok(())
}

fn main() {

    let mut alpha: Vec<char> = ('a'..='z').collect();
    alpha.append(&mut ('A'..='Z').collect());

    let nums: Vec<u32> = (1..=52).collect();

    let priorities: HashMap::<char, u32> = {
        alpha.into_iter()
            .zip(nums.into_iter())
            .collect()
    };

    sol1(&priorities);
    sol2(&priorities);
}
