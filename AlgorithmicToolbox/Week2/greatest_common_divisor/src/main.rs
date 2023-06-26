use std::io;

fn main() {

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line!");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line!");
    
    let x: u128 = input1.trim().parse().expect("Please type an integer.");
    let y: u128 = input2.trim().parse().expect("Please type an integer.");
    println!("{}", gcd(x,y));

}


/* fn naive_gcd(a: u128, b: u128) -> u128{
    let mut best: u128=0;
    let max: u128 = a+b;
    for i in 1..max{
        let d: u128 = i;
        if (a%d==0) & (b%d==0){
            best = d;
        }
    }

    best
} */

fn gcd(a: u128,b: u128) -> u128{
    // Euclidean Algorithm 
    let aprime: u128;
    if b ==0{
        return a;
    } else{
        aprime = a%b;
    }

    gcd(b, aprime)
}