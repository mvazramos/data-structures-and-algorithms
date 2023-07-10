use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let  numbers: Vec<u128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let x: u128 = numbers[0];
    let y: u128 = numbers[1];
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