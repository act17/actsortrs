#![allow(non_snake_case)]

use std::io;

mod arraygen;
use crate::arraygen::arraygen;

mod bubble;
use crate::bubble::bubblesort;

fn main() {

    println!("ACT's Rust Sorting Algorithm Implementation Demo");
    println!("\nPlease enter the length of the array you would like to sort.");
    println!("(Must be a positive integer.)");
 
    // Thanks to B-Fuze32 for helping me with this check.
 
    let ArraySize: usize = {

	let mut Buffer = String::new();
	loop {

	    println!("Please enter a positive, whole number:");
	    io::stdin().read_line(&mut Buffer).unwrap();

	    match Buffer.trim().parse() {
		Ok(num) => break num,
		_ => Buffer.clear(),
	    }
	}
    };

    let _ArrayType: u8 = {

	let mut Buffer = String::new();
	loop {

            println!("\nPlease enter the type of sorting you'd like:");
	    println!("0: Bubble Sort");
	    io::stdin().read_line(&mut Buffer).unwrap();

	    match Buffer.trim().parse() {
                Ok(num) => break num,
		_ => Buffer.clear(),
	    } 
	}
    };
   
    let mut Array = vec![0; ArraySize];
    
    arraygen(ArraySize, &mut Array);

    println!("\nOriginal Array:");
    println!("{:?}",Array);
    
    bubblesort(ArraySize, &mut Array);
    
    println!("\nSorted Array:");
    println!("{:?}",Array);
   
}
