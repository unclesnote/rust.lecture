
use std::io;
use rand::Rng;



#[allow(unused)]
pub fn main() {
    // https://doc.rust-kr.org/ch02-00-guessing-game-tutorial.html
    println!("02/main.rs");
    //--------------------------------------------------------------------------
    let i_func_num = test_sum();

    //------------------------------------------------------------------
    loop {
        let mut str_guess = String::new();
        let i_rand = rand::thread_rng().gen_range(1..=100);

        println!("please input your guess: ");
        io::stdin().read_line(&mut str_guess).expect("fail to read line");    // no warning even if not handle expect ?
        str_guess = str_guess.trim().to_string();

        println!("str_guess: {str_guess}, i_rand: {i_rand}");
        //let i_guess: i32 = str_guess.trim().parse().expect("please type a number");
        let i_guess: i32 = match str_guess.trim().parse() {
            Ok(mynum) => {
                println!("mynum: {mynum}");
                mynum
            },
            Err(_) => {
                println!("str_guess: {str_guess}, continue");
                continue
            } ,
        };
        //------------------------------------------------------------------
        match i_guess.cmp(&i_rand) {
            std::cmp::Ordering::Less => {
                println!("you are small, you({i_guess}) < rand({i_rand}) ")
            },
            std::cmp::Ordering::Greater => {
                println!("you are big, you({i_guess}) > rand({i_rand})")
            },
            std::cmp::Ordering::Equal => {
                println!("you are equel, you({i_guess}) == rand({i_rand})");
                break;
            },
        }
    }

    //------------------------------------------------------------------
}


fn test_sum() -> i32 {
    let x:i32 = 2;
    let y:i32 = 10;
    println!("x: {x}, y:{}", y);
    x + y
}