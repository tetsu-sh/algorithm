// use algorythm::primenumber;
// mod gcd;
// mod structt;
// mod combination;
// mod expectation;
// mod montecaro;
// mod sort;
// mod dynamic_plan;
// mod geometory;
// mod newton;
// mod graph;
// mod count_add;
// mod hueristic;
mod greedy_algorithm;



fn main() {
    println!("Hello, world!");
    // primenumber::is_prime_number(23);
    // primenumber::factorization_in_prime_number(138);
    
    // gcd::run();  
    // combination::run();
    // expectation::run();
    // montecaro::run();

    // sort::run();
    // dynamic_plan::run();
    // geometory::run();
    
    // newton::run();
    // graph::run();
    // count_add::run();
    // hueristic::run();
    greedy_algorithm::run();

    // structt::run();
}

// use proconio::input;

// fn main(){
//     input! {
//         n:usize,
//         mut p:[usize;n]
//     }
//     let mut counter=vec![];
//     for i in 0..n{
//         let m=n-i;
//         let mut cursor=0;
//         loop {
//             if p[cursor]==m{
//              let diff=m-(cursor+1);
//              if diff >2{
//                 counter.push(("B",cursor+1));
//                 p.swap(cursor,cursor+2);
//                 cursor+=2;
//                 continue;
//              }else if diff==2 {
//                 counter.push(("B",cursor+1));
//                 p.swap(cursor,cursor+2);
//                 break;
//              }else if diff==1 {
//                 counter.push(("A",cursor+1));
//                 p.swap(cursor,cursor+1);
//                 break; 
//              }else {
//                 break;
//              }
             
//             }
//             cursor+=1;
//         }
//     }
//     println!("{}",counter.len());
//     // println!("{:?}",counter);
//     // println!("{:?}",p);
//     for (s,c) in counter{
//         println!("{} {}",s,c)
//     }
// }