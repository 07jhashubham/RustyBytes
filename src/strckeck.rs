use std::cell::Cell;

struct Person<'q> {
     name: Cell<&'q str>,
     last: String,
     age: u8,
     year: u16,
}


enum ColorChoise {
    Blue,
    Green,
    Pink,
    Yellow,
    Gray,
    Gold,
    Silver,
}


struct Car {
    brand: String,
    model: String,
    year: u16,
    color: ColorChoise,
}


struct PerName(String,String,u16);

fn create_per() -> PerName {
    PerName("Raju".to_string(),"Shrivastav".to_string(), 1432)
}

pub fn print_per() {
    let par = create_per();
    println!("{:?}",par);
}



fn create_car() -> Car {
    Car{brand:"Lamborgini".to_string(),model:"Cyran".to_string(),color:ColorChoise::Green, year:2008}
}

pub fn print_car() {
    let new_car = create_car();
    println!("{:?}",new_car);
}

 fn create_person() -> Person<'static> {
    let p1 = Person{name: Cell::from("Shubham"), last:"Jha".to_string(), age:32, year:1978};
    p1
}

pub fn print_person() {
    let per = create_person();
    println!("first Name: {0}, last: {1}, age: {2}, year: {3}", per.name.get(), per.last, per.age, per.year);
}

