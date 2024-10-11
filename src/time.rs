use std::{ops::Sub, time::Duration};

pub fn test_time(){

    let dur = Duration::from_secs(5);

    let dur2 = Duration::from_millis(5500);

    let dur3 = Duration::checked_sub(dur, dur2);


    println!("{}", dur3.unwrap_or_default().as_millis())

}