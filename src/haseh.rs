
use std::collections::{HashMap, HashSet};

pub fn hassh_check() {
   let mut stick_price = HashMap::<String, f32>::new();
    println!("{}",stick_price.len());
    println!("{}", stick_price.is_empty());

    stick_price.insert("REL".to_string(), 2332.90);
    stick_price.insert("TCS".to_string(), 1998.25);
    stick_price.insert("INFO".to_string(), 550.45);
    stick_price.insert("TECHM".to_string(), 90.5);
    stick_price.insert("MDH".to_string(), 122.75);

    println!("{:?}",stick_price);

    stick_price.remove(&("TCS".to_string()));


    stick_price.entry("KNR".to_string()).or_insert(445.35);
    stick_price.entry("KNR".to_string()).or_insert(325.35);

    println!("{:?}",stick_price);

    for (name,price) in stick_price.clone(){
        println!("Stock {} price is {}.", name, price);
    }

    println!("{:?}", stick_price);

}

pub fn hassh() {
 let planet = HashSet::from(["Mercury", "Venus", "Earth", "Mars"]);

 let Outer_planet = HashSet::from(["Jupiter", "Saturn", "Uranas", "Neptune"]);

 let pl = planet.symmetric_difference(&Outer_planet);

 for p in pl {
    println!("This is {}", p);
 }
    
}