use std::collections::VecDeque;

use proconio::input;
use proconio::marker::{Usize1,Chars};

use crate::graph;

    pub fn run(){
    //    easy_graph(); 
    // shortest_way();
    nibugraph();
    }

    fn graph(){
        // let mut graph:Vec<Vec<i32>>;
        let mut graph:Vec<Vec<i32>>=Vec::new();

        let inputs=[[1,2],[2,4],[2,5],[3,4],[1,5]];
        for i in 0..inputs.len(){
            graph[i].push(0);
        }
        for i in  1..inputs.len(){
        graph[inputs[i][0]].push(inputs[i][1] as i32); 
        graph[inputs[i][1]].push(inputs[i][0] as i32); 
        } 
    }

    fn easy_graph(){
        // 
        input! {
            n:usize,
            m:usize,
            gin:[[usize;2];m]
        }
        let mut ans=0;
        let mut graph:Vec<Vec<i32>>=vec![vec![]];

        for _ in 0..m{
            graph.push(vec![])
        }
        for i in  0..m{
        graph[gin[i][0]].push(gin[i][1] as i32); 
        graph[gin[i][1]].push(gin[i][0] as i32); 
        }
        for v in 1..graph.len(){
            let mut count_over_self=0;
            for i in 0..graph[v].len(){
                if v>graph[v][i] as usize{
                   count_over_self+=1; 
                }     
            } 
            if count_over_self==1{
                ans+=1;
            }
        }
        println!("{}",ans);
    }


    fn shortest_way(){
        input! {
            r:usize,
            c:usize,
            s:(Usize1,Usize1),
            g:(Usize1,Usize1),
            t:[Chars;r]
        }
        println!("{:?}",t);
        let mut searched=vec![vec![false;c];r];
        let mut que=VecDeque::new();
        que.push_back((s,0));
        searched[s.0][s.1]=true;
        let diffs=vec![(1,0),(-1,0),(0,1),(0,-1)];
        while !que.is_empty(){
            let ((y,x),d)=que.pop_front().unwrap();
            if x==g.0&&y==g.1{
                println!("{}",d);
            }
            for diff in &diffs{
                let y =(y as isize+diff.1) as usize;
                let x =(x as isize+diff.0) as usize;
                if searched[y][x]==false && t[y][x]=='#'{
                    que.push_back(((y,x),d+1));
                    searched[y][x]==false;
                }
            }
        }

    }


    fn nibugraph(){
        input! {
            n:usize,
            m:usize,
            gin:[[usize;2];m]
        }
        let mut graph=vec![vec![];n+1];
        for i in  0..m{
            graph[gin[i][0]].push(gin[i][1]); 
            graph[gin[i][1]].push(gin[i][0]); 
            }
        let mut colors:Vec<usize>=vec![0;n+1];
        for i in 1..n+1{
            if colors[i]==0{
                colors[i]=1;
                dfs(i, &mut colors, &graph)
            }
        }
        let mut answer=true;
        for i in 0..m{

            if colors[gin[i][0]]==colors[gin[i][1]]{
                answer=false;
            }
        }
        println!("{:?}",gin);
        println!("{:?}",graph);
        println!("{:?}",colors);
        
        println!("{}",answer);

        fn dfs(pos:usize,colors:&mut Vec<usize>,graph:&Vec<Vec<usize>>){
            for &i in &graph[pos]{
                if colors[i]==0{
                    colors[i]=3-colors[pos] as usize;
                    dfs(i,colors,graph);
                }
            }
        }
    }