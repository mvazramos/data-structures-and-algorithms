use std::io; 

fn main(){

    let mut input= String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let amount: u128 = input.trim().parse().expect("Please type a number.");

    println!("{}", get_change(amount));
}


fn get_change(m: u128) -> u128{
    /* 
    If the input is 126, first we check how many coins of 10 we need by taking the ratio
    126/10 =  12 (because 12*10 = 120)

    Then what value remains? 126%10 = 6  (the remainder)

    How many 5 coins do we need? 
    (126%10)/5 = 6/5 = 1

    How many 1 coins do we need?
    ((126%10)%5) =  1

     */
    let mut n:u128 = 0;
    n+=m/10;
    n+=(m%10)/5;
    n+=(m%10)%5;

    n
}