use util::get_lines;

const LOWER_CASE_OFFSET: u8 = b'a' - 1;
const UPPERCASE_OFFSET: u8 = b'A' - 27;

fn main() {
    get_lines(|content| {
        let pairs = content.into_iter().map(|w| {
            let (a, b) = w.split_at(w.len() / 2);
            Vec::from([a.to_owned(), b.to_owned()])
        });
        let sum: usize = pairs.map(find_items_in_common).map(to_priority).sum();
        println!("part 1: {:?}", sum);
    })
}

fn find_items_in_common(pair: Vec<String>) -> char {
    match pair.split_first() {
        Some((first, rest)) => {
            'outer: for c in first.chars() {
                for v in rest {
                    if !v.contains(c) {
                        continue 'outer;
                    }
                }
                return c;
            }
            char::from(LOWER_CASE_OFFSET)
        }
        None => char::from(LOWER_CASE_OFFSET),
    }
}

fn to_priority(c: char) -> usize {
    let offset = if c.is_uppercase() {
        UPPERCASE_OFFSET
    } else {
        LOWER_CASE_OFFSET
    };
    let c = (c as u8) - offset;
    c.into()
}
