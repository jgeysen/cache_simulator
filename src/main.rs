// LRU Cache simulator
use getopt::Opt;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn _type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "",
    }
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn get_cli_arguments() -> Result<(i32, i32, i32, String), getopt::Error> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "s:b:e:t:");

    let mut s = String::new();
    let mut b = String::new();
    let mut e = String::new();
    let mut t = String::new();

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('s', Some(arg)) => s = arg.clone(),
                Opt('b', Some(arg)) => b = arg.clone(),
                Opt('e', Some(arg)) => e = arg.clone(),
                Opt('t', Some(arg)) => t = arg.clone(),
                _ => unreachable!(),
            },
        }
    }
    let s = s.parse::<i32>().unwrap();
    let b = b.parse::<i32>().unwrap();
    let e = e.parse::<i32>().unwrap();

    return Ok((s, b, e, t));
}

fn preprocess_line(line: &String, s: &i32, b: &i32) -> (String, String, String) {
    let general_re = Regex::new(r"^ [LMS] [0-9a-fA-F]+,[0-9]+$").unwrap();

    let code = String::new();
    let set_index = String::new();
    let tag = String::new();

    if line.starts_with("I") {
    } else if general_re.is_match(&line) {
        // Find the code and the memory address
        let re = Regex::new(r"^ ([LMS]) ([0-9a-fA-F]+),[0-9]+$").unwrap();
        let code = re
            .captures(&line)
            .unwrap()
            .get(1)
            .map_or("", |m| m.as_str());
        let address = re
            .captures(&line)
            .unwrap()
            .get(2)
            .map_or("", |m| m.as_str());

        // Translate memory address to binary:
        let address_binary = convert_to_binary_from_hex(&address);

        // from binary memory address, find the tag
        let addres_size = &address_binary.len();
        let addres_size = *addres_size as i32;
        let tag_length = &addres_size - b - s;
        let tag_length = tag_length as usize;
        let tag = &address_binary[0..tag_length];

        // from binary memory address, find the set_index
        let set_length = &addres_size - b;
        let set_length = set_length as usize;
        let set_index = &address_binary[tag_length..set_length];

        //         println!("Code: {}", &code);
        //         println!("Address: {:?}", &address);
        //         println!("Address in binary: {:?}", &address_binary);
        //         println!("tag_length: {}", &tag_length);
        //         println!("tag: {}", &tag);
        //         println!("set_length: {}", &set_length);
        //         println!("set_index: {}", &set_index);

        return (code.to_string(), tag.to_string(), set_index.to_string());
    }
    return (code.to_string(), tag.to_string(), set_index.to_string());
}

fn process_line(
    cache: &mut HashMap<String, String>,
    hits: &mut i32,
    misses: &mut i32,
    evictions: &mut i32,
    code: &String,
    tag: &String,
    set_index: &String,
) {
    if code == "L" {
        check_cache(cache, hits, misses, evictions, tag, set_index);
    }
    if code == "S" {
        check_cache(cache, hits, misses, evictions, tag, set_index);
    }
    if code == "M" {
        check_cache(cache, hits, misses, evictions, tag, set_index);
        check_cache(cache, hits, misses, evictions, tag, set_index);
    }
}

fn check_cache(
    cache: &mut HashMap<String, String>,
    hits: &mut i32,
    misses: &mut i32,
    evictions: &mut i32,
    tag: &String,
    set_index: &String,
) {
    if cache.contains_key(&set_index as &str) && cache.get(&set_index as &str) == Some(tag) {
        *hits += 1;
        //         println!("hit");
    }
    if cache.contains_key(&set_index as &str) && cache.get(&set_index as &str) != Some(tag) {
        *misses += 1;
        //         println!("miss");
        cache.remove(&set_index as &str);
        //         println!("eviction");
        cache.insert(set_index.to_string(), tag.to_string());
        *evictions += 1;
    }
    if !cache.contains_key(&set_index as &str) {
        *misses += 1;
        //         println!("miss");
        cache.insert(set_index.to_string(), tag.to_string());
    }
}

fn main() -> Result<(), getopt::Error> {
    // Stores the iterator of lines of the file in lines variable.
    let (s, b, _e, t) = get_cli_arguments().unwrap();
    let reader = read_lines(t);

    // initialise the cache, the hits, misses and evictions variables
    let mut cache: HashMap<String, String> = HashMap::new();
    let mut hits: i32 = 0;
    let mut misses: i32 = 0;
    let mut evictions: i32 = 0;

    // Iterate over the lines of the file
    for line in reader {
        let (code, tag, set_index) = preprocess_line(&line.unwrap(), &s, &b);
        process_line(
            &mut cache,
            &mut hits,
            &mut misses,
            &mut evictions,
            &code,
            &tag,
            &set_index,
        );
    }
    println!("hits:{} misses:{} evictions:{}", &hits, &misses, &evictions);
    Ok(())
}
