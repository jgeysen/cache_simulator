// tests for the LRU cache simulator
use sim::{check_cache, convert_to_binary_from_hex, preprocess_line};
use std::collections::HashMap;

// test the binary conversion of string "1"
#[test]
fn binary_hex_converter_1() {
    let bin = convert_to_binary_from_hex("1");
    assert_eq!(bin, "0001");
}

// test the binary conversion of string "1ABC78"
#[test]
fn binary_hex_converter_2() {
    let bin = convert_to_binary_from_hex("1ABC78");
    assert_eq!(bin, "000110101011110001111000");
}

// test the binary conversion of string "AF883BC78"
#[test]
fn binary_hex_converter_3() {
    let bin = convert_to_binary_from_hex("AF883BC78");
    assert_eq!(bin, "101011111000100000111011110001111000");
}

// test the binary conversion of string "F123B8"
#[test]
fn binary_hex_converter_4() {
    let bin = convert_to_binary_from_hex("F123B8");
    assert_eq!(bin, "111100010010001110111000");
}

// test the binary conversion of string "BBBA8"
#[test]
fn binary_hex_converter_5() {
    let bin = convert_to_binary_from_hex("BBBA8");
    assert_eq!(bin, "10111011101110101000");
}

// test preprocessing of a single line of the trace file, with given s and b input variables
#[test]
fn preprocess_line_1() {
    let (code, tag, set_index) = preprocess_line(&String::from(" L AF883BC78,8"), &8, &8);
    assert_eq!(code, "L");
    assert_eq!(tag, "10101111100010000011");
    assert_eq!(set_index, "10111100");
}

// test preprocessing of a single line of the trace file, with given s and b input variables
#[test]
fn preprocess_line_2() {
    let (code, tag, set_index) = preprocess_line(&String::from(" L AF883BC78,3"), &12, &4);
    assert_eq!(code, "L");
    assert_eq!(tag, "10101111100010000011");
    assert_eq!(set_index, "101111000111");
}

// test preprocessing of a single line of the trace file, with given s and b input variables
#[test]
fn preprocess_line_3() {
    let (code, tag, set_index) = preprocess_line(&String::from(" S AF883BC78,9"), &4, &16);
    assert_eq!(code, "S");
    assert_eq!(tag, "1010111110001000");
    assert_eq!(set_index, "0011");
}

// test the check_cache function by passing on an initialised cache and checking the output:
#[test]
fn check_cache_1() {
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut hits: i32 = 0;
    let mut misses: i32 = 0;
    let mut evictions: i32 = 0;

    let set_index: String = String::from("1001001");
    let tag: String = String::from("1001");
    let e = 1;

    check_cache(&mut cache, &mut hits, &mut misses, &mut evictions, &tag, &set_index, &e);
    assert_eq!(misses, 1);
    assert_eq!(hits, 0);
    assert_eq!(evictions, 0);
    assert!(cache.contains_key(&set_index));
    assert_eq!(cache.get(&set_index).unwrap(), &vec![tag]);
}

// test the check_cache function by passing on an initialised cache and checking the output:
#[test]
fn check_cache_2() {
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut hits: i32 = 0;
    let mut misses: i32 = 0;
    let mut evictions: i32 = 0;

    let set_index: String = String::from("1001001");
    let tag: String = String::from("1001");
    let e = 1;

    // on the same set_index key, insert the same tag, to invoke a hit:
    let mut vec = Vec::with_capacity(e as usize);
    vec.insert(0, tag.to_string());
    cache.insert(set_index.to_string(), vec);

    check_cache(&mut cache, &mut hits, &mut misses, &mut evictions, &tag, &set_index, &e);
    assert_eq!(misses, 0);
    assert_eq!(hits, 1);
    assert_eq!(evictions, 0);
    assert!(cache.contains_key(&set_index));
    assert_eq!(cache.get(&set_index).unwrap(), &vec![tag]);
}

// test the check_cache function by passing on an initialised cache and checking the output:
#[test]
fn check_cache_3() {
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut hits: i32 = 0;
    let mut misses: i32 = 0;
    let mut evictions: i32 = 0;

    let set_index: String = String::from("1001001");
    let tag: String = String::from("1001");
    let e: i32 = 1;

    // on the same set_index key, insert a different tag, to invoke a miss + eviction:
    let mut vec = Vec::with_capacity(e as usize);
    vec.insert(0, "0110".to_string());
    cache.insert(set_index.to_string(), vec);

    check_cache(&mut cache, &mut hits, &mut misses, &mut evictions, &tag, &set_index, &e);
    assert_eq!(misses, 1);
    assert_eq!(hits, 0);
    assert_eq!(evictions, 1);
    assert!(cache.contains_key(&set_index));
    assert_eq!(cache.get(&set_index).unwrap(), &vec![tag]);
}

// test the check_cache function by passing on an initialised cache and checking the output:
#[test]
fn check_cache_4() {
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut hits: i32 = 0;
    let mut misses: i32 = 0;
    let mut evictions: i32 = 0;

    let set_index: String = String::from("1001001");
    let tag_1: String = String::from("1001");
    let tag_2: String = String::from("0110");
    let e: i32 = 2;

    // on the same set_index key, insert a vector with 2 tags: [tag_2, tag_1]
    // the LRU principle should ensure the tags should be swapped in the cache.
    let mut vec = Vec::with_capacity(e as usize);
    vec.insert(0, tag_1.to_string());
    vec.insert(0, tag_2.to_string());
    cache.insert(set_index.to_string(), vec);

    check_cache(&mut cache, &mut hits, &mut misses, &mut evictions, &tag_1, &set_index, &e);
    assert_eq!(misses, 0);
    assert_eq!(hits, 1);
    assert_eq!(evictions, 0);
    assert!(cache.contains_key(&set_index));
    // the LRU principle should ensure the tags are swapped:
    assert_eq!(cache.get(&set_index).unwrap(), &vec![tag_1, tag_2]);
}
