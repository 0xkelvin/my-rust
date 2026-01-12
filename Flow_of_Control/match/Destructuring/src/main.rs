fn main() {
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        (0, y, z) => println!("First is 0, y is {:?}, z is {:?}", y, z),
        (1, ..) => println!("First is 1 and the rest doesnt matter"),
        (.., 2) => println!("Last is 2 and the rest desnt matter"),
        (3, .., 4) => println!("First 3, Last is 4 and the rest doesnt matter"),
        _ => println!("It doesnt matter what they are"),
    }

    let array = [1, -2, 6];
    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
    }
}
