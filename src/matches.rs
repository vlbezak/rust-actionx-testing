pub fn test_matches() {
    println!("Test matches");

    test_match_range(1 as i32);
    test_match_range(-1 as i32);
    test_match_range(20 as i32);
    test_match_range(40 as i32);
}

fn test_match_range(num: i32){
    match num {
        c if c >= 0 && c <= 30 => println!("arm 1: {}", c),
        c if c < 0 => println!("arm 2: {}", c),
        c => println!("default arm {}", c)
    }

}