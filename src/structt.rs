
pub fn run(){
    let person_name=String::from("fdafds");
    let address=String::from("dakfds");
    let contact_method=String::from("fadf");
    let customer = Customer::new(person_name, address, contact_method);
    customer.buy();
}

#[derive(Debug)]
pub struct PersonName(String);

pub struct Address(String);

pub struct ContactMethod(String);

pub struct Customer {
    person_name:PersonName,
    address:Address,
    contact_method:ContactMethod,
}

// #derive['Debug'];
impl Customer {
    pub fn new(person_name:String,address:String,contact_method:String)->Self{
        Customer{
            person_name:PersonName(person_name),
            address:Address(address),
            contact_method:ContactMethod(contact_method)
        }
    }
    pub fn buy(self){
        println!("{:?}buy",&self.person_name);
    }
}




