extern crate rand;
use rand::Rng;
//use std::collections::LinkedList;
use std::time::Duration;
use std::thread;


fn main(){
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Sorry. NOT read line.");
    let trimmed = input.trim();
    match trimmed.parse::<>(){
        Ok(num) => {
            let mut n = 0;
            while n< num{
                randword();
                n = n + 1;
                thread::sleep(Duration::from_millis(40));
            }
        },
        Err(..) => {
            println!("Sorry. NOT parsed.");
        },
    };
}

fn randword(){
    const ALPHAB: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(!@#$%*&";
    let mut rng = rand::thread_rng();
    let rw_len = rng.gen_range(1..32);
    let randword: String = (0..rw_len)
        .map(|_|{
            let idx = rng.gen_range(0..ALPHAB.len());
            ALPHAB[idx] as char
        })
        .collect();
        println!("{randword}");
}

