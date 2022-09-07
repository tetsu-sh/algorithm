use proconio::input;

pub fn run(){
    used_to_sing_song_together();
}

fn min_bill(){
    input!{
        n:usize,
    }
    let mut answer=0;
    answer+=n/10000;
    let n_deivided_10000=n%10000;
    answer+=n_deivided_10000/5000;
    let n_deivided_10000_divided_5000=n_deivided_10000%5000;
    answer+=n_deivided_10000_divided_5000/1000;
    println!("{}",answer)
}

fn interval_scheduling(){
    input!{
        n:usize,
        mut lrs:[(usize,usize);n]
    }
    lrs.sort_by(|a,b|a.1.cmp(&b.1));
    println!("{:?}",lrs);
    let mut current_time=0;
    let mut answer=0;
    for lr in lrs{
        if current_time<=lr.0{
            current_time=lr.1;
            answer+=1;
        }
    }
    println!("{}",answer);
}

fn used_to_sing_song_together(){
    input! {
        n:usize,
        mut a:[isize;n],
        mut b:[isize;n],
    }
    a.sort_unstable();
    b.sort_unstable();
    let mut answer=0;
    for i in 0..n{
        answer+=a[i].abs_diff(b[i]);
    }
    println!("{}",answer);
}