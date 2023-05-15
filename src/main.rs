const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    // shadowing vs mut
    let spaces = "   ";
    // let mut spaces = "   ";
    // spaces = spaces.len(); // error[E0308]: mismatched types
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
