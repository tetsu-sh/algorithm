#[allow(unused_imports)]
use std::ops::Rem;

use proconio::input;

pub fn run(){
    prims_in_interval();
}

#[allow(dead_code)]
fn how_many_ways(){
    input! {
        n:usize,
        x:usize,
    }
    let mut ans=0;
    let max_a={
        if x/3>n{n} else {x/3}
    };
    println!("{}",max_a); 
    for a in 1..max_a+1{
        
         for b in a+1..n{
            println!("a:{} b:{}",a,b); 
            let c=x-a-b;
            if n>=c&&c>b{
                println!("a:{} b:{} c:{}",a,b,c); 

                ans+=1;
            }
         }
    } 
    println!("{}",ans); 
}

#[allow(dead_code)]
fn beautiful_rectangle(){
    input! {
        n:i64
    }
    let max_site_to_search=(n as f64).powf(0.5) as i64;
    println!("{}",max_site_to_search);
    let mut min_arround=100000000000000;
    for h in 1..max_site_to_search+1{
        if n%h==0{
            let arround=2*(h+(n/h));
            if arround<min_arround{
                min_arround=arround
            };
            
        }
    }
    println!("{}",min_arround);
}

#[allow(dead_code)]
fn large_lcm(){
    const MAX:u128=1_000_000_000_000_000_000;
    input! {
        a:u128,
        b:u128,
    }
    let gcd=gcd(a, b);
    let lcm:u128=(a/gcd)*b;
    if lcm>MAX{println!("large")}else{println!("{}",lcm)}

}
use num_traits::*;

#[allow(dead_code)]
fn gcd<T:PrimInt>(a:T,b:T)->T
{
    if a%b==T::zero(){
        b
    }else{
        gcd(b,a%b)
    }
}

#[allow(dead_code)]
fn maxmal_value(){
    input! {
        n:usize,
        b:[usize;n-1]
    }
    use core::cmp::min;
    let mut answer=b[0];
    for i in 0..n-2{
        answer+=min(b[i],b[i+1]);
    }
    // one liner
    // let ans = b
    //     .iter()
    //     .zip(b.iter().skip(1))
    //     .map(|(&b0, &b1)| b0.min(b1))
    //     .sum::<i32>();
    answer+=b[b.len()-1];
    println!("{}",answer)

}



#[allow(dead_code)]
fn score_sum_queries(){
    input! {
        n:usize,
        cn:[(usize,usize);n],
        q:usize,
        lrs:[(usize,usize);q],
    }
    let mut class_1_accumulative_sum:Vec<usize>=vec![0];
    let mut class_2_accumulative_sum:Vec<usize>=vec![0];
    for (i,(c,n)) in cn.iter().enumerate(){
        if *c==1{
            class_1_accumulative_sum.push(class_1_accumulative_sum[i]+*n);
            class_2_accumulative_sum.push(class_2_accumulative_sum[i]);
        }else{
            class_2_accumulative_sum.push(class_2_accumulative_sum[i]+*n);
            class_1_accumulative_sum.push(class_1_accumulative_sum[i]);
        }
    }
    for (l,r) in lrs{
        println!("{} {}",class_1_accumulative_sum[r]-class_1_accumulative_sum[l-1],class_2_accumulative_sum[r]-class_2_accumulative_sum[l-1])
    }
}


#[allow(dead_code)]
fn newton(){
    // y=e^x
    let mut a=1.0;
    let r=2.0;
    for _ in 0..10{
        let x=a;
        let y=a.exp();
        let sessen_a=y;
        let sessen_b=y-x*sessen_a;
        a=(r-sessen_b)/sessen_a;
        println!("{}",a)
    }
}




#[allow(dead_code)]
#[allow(clippy::collapsible_if)]
fn cooking(){
    input! {
        n:usize,
        mut t:[usize;n],
    }
    let sum:usize=t.iter().sum();
    let mut dp=vec![vec![false;sum+1];n];
    dp[0][0]=true;
    for i in 1..n{
        for j in 0..sum+1{
            if j<t[i]{
                if dp[i-1][j]{
                    dp[i][j]=true;
                }  
            }
            if j>=t[i]{
                if dp[i-1][j]||dp[i-1][j-t[i]]{
                    dp[i][j]=true;
                } 
            }
            
        }
    }
    let mut answer=1000000000;
    for i in 0..sum{
        if dp[n-1][i]{
            let cooking_time=i.max(sum-i);
            answer=answer.min(cooking_time);
        }
    }
    println!("{:?}",answer);


}



#[allow(dead_code)]
fn prims_in_interval(){
    input! {
        l:usize,
        r:usize
    }
    let mut prime=vec![true;r-l+1];
    let goal=(r as f64).powf(0.5)as usize;
    if l==1{
        prime[0]=false;
    }
    for i in 2..=goal{
        let start=(l + i - 1) / i * i;
        for j in (start..=r).step_by(i){
            if j==i{continue;}
            prime[j-l]=false;
        }
    }
    let answer=prime.iter().filter(|&p|*p).count();
    println!("{}",answer);
}