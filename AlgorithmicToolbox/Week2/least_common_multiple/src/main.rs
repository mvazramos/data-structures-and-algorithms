use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let  numbers: Vec<u128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let x: u128 = numbers[0];
    let y: u128 = numbers[1];
    println!("{}", lcm(x,y));

}

fn lcm(a: u128,b: u128) -> u128{
    let product:u128 = a*b;
    let gcd: u128 = gcd(a,b);
    return product/gcd;
    
}

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