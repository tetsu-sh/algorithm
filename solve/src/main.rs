use clap::Parser;
use proconio::input;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    func_num: u32,
}

pub fn main() {
    println!("i am solve");

    let args = Args::parse();
    match args.func_num {
        0 => three_three(),
        1 => three_seven(),
        _ => println!("miss"),
    }
}

fn three_three() {
    input! {
        n:usize,
        nums:[usize;n],
    }
    let mut min = 100000000000;
    let mut secondary_min = 1000000000000000000;
    for num in nums.iter() {
        if *num < min {
            min = *num;
            secondary_min = min;
        } else if *num < secondary_min {
            secondary_min = *num;
        }
    }
    println!("{}", min);
    println!("{}", secondary_min);
}

fn three_four() {
    input! {
        n:usize,
        nums:[usize;n],
    }
    let mut max_diff = 0;
    for i in 0..n {
        for j in i + 1..n {
            let diff = nums[i].abs_diff(nums[j]);
            if max_diff < diff {
                max_diff = diff
            }
        }
    }
    println!("{}", max_diff);
}

fn three_seven() {
    input! {
        n:String,
    }
    let num = n.len();
    let mut answer = 0;
    for bit in 0..(1 << (num - 1)) {
        let mut tmp = 0;
        for i in 0..(num - 1) {
            tmp *= 10;
            let c = n.chars().nth(i).unwrap();
            tmp += c as i32 - 48;
            if (bit & (1 << i)) != 0 {
                answer += tmp;
                tmp = 0;
            }
        }
        tmp *= 10;
        let c = n.chars().nth(num - 1).unwrap();
        tmp += c as i32 - 48;
        answer += tmp;
    }
    println!("{}", answer);
}
