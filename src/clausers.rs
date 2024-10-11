pub fn test_clausers() {

    struct Person {
        name:String,
        last_name: String,
    }

    // let age = |x, y| x+y;
    // let ans = age(32f32, 68.2f32);

    // let result = |x| println!("result is {}", ans+x);
    // result(32f32);

    let mut p1 = Person{name: "Raju".to_string(), last_name:"Jha".to_string()};
    let mut change_name = |x:&str| p1.name = x.to_string();
    change_name("Kamal");

    // println!("{}", p1.name);

    change_name("Shayam");

    println!("{}", p1.name);

    let age = 43u16;

    





}