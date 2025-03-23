// Title: Check if a number is prime

fn main(){
    let number = 111131;
    if is_prime(number){
        println!("{} is a prime number",number);
    }
    else {
        println!("{} is not a prime",number);
    }
}


fn is_prime(n:u32) -> bool{
    for i in 2..=((n as f64).sqrt() as u32){
        if n % i == 0 {
            return false
        }
    }
    true
}