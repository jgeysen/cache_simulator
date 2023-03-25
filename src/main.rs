// LRU Cache simulator
use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use getopt::Opt;

fn _type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string()
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn get_cli_arguments() -> Result<(i32, i32, i32), getopt::Error>{
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "s:b:e:");

    let mut s = String::new();
    let mut b = String::new();
    let mut e = String::new();

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('s', Some(arg)) => s = arg.clone(),
                Opt('b', Some(arg)) => b = arg.clone(),
                Opt('e', Some(arg)) => e = arg.clone(),
                _ => unreachable!(),
            },
        }
    }
    let s = s.parse::<i32>().unwrap();
    let b = b.parse::<i32>().unwrap();
    let e = e.parse::<i32>().unwrap();

//     println!("Option s: {}, {}", s, type_of(&s));
//     println!("Option b: {}, {}", b, type_of(&b));
//     println!("Option e: {}, {}", e, type_of(&e));

    return Ok((s, b, e))
}

fn main() -> Result<(), getopt::Error>{
    // Stores the iterator of lines of the file in lines variable.
    let _reader = read_lines("./traces/trans.trace".to_string());
    let (_s, _b, _e) = get_cli_arguments().unwrap();

    // Iterate over the lines of the file, and in this case print them.
//     for line in lines {
//         type_of(&line);
//         println!("{}", line.unwrap());
//     }

    Ok(())
}
