use std::io;

pub fn run(){
    // total_study_time()
    withdrow_coin()

}

fn total_study_time(){
    let mut input=String::new();
    println!("n a bを入力");
    match io::stdin().read_line(&mut input){
        Ok(_)=>{
            let v:Vec<i32>=input.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
            println!("{:?}",v);
            let n= v[0];
            let a=v[1];
            let b=v[2];
            let answer:f32=n as f32 * (a as f32 / 3.+b as f32*2./3.);
            println!("answer{}",answer);

        }
        Err(err)=>{
            println!("{}",err);
        }
    }
}

fn withdrow_coin(){
    let mut input=String::new();
    println!("input n");
    _=io::stdin().read_line(&mut input);
    println!("{}",input);
    let mut answer:f32=0.;
    let n:i32 = input.trim().parse().ok().unwrap();
    for i in 0..n{
        answer+=n as f32 / (n-i) as f32;
    }
    println!("answer:{}",answer);
    
}
