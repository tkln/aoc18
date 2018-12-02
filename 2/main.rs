use std::io;

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

fn main() {
    let ids: Vec<Vec<char>> = read_input();
    println!("{}", ids.len());
}
