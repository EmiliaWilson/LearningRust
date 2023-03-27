use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elf_top_three = vec![-1; 3];
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut elf_total: i32 = 0;
        for line in lines {
            if let Ok(items) = line {
                if items == "" {
                    for i in 0..elf_top_three.len() {
                        if elf_total > elf_top_three[i] {
                            elf_top_three.insert(i, elf_total);
                            elf_top_three.pop();

                            let sum: i32 = elf_top_three.iter().sum();
                            println!("current max: {}\n", sum);

                            for i in 0..elf_top_three.len() {
                                println!("{}: {}\n", i, elf_top_three[i]);
                            }
                        break; 
                        }
                    }
                    elf_total = 0;
                } else {
                    elf_total += items.parse::<i32>().unwrap();
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
