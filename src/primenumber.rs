pub fn is_prime_number(age:i32) ->bool {
    let mut count = 2;
    while count*count < age{
        if age % count==0 {
            println!("'{}'sosujanaiyo",age);
            false;
        }
       count+=1; 
    };
    println!("'{}'sosuudayo",age);
    true
}

pub fn factorization_in_prime_number(n:i32){
    let mut numbers_factorize=Vec::new();
    let mut i=2;
    let mut s = n;
    while i*i<=s{
        while s%i==0{
            s/=i;
            numbers_factorize.push(i)
        }
        i+=1
    }
    if s>=2{
        numbers_factorize.push(s);
    }
    println!("{:?}",numbers_factorize);
}