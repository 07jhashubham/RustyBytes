pub fn test_iter() {

//  let  f1 = vec!["Apple", "Banana","Avocado", "Strawberry", "Chiku"];

//  let f2 = vec!["DragonFruit", "Avocado", "watermelon", "Kiwi" ];

//  let mut and_f1 = f1.iter();

//  and_f1.next();
//  and_f1.next();

//  println!("{:?}",and_f1.next().unwrap());

// let f3 = f1.iter().chain(&f2);

// f3.for_each(|e| println!("This is {}", e));


// let chek = f1.iter().map(|e| String::from(*e)).map(|mut e| {e.push_str(" fruit"); return e;} );

// let f = chek.map(|mut e| {e.push_str(" fruit"); return e;});

// chek.for_each(|e| println!("{}",e));


let firsst_names = vec!["Shubham", "Prem", "Ram", "Karan"];
let firstname = firsst_names.iter().map(|e| String::from(*e));


let last_name = vec!["Jha", "Singh", "Gautam", "Gill"];
let lastname = last_name.iter().map(|e| String::from(*e));

let full_name = firstname.zip(lastname);

full_name.skip(1).take(2).for_each(|e| println!("First name {} and Last name {}.", e.0,e.1));

// for (index, name) in full_name.enumerate() {
//     println!("{} {} {}", index, name.0, name.1);
// }

let food = vec![("burgur", 12), ("cake", 32), ("snacks", 12), ("rusk", 54)];

let a = food.iter().fold(0u32, | a, e| e.1 + a );
println!("{}",a);


let mut pek = food.iter().peekable();
pek.next();
let ch = pek.peek();

println!("{}", ch.unwrap().0);
println!("{}", pek.next().unwrap().0);



}