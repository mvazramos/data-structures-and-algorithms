use std::io; 
//use std::cmp;

fn main(){

    let mut input_size = String::new();
    let mut input_sequence= String::new();

    io::stdin().read_line(&mut input_size).expect("Failed to read line");
    let size: usize = input_size.trim().parse().expect("Please type a number.");

    io::stdin().read_line(&mut input_sequence).expect("Failed to read line");
    let numbers: Vec<u64> = input_sequence.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    assert_eq!(numbers.len(), size); // Check if sequence of numbers has the defined length

    println!("{}",maximum_pairwise_distance(numbers));


}


fn maximum_pairwise_distance(v: Vec<u64>) -> u64{

    let mut max1:u64 = 0;
    let mut max2:u64 = 0;
    let mut idx1:usize = 0;


    for (idx, value) in v.iter().copied().enumerate(){
        if value>= max1 {
            max1=value;
            idx1=idx;
        }
    }

    for (idx, value) in v.iter().copied().enumerate(){
        if value>= max2 && idx!=idx1 {
            max2=value;
        }
    }


   //max_product
   max1*max2
}

