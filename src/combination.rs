use rand::Rng;


pub fn run(){
    // choice_five_card();
    ncr();
    sansanyon();
    sansanroku();

}

// fn choice_five_card(){
//     let mut rng=rand::thread_rng();
//     let n:i32=rng.gen_range(5,100);
//     let a=rng.gen_range(1, 1000);
//     println!("{}",n);
//     println!("{}",a);
//     let total=1000;
//     let mut A=Vec<i32>::new();

// }

fn ncr(){
    let mut rng=rand::thread_rng();
    let r:i64=rng.gen_range(5,20);
    let n=rng.gen_range(r, 20);
    println!("r:{} n:{}",r,n);
    let mut f_n=1;
    let mut f_n_r=1;
    let mut f_r=1;
    for i in 1..n+1{f_n*=i};
    for i in 1..n-r+1{f_n_r*=i};
    for i in 1..r+1{f_r*=i};
    let ncr=f_n*f_n_r/f_r;
    println!("ncr:{}",ncr);


}

fn sansanyon(){
    let ai=[100,200,300,200,100,300,400];
    let mut count_100 =0;
    let mut count_200 =0;
    let mut count_300 =0;
    let mut count_400 =0;
    for i in ai{
        if i==100{
            count_100+=1;
        }
        if i==200{
            count_200+=1;
        }
        if i==300{
            count_300+=1;
        }
        if i==400{
            count_400+=1;
        }
    }
    let count=count_100*count_400+count_200*count_300;
    println!("count:{}",count);
}

fn sansanroku(){
    let mut cnt=[0;99999];
    
    let a=[2,4,6,1000,2,99998,99000];
    for i in 0..a.len(){
        cnt[a[i]-1]+=1;
    }
    println!("cntlen:{:?}",cnt.len());
    let mut answer=0;
    for i in 0..50000{
        answer+=cnt[i]*cnt[99998-i];
    }
    println!("answer:{}",answer);

}