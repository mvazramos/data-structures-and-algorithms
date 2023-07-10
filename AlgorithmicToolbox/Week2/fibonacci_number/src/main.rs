use std::io;
use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE","full");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    
    let numbers: Vec<u128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = numbers[0];
    let m = numbers[1];
    println!("{}", fibonacci_huge(n,m));

}


fn fibonacci_huge(n: u128, m: u128) -> u128{
    

    let mut previous: u128 = 0;
    let mut current: u128  = 1;

    let r = n % pisano(m);

    if r==0{
       return 0;
    } else if r==1{
       return 1;
    }

    for _ in 0..r-1{
        let temp = current;
        current = (previous + current)%m;
        previous = temp;
    }

    return current%m;

}


fn pisano(l: u128) -> u128{

    let mut previous: u128 = 0;
    let mut current: u128  = 1;

    let mut result: u128=0;

    /* let max = l*l ; */
    for i in 0..l*l{
        let temp = current;
        current = (previous + current)%l;
        previous = temp;

        if (previous==0) & (current==1){
            result=i+1;
        }
    }
    return result;
}