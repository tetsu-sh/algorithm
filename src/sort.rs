pub fn run(){
    let mut nums=[2,4,10,3,5,8,7,20,14];
    select_sort(&mut nums);
    play_list();
    let mut nums:Vec<i32>=[2,4,10,3,5,8,7,20,14].to_vec();
    let sorted_nums=merge_sort(&mut nums);
    println!("{:?}",sorted_nums);
}


fn select_sort(nums:&mut [i32]){
    for i in 0..nums.len(){
        let mut min=i;
        let mut min_value=nums[i];
        for j in i+1..nums.len(){
            if min_value>nums[j]{
                min=j;
                min_value=nums[j];
            }
        }
        nums.swap(i, min);
    }
    println!("sorted:{:?}",nums)

}

fn merge_sort(nums:&mut Vec<i32>) -> Vec<i32>{
    if nums.len()==1{
        return nums.to_vec();
    }
    let mut l_nums:Vec<i32>=nums[0..nums.len()/2].to_vec();
    let mut r_nums:Vec<i32>=nums[nums.len()/2..nums.len()].to_vec();

    let mut sorted_l_nums=merge_sort(&mut l_nums);
    let mut sorted_r_nums=merge_sort(&mut r_nums);

    let mut sorted_nums=vec![];
    while sorted_nums.len()!=nums.len(){
        if sorted_l_nums.len()==0 {
            sorted_nums.push(sorted_r_nums.remove(0));
        } else if sorted_r_nums.len()==0 {
            sorted_nums.push(sorted_l_nums.remove(0));
        } else if  sorted_l_nums[0]>sorted_r_nums[0]{
            sorted_nums.push(sorted_r_nums.remove(0));
        } else if sorted_r_nums>sorted_l_nums{
            sorted_nums.push(sorted_l_nums.remove(0));
        }
    }
    sorted_nums

}

fn kaijo(num:i32) ->i32{
   if num==1{return num}
   kaijo(num-1)*num 
}

fn play_list(){
    let l=[2,3,4,5];
    let k=l[0];
    println!("{:?}{}",l,k);
    println!("{}",l[0]);
}
