use std::io;

fn main() {
    let mut input = String::new();
    let mut q: String = String::new();
    let mut input2 = String::new();
    let mut numberqueries: String = String::new();

    io::stdin().read_line(&mut q).expect("Failed to read line.");
    let _key: i128 = q.trim().parse().expect("Please type a number");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: Vec<i128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    io::stdin().read_line(&mut numberqueries).expect("Failed to read line.");
    let _nqueries: i128 = numberqueries.trim().parse().expect("Please type a number");
    
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let B: Vec<i128> = input2.split_whitespace().map(|x| x.parse().unwrap()).collect();


    //println!("{}", binary_multiple(A, B).iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
    binary_multiple(&A, &B);
    println!();
}



fn binary_search(A: &Vec<i128>, q: i128) -> i128{

    let mut min_index: i128=0;
    let mut max_index: i128 = (A.len()-1) as i128;

    if q<A[min_index as usize] || q>A[max_index as usize]{
        return -1;
    }

    while max_index>=min_index{
        let mid: i128 = (min_index+max_index)/2;
        //let index:usize = mid as usize;
        if A[mid as usize] == q{
            return mid;
        } else if A[mid as usize]<q{
            min_index = mid +1;
        } else{
            max_index = mid-1;
        }
    } 

    return -1;

}


fn binary_multiple(A: &Vec<i128>, B: &Vec<i128>){
    //let mut result: Vec<i128> = Vec::new();

    for i in 0..B.len(){
        print!("{} ",binary_search(A, B[i]));
        //result.push(qi);
    }

    //return result;
}
