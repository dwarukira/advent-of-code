use util::get_lines;

fn main() {
    get_lines(|content| {
        let pairs = content.iter().map(|line| {
            let sections = line
                .split(",")
                .map(|part| {
                    let parts = part
                        .split("-")
                        .map(|n| n.parse::<usize>().unwrap_or(0))
                        .collect::<Vec<usize>>();
                    (parts[0], parts[1])
                })
                .collect::<Vec<(usize, usize)>>();
            (sections[0], sections[1])
        });
        let contained = pairs.to_owned().filter(|(a, b)| contains_point(*a, *b));
        println!("part 1: {:?}", contained.collect::<Vec<_>>().len());

        let overlaps = pairs.to_owned().filter(|(a, b)| overlaps(*a, *b));
        println!("part 2: {:?}", overlaps.collect::<Vec<_>>().len());
    });
}

fn contains_point((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
    (x1 <= x2 && y1 >= y2) || (x2 <= x1 && y2 >= y1)
}

fn overlaps((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
    let a = x1 <= x2 && y1 >= x2;
    let b = x1 <= y2 && y1 >= y2;
    let c = x2 <= x1 && y2 >= x1;
    let d = x2 <= y1 && y2 >= y1;
    a || b || c || d
}