// LRU Cache simulator
use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::env;
use getopt::Opt;
use regex::Regex;
use ascii_converter::hexadecimal_to_binary;

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

    return Ok((s, b, e))
}

fn preprocess_line(line: &String, _s: &i32, _b: &i32) {
    let general_re = Regex::new(r"^ [LMS] [0-9a-fA-F]+,[0-9]+$").unwrap();

    if line.starts_with("I") {
    } else if general_re.is_match(&line) {
        // Find the code and the memory address
        let re = Regex::new(r"^ ([LMS]) ([0-9a-fA-F]+),[0-9]+$").unwrap();
        let code = re.captures(&line).unwrap().get(1).map_or("", |m| m.as_str());
        let address = re.captures(&line).unwrap().get(2).map_or("", |m| m.as_str());
        let address_hex_vec: &Vec<_> = &address.matches(char::is_alphanumeric).collect();
        let doubled: Vec<String> = address_hex_vec.into_iter().map(|x| x.to_string()).collect();

        let address_bin = hexadecimal_to_binary(&doubled).unwrap();

        println!("Code: {}\nAddress: {:?}", &code, &address);
        println!("Address in binary: {:?}", &address_bin);

        // Translate memory address to binary:

        // from memory address, find the tag


        // from memory address, find the set_index


    }
//     return (code, tag, set_index)
}

fn main() -> Result<(), getopt::Error>{
    env::set_var("RUST_BACKTRACE", "1");
    // Stores the iterator of lines of the file in lines variable.
    let reader = read_lines("./traces/trans.trace".to_string());
    let (s, b, _e) = get_cli_arguments().unwrap();

    // Iterate over the lines of the file, and in this case print them.
    for line in reader {
        preprocess_line(&line.unwrap(), &s, &b)
//         process_line(code, tag, set_index)
    }

    Ok(())
}
