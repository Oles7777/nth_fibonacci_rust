use std::io;

fn main() {
    println!("Please enter the position of the n-th fibonacci sequnce digit you wish to get. Example: 6");

    let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("You did not enter a vild value");

        // Converting the previous string answer to a now a signed 32bit integer
        let number: usize = number.trim().parse().expect("Please enter a number.");
    fibonacci(number);
}
// fibonacci implementation only for the positive numbers
fn fibonacci(x: usize) {
    let mut a = vec![1,1]; // have to use a vector since arrays have fixed-size and you cant append them
    while a.len() < x {
        let last_entry = a[a.len()-1] + a[a.len()-2];
        a.push(last_entry);
    };
    let final_entry = a[a.len()-1];
println!("The value of the Fibonacci sequnce at the {x}th position is {final_entry}");
}