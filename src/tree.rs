
#[allow(dead_code)]
#[derive(Debug)]
struct Person <PetType> where PetType: Animal + NotDangerous {
    first_name: String,
    pet :PetType
}

#[allow(dead_code)]
trait Animal {
    fn make_sound(&self) -> ();
}
trait NotDangerous {
    
}

#[allow(dead_code)]
#[derive(Debug)]
struct Dog {}
impl NotDangerous for Dog {
    
}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Barking Sound")
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Meww Sound");
    }
}
impl NotDangerous for Cat {
    
}
#[allow(dead_code)]
#[derive(Debug)]
struct Tiger {}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Roar Sound");
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct Bear {}

impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Russian SOund")
    }
}


pub fn che() {
    let pet1 = Dog{};
    let pet2 = Cat{};
    let pet3 = Bear{};
    let pet4 = Tiger{};
    let p1 = Person{first_name:"Jha".to_string(),pet:pet4};
    println!("{}", p1.first_name);
    p1.pet.make_sound();
}