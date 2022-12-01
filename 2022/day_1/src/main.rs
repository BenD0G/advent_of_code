mod input;

use input::INPUT;

fn solve_1() -> u64 {
    let mut current_best = 0;
    let mut current_elf_total = 0;

    for row in INPUT.lines() {
        if row.is_empty() {
            if current_elf_total > current_best {
                current_best = current_elf_total;
            }
            current_elf_total = 0;
        } else {
            current_elf_total += row.parse::<u64>().unwrap();
        }
    }

    current_best
}

fn solve_2() -> u64 {
    let mut current_gold_medalist = 0;
    let mut current_silver_medalist = 0;
    let mut current_bronze_medalist = 0;
    let mut current_elf_total = 0;

    for row in INPUT.lines() {
        if row.is_empty() {
            if current_elf_total > current_gold_medalist {
                current_bronze_medalist = current_silver_medalist;
                current_silver_medalist = current_gold_medalist;
                current_gold_medalist = current_elf_total;
            } else if current_elf_total > current_silver_medalist {
                current_bronze_medalist = current_silver_medalist;
                current_silver_medalist = current_elf_total;
            } else if current_elf_total > current_bronze_medalist {
                current_bronze_medalist = current_elf_total;
            }
            current_elf_total = 0;
        } else {
            current_elf_total += row.parse::<u64>().unwrap();
        }
    }
    current_gold_medalist + current_silver_medalist + current_bronze_medalist
}

fn main() {
    let solution_1 = solve_1();
    println!("{}", solution_1);

    let solution_2 = solve_2();
    println!("{}", solution_2);
}
