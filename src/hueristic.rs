use proconio::input;
use std::collections::VecDeque;

pub fn run(){
    difference_optimizetion();
}

fn difference_optimizetion(){
    input!{
        n:usize,
        m:usize,
        abs:[(usize,usize);m]
    }
    let mut graph=vec![vec![];n+1];
    for i in 0..m{
        graph[abs[i].0].push(abs[i].1);
    }
    // [[2,5][1,4]]
    let mut que=VecDeque::new();
    let mut visited=vec![-1;n+1];
    que.push_back(1);
    visited[1]=0;
    while !que.is_empty(){
        let post=que.pop_front().unwrap();
        for next in graph[post].iter(){
            if visited[*next]==-1{
                if visited[post]>=120{
                visited[*next]=visited[post];
                }
                visited[*next]=visited[post]+1;
            };
            que.push_back(*next);
        }
    }
    for i in 1..n+1{
        println!("{}",visited[i])
    }
}

fn mod_sum(){
    input! {
        n:isize,
    }
    println!("{}",n*(n-1)/2)
}