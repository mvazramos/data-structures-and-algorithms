use std::io;
use std::convert::TryInto;

fn main() {
    let mut costs: Vec<f64> = Vec::new();
    let mut weights: Vec<f64> = Vec::new();

    let mut param = String::new();
/*     let mut input2 = String::new();
    let mut input_weights = String::new(); */

    io::stdin().read_line(&mut param).expect("Failed to read line.");
    let parameters: Vec<f64> = param.split_whitespace().map(|x| x.parse().unwrap()).collect();
    if parameters.len()!=2 {panic!("There must be only two integers!")};  

    let n: u64 = parameters[0] as u64;
    let capacity: f64 = parameters[1];


    for _ in 0..n {
        let mut input =  String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let values: Vec<f64> = input
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid number"))
            .collect();

        if values.len() != 2 {
            println!("Invalid pair. Please enter a cost and a weight.");
            continue;
        }

        costs.push(values[0]);
        weights.push(values[1]);
    }

    

    println!("{}",get_optimal_value(capacity, weights, costs))


}


fn get_optimal_value(w: f64, weight: Vec<f64>, cost: Vec<f64>)->f64{
    /*
    The backpack fits 9;
            (saffron, vanilla, cinnamon)
    weights: W = (4, 3, 5)        
    prices:   (5000, 200, 100)

    min(4,9) -> 4 
    W = 9 - 4 = 5
    5000*4 

    200*3 = 600

    100*2  -> 20800

    What is the most valuable loot?
        max 5000*u1 + 200*u2 + 100*u3
    s.t. :
        u1<= 4
        u2<= 3
        u3<= 5
        u1 + u2 + u3 <= 9
    */
    
    let mut weight = weight.clone();
    let mut cost   = cost.clone();

    
    let w: f64 = w as f64; 
    let mut capacity: f64 = w; 
    let mut check_weight: f64=0.;
    let mut result: f64 = 0.;


    while check_weight< w{

        if weight.len()==0{
            break;
        };
        
        let m: usize =  price_compute(cost.clone(), weight.clone()).iter()
                        .enumerate()
                        .max_by(|(_, &value1), (_, &value2)| value1.partial_cmp(&value2).unwrap()) // because its f64
                        .map(|(index, _)| index).unwrap();

        let amount: f64 =  min(weight[m] as f64, capacity) as f64;
        let value: f64 = (cost[m] as f64)*(amount/(weight[m] as f64));

        capacity-=weight[m];
        weight.remove(m);
        cost.remove(m);
       
        check_weight+= amount;
        result += value;
    } 
    
    (result*1000.0).round()/1000.0
}


fn min(a: f64, b: f64)-> f64{
    if a>b{
        return b
    }

    return a
}

fn price_compute(a: Vec<f64>, b: Vec<f64>) -> Vec<f64>{
    let mut price:Vec<f64> = Vec::new();
    let max:usize = a.len().try_into().unwrap(); 
    for i in 0..max{
        let x: f64 = a[i] as f64;
        let y: f64 = b[i] as f64;
        price.push(x/y);
    }

    price
}



