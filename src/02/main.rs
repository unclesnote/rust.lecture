
use std::io;
use rand::Rng;



#[allow(unused)]
pub fn main() {
    // https://doc.rust-kr.org/ch02-00-guessing-game-tutorial.html
    println!("02/main.rs");
    //--------------------------------------------------------------------------
    let testsum = test_sum();
    let mut guess = String::new();
    let rand = rand::thread_rng().gen_range(1..=100);

    println!("guess the number!");
    println!("please input your guess.");

    //------------------------------------------------------------------
    /*
    std::io::stdin()
    .read_line(&mut guess)
    .expect("fail to read line");
    */

    let rst = io::stdin().read_line(&mut guess);
    rst.expect("fail to read line");        // no warning even if not handle expect ?
    //------------------------------------------------------------------
    println!("testSum:{testsum}, guess: {guess}---, rand: {rand}")
    //------------------------------------------------------------------
}


fn test_sum() -> i32 {
    let x:i32 = 2;
    let y:i32 = 10;
    println!("x: {x}, y:{}", y);
    x + y
}