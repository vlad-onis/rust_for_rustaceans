
/// Any number of shared references are allowed and each of them is Copy
/// Only 1 mutable reference can exist
#[allow(dead_code)]
fn variable_control() {

    let mut x1 = 4;
    let y1 = 5;
    let x2 = &x1;
    
    let mut x3 = &x1; 
    x3 = &y1;  // You can change what x3 points to because it's mutable
    println!("{}", x3);

    let x3 = &mut x1; 

    *x3 = 10;
   
    // If this immutable borrow will be used later x3 will not be allowed to mutate x1 
    // because x2 expects it to be immutable
    // println!("{}", x2); 
    println!("{}", x3);
}

fn main() {

}