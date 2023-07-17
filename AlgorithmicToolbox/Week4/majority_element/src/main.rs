use std::io;

fn main() {
    let mut q: String = String::new();
    let mut input = String::new();

    io::stdin().read_line(&mut q).expect("Failed to read line.");
    let _n: i128 = q.trim().parse().expect("Please type a number");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let l: i128 = 0;
    let r: i128 = (numbers.len()-1) as i128;
    if get_majority_element(&numbers, l, r)!=-1{
        println!("{}",1)
    }else{
        println!("{}",0)
    }
}

/*
    e.g.             [2,2,4,2,1,2]  -> n/2 = 3

    1º            [2,2,4] & [2, 1, 2]

    2º [2] & [2,4]                  [2] & [1,2]
        0      0                     0      0 
*/

fn get_majority_element(vec: &Vec<i128>, left: i128, right: i128)-> i128{
    if left==right{ return vec[left as usize]};

    let mid: i128 = (right+left)/2;

    let left_element = get_majority_element(vec, left, mid);
    let right_element = get_majority_element(vec, mid+1, right);

    

    if left_element==right_element{
        return left_element;
    }

    let count_left = frequency(vec, left, right, left_element);
    let count_right =  frequency(vec, left, right, right_element);

    let threshold:i128 = ((right-left+1)/2) as i128;
    
    
    if count_left > threshold{
        return left_element;
        
    }else if count_right > threshold{
        return right_element;

    }else{
        return -1;
    }

}

fn frequency(vec: &Vec<i128>, left: i128, right: i128, target: i128)->i128{

    let mut count = 0;

    for i in left..=right{
        if vec[i as usize]==target{ count += 1}
    }

    count

}