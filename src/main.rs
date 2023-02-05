fn main() {
    let mut n1 = 1;
    let mut n2: i32 =1;
    let mut n3: i32;
    println!("{}",n1);
    println!("{}",n2);
    for _i in 1..=30 {
        n3=n1+n2;
        println!("{}",n3);
        n1=n2;
        n2=n3;
    }
}
