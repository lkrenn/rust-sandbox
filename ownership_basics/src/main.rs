fn main() {
    let s1 = String::from("hello");
    let s2 = takes_ownership(s1);

    // The line below is commented out to avoid a compile-time error, as s1's ownership has been moved.
    // println!("{}, world!", s1); // This line would fail if uncommented because s1's ownership has been moved.
    println!("{}, world!", s2);

    let x = 5;
    makes_copy(x);
    // This works because i32 is Copy, so it's not moved but copied.
    println!("x is still accessible: {}", x);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn takes_ownership(some_string: String) -> String { // some_string comes into scope.
    println!("some_string comes into scope: {}", some_string);
    some_string // some_string is returned and moves out to the caller.
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("some_integer comes into scope: {}", some_integer);
    // some_integer goes out of scope. No special action needed because i32 is Copy.
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String.
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
