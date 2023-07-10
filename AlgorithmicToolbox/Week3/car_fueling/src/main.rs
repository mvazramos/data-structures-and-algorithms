use std::io;
use std::process;

fn main() {
    let mut input1 = String::new(); // d
    let mut input2 = String::new(); // m
    let mut input3 = String::new(); // n
    let mut input4 = String::new(); // stops

    // Distance between cities;
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let d: u128 = input1.trim().parse().expect("Please type a number.");

    if d<1 || d>100000{
        println!("{}",-1);
        process::exit(1);
    }

    // Maximum miles with full tank; 
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let m: u128 = input2.trim().parse().expect("Please type a number.");

    if m<1 || m>400{
        println!("{}",-1);
        process::exit(1);
    }

    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let n: u128 = input3.trim().parse().expect("Please type a number.");

    if n<1 || n>300{
        println!("{}",-1);
        process::exit(1);
    }

    io::stdin().read_line(&mut input4).expect("Failed to read line");
    let mut stops: Vec<u128> = input4.split_whitespace().map(|x| x.parse().unwrap()).collect();

    if !stops.windows(2).all(|w| w[0] < w[1]){
        println!("{}",-1);
        process::exit(1);
    }


    let max: u128 = stops[stops.len()-1];

    //Another restriction:
    //The distance between stops must be <m 

    if max>d{
        println!("{}",-1);
        process::exit(1);
    }

    if max==m{
        println!("{}",0);
        process::exit(1);
    }



    stops.push(d);
    stops.insert(0,0);


    let mut number_of_stops: u128 = 0;
    let mut cum_dist: u128 = 0;


    for i in 1..stops.len(){
        cum_dist+= stops[i] - stops[i-1];


        if (stops[i] - stops[i-1])>m{
            println!("{}",-1);
            process::exit(1);
        }

        if cum_dist>m{
            cum_dist=stops[i] - stops[i-1];
            number_of_stops+=1;
        }
        
    }

    println!("{}", number_of_stops);


}