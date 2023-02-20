#![allow(non_snake_case)]

// This function is what does the sorting. It also returns
// a value used for recursion.
fn quicksrt(Min: usize, Max: usize, Array: &mut Vec<usize>) -> usize {
    let Pivot: usize = Array[Max];
    let mut Index: i32 = Min as i32 - 1;
    let mut Temp: usize;

    for i in Min..Max {
        if Array[i] <= Pivot {
            Index += 1;
	    Temp = Array[i];
	    Array[i] = Array[Index as usize];
	    Array[Index as usize] = Temp;
	}
    }
    Index += 1;
    Temp = Array[Max];
    Array[Max] = Array[Index as usize];
    Array[Index as usize] = Temp;
    return Index as usize;
}

// This function partitions the array, and then repeats the process recursively.
fn quickpart(Min: usize, Max: usize, Array: &mut Vec<usize>){
    // In the case that we've reached what you can consider "the end"
    // of the recursion branch, we terminate.
    if Min >= Max {
        return;
    }
    let Partition: usize = quicksrt(Min, Max, Array);
    // This is done for memory safety purposes.
    if Partition != 0 {
        quickpart(Min, Partition - 1, Array);
    }
    
    quickpart(Partition + 1, Max, Array);
}

// This function acts as a wrapper to the other two functions that part and sort the array.
pub fn quicksort(Size: usize, Array: &mut Vec<usize>){
    quickpart(0, Size - 1, Array); 
}
