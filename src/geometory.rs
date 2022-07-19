
pub fn run(){
    min_distance_dots()
}

fn min_distance_dots(){
    let mut min=1.0e+10;
    let n:[[i32;2];3]=[[1,3],[2,6],[10,5]];
    for i in 0..n.len(){
        for j in i+1..n.len(){
            let distance=(((n[i][0]-n[j][0])*(n[i][0]-n[j][0])+(n[i][1]-n[j][1])*(n[i][1]-n[j][1])) as f64).sqrt();
            if distance<min{
                min=distance;
            }
        }
    }
    println!("{}",min);

}