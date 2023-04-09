// LRU Cache simulator
use getopt::Opt;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Returns the binary representation of the hexadecimal string input.
///
/// # Arguments
///
/// * `hex` - A string containing a hexadecimal string
pub fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

/// Returns the binary representation of a single hex character.
///
/// # Arguments
///
/// * `c` - A single hex character, which will be translated into binary.
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

/// Returns a buffer reader for the file given by `filename`.
///
/// # Arguments
///
/// * `filename` - the path to a file containing data/instruction memory trace.
pub fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

/// Parses and returns the command line interface arguments.
pub fn get_cli_arguments() -> Result<(i32, i32, i32, String), getopt::Error> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "s:b:E:t:");

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
                Opt('E', Some(arg)) => e = arg.clone(),
                Opt('t', Some(arg)) => t = arg.clone(),
                _ => unreachable!(),
            },
        }
    }
    let s = s.parse::<i32>().unwrap();
    let b = b.parse::<i32>().unwrap();
    let e = e.parse::<i32>().unwrap();

    // CLI arguments b, s an e can't be zero.
    if b == 0 {
        panic!("The value of cli argument b cannot be 0.");
    }
    if s == 0 {
        panic!("The value of cli argument s cannot be 0.");
    }
    if e == 0 {
        panic!("The value of cli argument E cannot be 0.");
    }

    return Ok((s, b, e, t));
}

/// Returns the code, tag and set_index for a line in the trace file.
///
/// The line contains an operation I, L, M or S. The instruction operations are ignored.
/// The other operations, L, M or S are data operations and are parsed by this function.
/// The memory address is translated from hex into binary and the s and b arguments are
/// used to parse and return both the `set_index` and the `tag`.
///
/// # Arguments
///
/// * `line` - A line in the trace file, represented by a string.
/// * `s` - An integer which represents the number of bits used as key for the memory
/// address in the cache.
/// * `b` - An integer which represents the number of data bits for each memory address
/// in the trace file.
pub fn preprocess_line(line: &String, s: &i32, b: &i32) -> (String, String, String) {
    // Define the regex which each valid line in the trace file should match:
    let general_re = Regex::new(r"^ [LMS] [0-9a-fA-F]+,[0-9]+$").unwrap();

    let code = String::new();
    let set_index = String::new();
    let tag = String::new();

    // if the line starts with 'I', do nothing.
    if line.starts_with("I") {
        // if the line doesn't match the predefined regex, do nothing.
    } else if !general_re.is_match(&line) {
        // if the line matches the regex, process it.
    } else if general_re.is_match(&line) {
        // Find the code and the memory address
        // Using Regex, get the first and second regular expressions
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
        // the sum of the given b and s cli arguments can't be larger than the binary
        // address size
        if b + s > addres_size {
            panic!(
                "The sum of b (={}) and s (={}) exceeds the binary address size (={}).",
                &b, &s, &addres_size
            )
        }
        // the cli argument b can't be larger than the binary address size
        if b == &addres_size {
            panic!(
                "The argument b (={}) is equal to the binary address size (={}).",
                &b, &addres_size
            )
        }

        // extract the tag from the binary address
        let tag_length = &addres_size - b - s;
        let tag_length = tag_length as usize;
        let tag = &address_binary[0..tag_length];

        // extract the set_index from the binary address
        let set_length = &addres_size - b;
        let set_length = set_length as usize;
        let set_index = &address_binary[tag_length..set_length];

        // retrun code, tag and set_index
        return (code.to_string(), tag.to_string(), set_index.to_string());
    }
    return (code.to_string(), tag.to_string(), set_index.to_string());
}

