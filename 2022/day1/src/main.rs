use std::fs;


fn main() {
    match fs::read_to_string("input.txt") {
        Err(_) => println!("can't read the input file"),
        Ok(content) => {
            let elf_sums = content.split("\n\n").map(|c| -> usize { 
                c.split("\n").map(|l| l.parse::<usize>().unwrap_or(0)).sum()
             });
            let mut sum_vec: Vec<usize> = elf_sums.collect();
            sum_vec.sort();
            let l = sum_vec.len();
            let top: usize = sum_vec.split_off(l - 3).into_iter().sum();
            println!("{:?}", top);
        }
    }    
}

