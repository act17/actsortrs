// I'm going to be honest; the fact
// I need to do this is ridiculous.
#![allow(non_snake_case)]

fn main() {
    // Generated by random.org
    let mut Array: [i32; 10] = [9,3,8,7,4,6,0,1,5,2];
    let mut Offset = 0;
    let mut Current: i32 = 0;
    let mut Next: i32;
    
    // Printing the original array:
    println!("Original array:");
    for i in 0..10{
      println!("{}",Array[i]);
    }

    // This loop will run through each value to begin
    // the process of check-swapping.
    for a in 0..10{

      // This loop will actually check to see if numbers
      // must be swapped.
      for _b in 1..10 - a{
        Current = Array[Offset];
	Next = Array[Offset + 1];  

	// This process occurs in the case numbers must be swapped.
	if Current > Next{
          Array[Offset] = Next;
	  Array[Offset + 1] = Current;
	}
        else{
	  Current = Array[Offset + 1];
        }
	
	Offset = Offset + 1;
      }
	
      // This acts like a safeguard in case we've reached the final loop.
      if a != 9{
        Array[Offset] = Current;
      }

      // This resets our sorting method.
      Offset = 0;
    }

    println!("\nSorted array:");
    for i in 0..10{
      println!("{}",Array[i]);
    }
}
