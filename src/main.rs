#![allow(non_snake_case)]

use std::io;

mod arraygen;
use crate::arraygen::arraygen;
mod bubble;
use crate::bubble::bubblesort;
mod quick;
use crate::quick::quicksort;

const NUMOFALGS: usize = 1;

fn main() {

    println!("ACT's Rust Sorting Algorithm Implementation Demo");
    println!("\nPlease enter the length of the array you would like to sort.");
    println!("(Must be a positive integer.)");

    let SortTypes = ["0: Bubble Sort", "1: Quick Sort"];
    
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

    let _ArrayType: i32 = {

	let mut Buffer = String::new();
	loop {

            println!("\nPlease enter the type of sorting you'd like:");
            for i in 0..=NUMOFALGS {
                println!("{}",SortTypes[i]);
	    }
	    
	    io::stdin().read_line(&mut Buffer).unwrap();
            
	    match Buffer.trim().parse() {
                // In the case we entered a number...
		Ok(num) => match num {
		    // We then see if the number is between 0 and the maximum.
                    0..=NUMOFALGS => break num as i32,
		    _ => Buffer.clear(),
		},
		
		_ => Buffer.clear(),
	    }
	}
    };
   
    let mut Array = vec![0; ArraySize];
    
    arraygen(ArraySize, &mut Array);

    println!("\nOriginal Array:");
    println!("{:?}",Array);

    match _ArrayType {
        0 => bubblesort(ArraySize, &mut Array),
	1 => quicksort(ArraySize, &mut Array),
        _ => panic!("Oh boy! I need to really add safeguards!"),
    }
    
    println!("\nSorted Array:");
    println!("{:?}",Array);
   
}
