#![allow(non_snake_case)]

pub fn bubblesort(Size: usize, Array: &mut Vec<usize>){

    // Generated by random.org
    let mut Offset: usize = 0;
    let mut Current: usize = 0;
    let mut Next: usize;
    
    // This loop will run through each value to begin
    // the process of check-swapping.
    for a in 0..Size{

      // This loop will actually check to see if numbers
      // must be swapped.
        for _ in 1..Size - a{
            Current = Array[Offset];
	    Next = Array[Offset + 1];  

	// This process occurs in the case numbers must be swapped.
	if Current > Next{
            Array[Offset] = Next;
	    Array[Offset + 1] = Current;
	}
        else {
	    Current = Array[Offset + 1];
        }
	
	Offset = Offset + 1;
    }
	
      // This acts like a safeguard in case we've reached the final loop.
    if a != Size - 1{
        Array[Offset] = Current;
    }

      // This resets our sorting method.
        Offset = 0;
    }

}
