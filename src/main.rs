





// pub mod helpers;
// pub mod matc;
// pub mod ensum;
// pub mod strckeck;
// pub mod chek;
// pub mod tree;
// pub mod ves;
// pub mod haseh; 
// pub mod testa_iter;
pub mod time;

fn main() {
    println!("Hello, world!");
    // age_fac();
    // test_clausers();
    // test_string();
    // let st = ensum::enu();
    // println!("this is num {}", st.unwrap());

    // let sst = ennu();
    // println!("this is your Character {}", sst.unwrap().to_string());

    // chek::print_per();

    // tree::che();

    // ves::most();
    // ves::most2();
    // ves::most3();

    // haseh::hassh_check();
    // haseh::hassh();

    // testa_iter::test_iter();

    time::test_time();

    




}










#[allow(dead_code)]
fn age_fac() {
    let age_limit = 16u8;
    
    println!("Type your age here...");

    let input_age = &mut String::from("");
    std::io::stdin().read_line(input_age).unwrap();

    let age = input_age.replace("\n", "").parse::<u8>().unwrap();

    if age > age_limit {
        println!("You can have licence for driving...");
    }
    else if age == 16 || age == 15 {
        println!("YOu can wait one year for licence...")
    }
    else {
        println!("Wait you are a child now...")
    }

    let driving_licence = if age > 16 {true} else { false};
    println!("{}", driving_licence);
}



#[allow(dead_code)]
fn out_ln() {
    let x:f32 = 322.2;
    let y:u8 = x as u8 - 0;
    println!("{}",y);

    let  prt = '🌊';
    println!("{}", prt);

    let name = ("Jha", "Kunal", 54 as u8);
    println!("{:?}", name);

    let arr: [u8; 7] = [43,12,32,122,23,12,44];
    println!("{:?}",arr);

    let new_arr = &arr[1..=6];
    println!("{:?}", new_arr);
}
