use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    let n: i64 = input.trim().parse().expect("Not an integer!");
    let coins: Vec<i64> = vec![1,3,4];

    println!("{}",change(n, coins));
}

fn change(money: i64, coin: Vec<i64>) -> i64{
    
    let mut min_num_coins: Vec<i64>=vec![0; (money+1) as usize];
    min_num_coins[0]=0;
    
    for m in 1..=money{
        
        min_num_coins[m as usize] = 2*money;

        for i in 0..coin.len(){
        
            if m>=coin[i]{
        
                let num_coins = min_num_coins[(m-coin[i]) as usize] +1;
        
                if num_coins< min_num_coins[m as usize]{
                    min_num_coins[m as usize] = num_coins;
                }
            }
        }
    }

    return min_num_coins[money as usize]
}



