pub fn test_iterators() {
    let v1:Vec<i32> = vec![1,2,3];
    let v1_iter = v1.iter();
    for value in v1_iter {
        println!("Got: {}",value);
    }
}