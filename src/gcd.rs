
pub fn run() {
    let nums=vec![24,40,60,80,90,120];
    // let gcd_num=gcd_recursively(&mut nums);
    let gcd_num=gcd(&nums);
    let lcm_num=lcm(nums);
    println!("{}",gcd_num);
    println!("{}",lcm_num)

}

fn lcm(nums:Vec<i32>)->i32{
    let gcd = gcd(&nums);
    let mut nums_gcd=Vec::new();
    for i in &nums{
        nums_gcd.push(*i / gcd)
    }
    let mut x=1;
    for i in &nums_gcd{
        x*=*i
    }
    x*=gcd;
    x
}


fn gcd(nums: &Vec<i32>)-> i32{
    let mut new_nums=nums.clone();
    while new_nums.len()>1  {
        while new_nums[0]>=1 && new_nums[1]>=1 {
            if new_nums[0]>new_nums[1]{
                let new_num=new_nums[0]%new_nums[1];
                new_nums[0]=new_num;
            }else {
                let new_num=new_nums[1]%new_nums[0];
                new_nums[1]=new_num;
            }
        }
        if new_nums[0]>=1{
            new_nums.remove(1);
        } else {
            new_nums.remove(0);
        }
    }
    new_nums[0]
}

fn gcd_recursively(nums: &mut Vec<i32>)->i32{
    println!("ini:{:?}",nums);

    if nums[0]>=1 && nums[1]>=1{
        if nums[0]>nums[1]{
            let new = nums[0]%nums[1];
            println!("new:{}",new);
            let mut new_nums=nums.clone();
            new_nums[0]=new;
            gcd_recursively(&mut new_nums);
        } else {
            let new = nums[1]%nums[0];
            println!("new:{}",new);
            let mut new_nums=nums.clone();
            new_nums[1]=new;
            gcd_recursively(&mut new_nums);
        }
    }
    if nums[0]>=1{
        nums.remove(1);
        if nums.len()==1{
            println!("saidaikouyakusuuha1:{:?}",nums[0]);
            return nums[0]
        }
        gcd_recursively(nums)
    } else {
        nums.remove(0);
        if nums.len()==1{
            println!("saidaikouyakusuuha2:{:?}",nums[0]);
            nums[0]
        } else {
        gcd_recursively(nums)
    }
        }

}