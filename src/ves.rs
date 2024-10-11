pub fn most() {
    let mut chek:Vec<u16> = Vec::new();

    chek.push(12);
    chek.push(132);
    chek.push(10);
    chek.push(980);
    chek.push(109);
    chek.push(1220);
    chek.push(12);
    chek.push(132);
    chek.push(10);
    chek.push(980);
    chek.push(109);
    chek.push(1220);
    

    println!("{:?}", chek);
    println!("{}", chek.len());
    println!("{}", chek.capacity());
    println!("{}", chek[9]);
    println!("{:?}", &(&chek).as_slice()[3..=5]);
}


pub fn most2() {
    let mut case = vec!["Jha", "raju", "Pinky", "Sanju"];
    case.push("shanti");

    for cas in case.as_slice() {
        println!("this is {} ", cas);
    }
    println!("{}", case[1]);

}
#[derive(Debug)]
struct Car {
    brand: String,
    year: u16
}

pub fn most3() {
    let mut name: Vec<Car> = vec![];
    let mut name2: Vec<Car> = vec![];

    for _ in 0..=100 {
        name.push(Car{brand: "Hyundai".to_string(), year: 2000});
    }
    for _ in 0..=100 {
        name2.push(Car{brand: "Toyota".to_string(), year: 2008});
    }

    name.append(&mut name2);
    let keep = |x: &Car| {if x.brand.starts_with("T") {true} else {false} };
    name.retain(keep);
    name.reserve(1000);
    println!("{:?}", name);println!("{:?}",name.capacity());
    println!("{:?}",name.len());
    

    
}
