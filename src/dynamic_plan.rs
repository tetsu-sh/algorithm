use std::vec;

use proconio::input;

pub fn run() {
    // frog_jump();
    // step_up();
    // sum_num_cards();
    // summer_vacation();


}

fn frog_jump(){
    let h = [8,6,9,2,1];
    let n = h.len();
    
    let mut dp:Vec<i32> = vec![0,((h[0]-h[1])as i32).abs()];
    for i in 2..n{
        let two_step= dp[i-2]+((h[i]-h[i-2]) as i32).abs();
        let one_step=dp[i-1]+((h[i]-h[i-1])as i32).abs();
        if two_step<one_step{ dp.push(two_step);}
        else { dp.push(one_step);}
        
    }
    println!("{:?}",dp);

}

fn step_up() {
    let n=6;
    let mut dp:Vec<i32>=vec![1,1];
    for i in 2..n+1{
        dp.push(dp[i-1]+dp[i-2]);
    }
    println!("{:?}",dp);
}


fn sum_num_cards(){
    println!("input S,N,Ai. sample is 3 11 (2,5,9) is yes");
    // input! {
    //     s:usize,
    //     n:usize,
    //     a:[usize;n],
    // }
    // println!("s:{} n:{} a:{:?}",s,n,a);
    const a:[usize;3]= [2,5,9];
    const s:usize = 11;
    const n:usize = 3;
    let mut dp=vec![[false]];
    let mut dp=[[false;s+1];n+1];
    for i in 0..s+1{
        dp[0][i]=false;
    }
    dp[0][0]=true;

    for  i in 1..n{
        for j in 0..s+1 {
            if j<a[i-1]{
                dp[i][j]=dp[i-1][j];
            }
            if j>=a[i-1]{
                if (dp[i-1][j]||dp[i-1][j-a[i-1]]){
                    dp[i][j]=true;
                } 
            }
        }
    }
    println!("{:?}",dp);
    let mut count=0;
    for i in 0..n{
        if dp[i][11]{count+=1}
    }
    println!("{}",count);

}


fn summer_vacation(){
    let a = [2,5,3,3,1];
    let n = a.len();
    let mut dp1=vec![0];
    let mut dp2=vec![0];
    for i in 1..n+1{
        dp1.push(dp2[i-1]+a[i-1]);
        if dp1[i-1]>dp2[i-1]{
            dp2.push(dp1[i-1])
        } else{
            dp2.push(dp2[i-1])
        }
    }
    println!("{:?} {:?}",dp1,dp2);
    println!("{:?} {:?}",dp1[n-1],dp2[n-1])

}