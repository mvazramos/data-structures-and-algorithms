use std::io;
use std::convert::TryFrom;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    
    let n: u128 = input.trim().parse().expect("Please type an integer.");
    println!("{}", efficient_fibonacci(n));

}


fn efficient_fibonacci(n: u128) -> u128{
    let size:usize = usize::try_from(n).unwrap();
    let mut vec: Vec<u128> = Vec::new();

    vec.push(0);
    vec.push(1);

    for i in 2..size+1{
        vec.push(&vec[i-1] + &vec[i-2]); 
    }

    vec[size].clone()
}