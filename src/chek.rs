


#[derive(Debug)]
#[allow(dead_code)]
enum CarColor {
    White,
    Blue,
    Gray,
    Pink,
    Green,
    Gold,
    Silver
}

#[derive(Debug)]
struct Vehical {
    brand: String,
    model: String,
    color: CarColor,
    year: u16,

}

impl Vehical {
    fn paint(&mut self, new_color: CarColor) {
        self.color = new_color;
    }

    fn create_vehical() -> Vehical {
        let f2 = Vehical{model:"default".to_string(),brand:"default".to_string(),color:CarColor::Gold, year: 2000};
        f2
    }
    fn year_add(&mut self, new_age: u16) {
        self.year += new_age;
    }
}

fn new_vehical() -> Vehical {
    // let vh = Vehical{brand:"Hondya".to_string(), model:"i20".to_string(),color:CarColor::Gold,year:2008};
    // vh.paint(CarColor::Pink);
    let mut vh = Vehical::create_vehical();
    vh.paint(CarColor::Pink);
    vh.year_add(43);
    vh

}


pub fn print_per() {
    let f1 = new_vehical();
    println!("{:?}",f1);
}