

pub fn test_string() {
    let arr = [500,600,700,900,1000,1100,1200];

    match arr[1..=3] {

        [600,700, ..] => {
            println!("THis is your range");
        }

        _ => {
            println!("This is not your range")
        }
        
    }
}

pub fn test_match() {
    let age:u16 = 4;

    match age {
        0 | 5 => {
            println!("this is 0")
        }
        1..=54 => {
            println!("this is the age 54")
        }
        55.. =>  {
            println!("this is the best case 54")
        }
        // _ => {
        //     println!("all this excluding")
        // }
    }
}