use sim::{get_cli_arguments, preprocess_line, process_line, read_lines};
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<(), getopt::Error> {
    // get the CLI arguments s, b, e, t.
    let (s, b, e, t) = get_cli_arguments().unwrap();
    // if the given path doesn't exist; raise an error.
    if !Path::new(&t).exists() {
        panic!("The path to the trace file doesn't exist!");
    }
    // create a buffer reader for the file in the given path.
    let reader = read_lines(t);

    // initialise the cache, the hits, misses and evictions variables
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut hits: i32 = 0;
    let mut misses: i32 = 0;
    let mut evictions: i32 = 0;

    // Iterate over the lines of the file
    for line in reader {
        // extract the code, tag and set_index from the current line.
        let (code, tag, set_index) = preprocess_line(&line.unwrap(), &s, &b);
        // given code, tag and set_index, update the cache, misses, hits and evictions.
        process_line(
            &mut cache,
            &mut hits,
            &mut misses,
            &mut evictions,
            &code,
            &tag,
            &set_index,
            &e,
        );
    }
    // print out the result after iterating all lines in the trace file.
    println!("hits:{} misses:{} evictions:{}", &hits, &misses, &evictions);
    Ok(())
}
