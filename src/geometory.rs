
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


fn categorize_circle_distance(){
    let c1=[3.,2.];
    let r1=3.;
    let c2=[6.,4.];
    let r2=7.;
    let center_distance=  (((c1[0]-c2[0])*(c1[0]-c2[0])+(c1[1]-c2[1])*(c1[1]-c2[1])) as f64).sqrt();
    if center_distance<((r1-r2) as f64 ).abs(){
        println!("1");
    }
    if center_distance== ((r1-r2) as f64 ).abs(){
        println!("2");
    }

    if r1+r2>center_distance && center_distance>((r1-r2) as f64 ).abs(){
        println!("3");
    }
    if center_distance==r1+r2{
        println!("4")
    }
    if center_distance>r1+r2{
        println!("5")

    }

}

  

