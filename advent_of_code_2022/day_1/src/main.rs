fn main() {
    let input = include_str!("input.txt");
    let mut elves_cal: Vec<usize> = vec![0];

    let mut current_elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            current_elf += 1;
            elves_cal.push(0);
        } else {
            elves_cal[current_elf] += line.parse::<usize>().unwrap();
        }
    }

    elves_cal.sort_by(|a, b| b.cmp(a));

    // Part 1
    println!("{}", &elves_cal[0]);

    // Part 2
    println!("{}", &elves_cal[0] + &elves_cal[1] + &elves_cal[2]);
}
