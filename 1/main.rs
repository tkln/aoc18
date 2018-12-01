use std::io;

fn main()
{
    let mut deltas: Vec<i32> = Vec::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                line.pop();
                match line.parse::<i32>() {
                    Ok(v) => deltas.push(v),
                    Err(err) => println!("failed to parse: {}, {}", err, line),
                }
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            },
        }
    }
    println!("sum: {}", deltas.iter().fold(0, |sum, x| sum + x ));
}
