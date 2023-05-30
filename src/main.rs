fn main() {
    let x = 5;

    let x = x + 1;

    const TEST: u32 = 60*60;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("{TEST}")
}