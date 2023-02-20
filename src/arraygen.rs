#![allow(non_snake_case)]

/*
 * This function generates a pseudorandom array
 * from 1 to the size of the array inclusive.
 * Made by act, 2023
 *
 */

use rand::Rng;

pub fn arraygen(Size: usize, Array: &mut Vec<usize>){

    // Step 0: We fill the array with a progression
    // from 1 to the size.
    for n in 0..Size {
        Array[n] = n + 1;
    }

    // Step 1: We randomly determine an "offset". This
    // offset will be from 1 to the size of the array minus 1.
    // We also create "UD" - short for "Up-Down". It's a simple
    // 50%-50% switch of an operator to see if we swap up or down.
    let mut Offset: usize;
    let mut UD: usize;
    // We also also create a temporary buffer for the array values
    // to be used when swapping.
    let mut Buffer: usize;
    
    // Step 2: We then go through each element of the array, seeing
    // if we go Up (UD == 0) or Down (UD == 1) on the array.
    for n in 0..Size {

	// First we set the buffer.
        Buffer = Array[n];
        UD = rand::thread_rng().gen_range(1..=Size) % 2;
        Offset = rand::thread_rng().gen_range(1..Size);
	
	// Now begins the swapping.
	// We will swap upwards.
	if UD == 0 {
	    
	    // In the case our offset plus the current entry will overflow:
	    if Offset + n > Size - 1 {
		Array[n] = Array[(n + Offset) - Size];
		Array[(n + Offset) - Size] = Buffer;
	    }
	    // Otherwise, we just operate normally.
	    else {
                Array[n] = Array[n + Offset];
		Array[n + Offset] = Buffer;
	    }
	}
	
	// In case we swap downwards.
	if UD == 1 {

	    // This is just used for safety.
	    let TempCheck: i32 = n as i32;
            let TempOffset: i32 = Offset as i32;
	    
            if TempCheck - TempOffset < 0 {
                Array[n] = Array[Size - (Offset - n)];
		Array[Size - (Offset - n)] = Buffer;
	    }
	    else {
		Array[n] = Array[n - Offset];
		Array[n - Offset] = Buffer;
	    }
	}
    }
}
