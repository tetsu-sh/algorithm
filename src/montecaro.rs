use proconio::input;
use rand::prelude::*;

pub fn run(){
    double_circle_area_expectation();
}

fn double_circle_area_expectation(){
    // input!{
    //     n=:i32,
    // }
    let n=1000000;
    let mut m=0;
    let mut rng = rand::thread_rng();
    for _i in 0..n {
        let px=rng.gen_range(0.0..6.0);
        let py=rng.gen_range(0.0..9.0);
        if (px-3.)*(px-3.)+(py-3.)*(py-3.)<=9.{
            m+=1
        } else if (px-3.)*(px-3.)+(py-7.)*(py-7.)<=4.{
            m+=1;
        }
    }
    println!("{}",m);

}