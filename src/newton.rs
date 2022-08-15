pub fn run(){
    // newton()
    nibun_root();
}

fn newton(){
    let r= 2.;
    let mut a:f64= 2.;
    for _ in 1..5{
      let x=a;
    //   let y=a*a;
      let y=a*a*a;
    //   let y=a.powf(2./3.);
      
      let sessen_a=3.*x*x;
      let sessen_b=y-sessen_a*x;
      let next_a=(r-sessen_b)/sessen_a;
      println!("{}",next_a);
      a=next_a;
    }

}

fn nibun_root(){
    let mut l= 1.;
    let mut r= 2.;
    for i in 1..10{
        let m=(l+r)/2.;
        if m*m< 2.{
            l=m
        }
        if m*m>=2.{
            r=m
        }
        println!("{}kaime:{}",i,m);
    }
}