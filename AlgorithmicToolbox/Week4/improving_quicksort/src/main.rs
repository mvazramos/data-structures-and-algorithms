use std::io;

fn main() {
    let mut q: String = String::new();
    let mut input = String::new();

    io::stdin().read_line(&mut q).expect("Failed to read line.");
    let _n: i128 = q.trim().parse().expect("Please type a number");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers: Vec<i128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let l: i128 = 0;
    let r: i128 = (numbers.len()-1) as i128;
    quicksort(&mut numbers, l, r);

    for i in 0..numbers.len(){
        print!("{} ",numbers[i]);
        //result.push(qi);
    }
    println!();

}


fn quicksort(vec: &mut Vec<i128>, left: i128, right: i128){

    if vec.len()==1{
        return ;
    }

    if left>=right{
        return ;
    }

    //let m = partition2(vec, left, right);
    let mut m1:i128=0;
    let mut m2:i128=0;

    partition3(vec, left, right, &mut m1, &mut m2);

    quicksort(vec, left, m1-1);
    quicksort(vec, m2+1, right);

}




fn partition3(vec: &mut Vec<i128>, left: i128, right: i128,  p1:&mut i128 , p2: &mut i128){
    
    let x: i128 = vec[left as usize];
    let mut j = left;
    let mut k: i128 = right;

    let mut i = left + 1;
    while i<= k {

        if vec[i as usize]<x{
            vec.swap(i as usize, j as usize);
            j = j+1;
        
        }else if vec[i as usize]>x{
            vec.swap(i as usize, k as usize);
            k=k-1;
            i=i-1;
        
        } 

        i=i+1;
    }

    /*
       l [    <x     |       =x        |    >x     ] r
                    m1                 m2
        From l to m1-1 all elements that are smaller than x
        From m1 to m2 all elements that are equal to x
        From m2+1 to r all elements that are bigger than x;
    */
    
    *p1 = j;
    *p2 = k;
}

fn _partition2(vec: &mut Vec<i128>, left: i128, right: i128)-> i128{
    let x: i128 = vec[left as usize];
    let mut j = left;

    for i in left+1..right+1{
        if vec[i as usize]<=x{
            j = j+1;

            vec.swap(i as usize, j as usize);

        }
    }
    vec.swap(left as usize, j as usize);

    return j 
}