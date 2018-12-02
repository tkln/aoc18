use std::io;
use std::collections::HashMap;

fn read_input() ->  Vec<Vec<char>> {
    let mut ids: Vec<Vec<char>> = Vec::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(n)       => {
                if n == 0 {
                    break;
                }
                ids.push(line.chars().collect());
            }
            Err(err)    => println!("Failed to read line: {}", err),
        }
    }

    ids
}

fn count_letters(id: &Vec<char>) -> (i32, i32) {
    let mut chars: HashMap<char, u32> = HashMap::new();
    let mut doubles: i32 = 0;
    let mut triples: i32 = 0;

    /* Build a histogram of letter frequencies */
    id.iter().for_each(|c| *(chars.entry(*c).or_insert(0)) += 1);

    for n in chars.values() {
        if *n == 3 {
            triples = 1;
            continue;
        }
        if *n == 2 {
            doubles = 1;
        }
    }

    (doubles, triples)
}

fn main() {
    let ids: Vec<Vec<char>> = read_input();

    /* Sum up the letter counts */
    let pair = ids.iter().fold((0, 0), |acc, id| {
        let r = count_letters(id);
        (acc.0 + r.0, acc.1 + r.1)
    });

    println!("n: {}", ids.len());
    println!("checksum: {}", pair.0 * pair.1);
}