/// Process a single line in the trace file.
///
/// A single line of the file is processed. This line is represented by the code,
/// tag and set_index. Depending on the operation code, the cache is consulted a
/// number of times. Whilst processing the line, the cache, number of hits,
/// evictions and misses are updated, which is why they are mutable references.
///
/// # Arguments
///
/// * `cache` - mutable reference to the cache variable. A hashmap using the set_indexes
/// as key and a Vector of tags as values.
/// * `hits` -  The total number of hits in the trace file, mutable reference to 32 bit integer.
/// * `misses` - The total number of misses in the trace file, mutable reference to 32 bit integer.
/// * `evictions` - The total number of evictions in the trace file, mutable reference to 32 bit integer.
/// * `code` - The code for the line being processed, being L, M or S.
/// * `tag` - The binary representation of the tag of the current memory address.
/// * `set_index` - The binary representation of the set_index of the current memory address.
/// * `e` - The max length of each vector in the cache.
pub fn process_line(
    cache: &mut HashMap<String, Vec<String>>,
    hits: &mut i32,
    misses: &mut i32,
    evictions: &mut i32,
    code: &String,
    tag: &String,
    set_index: &String,
    e: &i32,
) {
    if code == "L" {
        check_cache(cache, hits, misses, evictions, tag, set_index, e);
    }
    if code == "S" {
        check_cache(cache, hits, misses, evictions, tag, set_index, e);
    }
    if code == "M" {
        check_cache(cache, hits, misses, evictions, tag, set_index, e);
        check_cache(cache, hits, misses, evictions, tag, set_index, e);
    }
}

/// Checks the cache for for a tag and set_index.
///
/// This function will check if the cache contains a certain set_index key.
/// If the cache contains that key, the vector which is stored as value of said
/// set_index key will be searched for the given tag. If the tag is in vector,
/// the tag will be pushed to the start of the vector, which is the position of the
/// most recently used tag. If the tag is not found in the vector, it is added to the
/// start of the vector, given the length restriction is not voided. If the vector
/// has reached max length, the last tag is popped (= Least Recently Used tag) and
/// the new tag is again added at the start of the vector.
/// If the cache doesn't contain the set_index key, the key is added with as value
/// a newly initialised vector containing the given tag.
///
/// # Arguments
///
/// * `cache` - A line in the trace file, represented by a string.
/// * `hits` - The total number of hits in the trace file, mutable reference to 32 bit integer.
/// * `misses` - The total number of misses in the trace file, mutable reference to 32 bit integer.
/// * `evictions` - The total number of evictions in the trace file, mutable reference to 32 bit integer.
/// * `tag` - The binary representation of the tag of the current memory address.
/// * `set_index` - The binary representation of the set_index of the current memory address.
/// * `e` - The max length of each vector in the cache.
pub fn check_cache(
    cache: &mut HashMap<String, Vec<String>>,
    hits: &mut i32,
    misses: &mut i32,
    evictions: &mut i32,
    tag: &String,
    set_index: &String,
    e: &i32,
) {
    // check if the given set_index is a key in the cache hashmap:
    if cache.contains_key(&set_index.to_string()) {
        // get the vector of tags stored in the cache with the set_index key
        let tag_vec = cache.get_mut(&set_index.to_string()).unwrap();
        // if the given tag is in the vector, we have a hit.
        // update the hits and the order of the tags in the vector
        // This means moving the current tag in the vector to the first place, which
        // is the most recently used position.
        if tag_vec.contains(tag) {
            *hits += 1;
            tag_vec.retain(|x| *x != tag.to_string());
            tag_vec.insert(0, tag.to_string());
        // if the given tag is not in the vector, we have a miss.
        } else if !tag_vec.contains(tag) {
            *misses += 1;
            // Update the vector with the given tag.
            // if the vector is shorter than the allowed lenght, it is simply added
            // at the front of the vector.
            if tag_vec.len() < *e as usize {
                tag_vec.insert(0, tag.to_string());
            // if the vector has reached its allowed length, pop the last tag (=
            // the least recently used (LRU) address tag) and add the current tag in the
            // front.
            } else {
                *evictions += 1;
                tag_vec.pop();
                tag_vec.insert(0, tag.to_string());
            }
        }
    // if the current set_index is not in the hashmap, initialise a vector with the
    // current tag and insert the that vector in the cache with the set_index as key.
    } else if !cache.contains_key(&set_index.to_string()) {
        *misses += 1;
        let mut vec = Vec::with_capacity(*e as usize);
        vec.insert(0, tag.to_string());
        cache.insert(set_index.to_string(), vec);
    }
}
