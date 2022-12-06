use util::get_lines;

fn main() {
    get_lines(|lines| {
        let mut iter = lines.iter();
        let crates = iter
            .by_ref()
            .cloned()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();

        let commands = iter
            .cloned()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();

        let mut stack = create_crates(crates);
        let mut stack2 = stack.clone();
        let commands = parse_commands(commands);
        execute(&mut stack, &commands);
        let top = stack
            .iter()
            .map(|c| c.last().unwrap())
            .fold(String::from("_"), |acc, c| format!("{} {}", acc, c));
        println!("{}", top);

        execute_2(&mut stack2, &commands);
        let top = stack2
            .iter()
            .map(|c| c.last().unwrap())
            .fold(String::from("_"), |acc, c| format!("{} {}", acc, c));
        println!("{}", top);
    })
}

fn create_crates(crates: Vec<&str>) -> Vec<Vec<char>> {
    let mut rows = crates
        .iter()
        .map(|row| {
            row.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|c| c[1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    rows.pop();

    let stack = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|row| row[i])
                .filter(|i| i.is_alphabetic())
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    stack
}

fn parse_commands(commands: Vec<&str>) -> Vec<(usize, usize, usize)> {
    commands
        .iter()
        .map(|command| {
            let parts = command.split(" ").collect::<Vec<_>>();
            (
                parts[1].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[5].parse().unwrap(),
            )
        })
        .collect::<_>()
}

fn execute(stack: &mut Vec<Vec<char>>, commands: &Vec<(usize, usize, usize)>) {
    for (count, from, to) in commands {
        for _ in 0..*count {
            move_create(stack, 1, from - 1, to - 1);
        }
    }
}

fn execute_2(stack: &mut Vec<Vec<char>>, commands: &Vec<(usize, usize, usize)>) {
    for (count, from, to) in commands {
        move_create(stack, *count, from - 1, to - 1);
    }
}

fn move_create(stack: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) {
    let len = stack[from].len();
    let keep = stack[from][0..len - count].to_owned();
    let to_move = stack[from][len - count..].to_owned();
    stack[from] = keep;

    for c in to_move {
        stack[to].push(c);
    }
}
