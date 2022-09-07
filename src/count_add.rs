use proconio::input;

pub fn run(){
    let ex_arr:Vec<i64>=vec![31 ,41 ,59 ,26 ,53];
    sum_of_dirrerence(ex_arr)
}

fn sum_of_dirrerence(mut arr: Vec<i64>){
    // let mut ex_arr:Vec<i64>=vec![1,4,2,5,8,7,10];
    // let sorted_arr=merge_sort(&mut ex_arr);
    arr.sort();
    // Ai*(N-2i+1)
    let n=arr.len() as i64;
    let mut sum:i64=0;
    for (i,num)in arr.into_iter().enumerate(){
        let i=i as i64;
        sum += num*(n-2*i+1);
    }
    println!("answer is:{}",sum);
}

fn manhattan(){
    input!{
        n:usize,
        xy:[(isize,isize);n]
    }
    let mut x=xy.iter().map(|&(x,_)|x).collect();
    let mut y=xy.iter().map(|&(_,y)|y).collect();
    x.sort();
    y.sort();
    let mut answer=0;
    for i in 0..n{
        let x_distance=x[i]*(2*i-n-1);
        let y_distance=y[i]*(2*i-n-1);
        answer+=x_distance+y_distance
    } 

}