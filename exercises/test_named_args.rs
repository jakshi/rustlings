fn call_me(num: i32) {
    println!("Called with: {}", num);
}

fn main() {
    // This works - normal positional argument
    call_me(3);

    // This would be a compilation error - Rust doesn't support named arguments
    // call_me(num: 3);  // Error: expected expression, found `:`

    println!("If you uncomment the line above, this won't compile!");
}
