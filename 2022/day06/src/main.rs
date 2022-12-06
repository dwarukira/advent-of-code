use util::get_lines;

fn main() {
    get_lines(|lines| {
        for line in lines.to_owned() {
            let count = get_marker(line, 4);
            println!("Part 1: {}", count);
        }
        println!("----------------------------------------");
        for line in lines {
            let count = get_marker(line, 14);
            println!("Part 2: {}", count);
        }
    })
}

fn get_marker(stream: &str, skip: usize) -> usize {
    match stream.chars().enumerate().find(|(c, _)| {
        let next_four = &stream.chars().skip(*c).take(skip).collect::<String>();
        let has_duplicates = next_four.chars().any(|c| next_four.matches(c).count() > 1);
        !has_duplicates
    }) {
        Some((c, _)) => c + skip,
        None => 0,
    }
}
