use std::slice;
fn main() {
   let mut num=5;

   let r1= &num as *const i32;
   let r2= &mut num as *const i32;


   unsafe {

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
    println!("-3 c {}",abs(-3));
   }

   unsafe fn dangerous(){}


   unsafe {
    dangerous();
   }



   let mut v = vec![1,2,3,5,8];

   let n = &mut v[..];


   let (a,b) = n.split_at_mut(3);


   assert_eq!(a,&mut [1,2,3]);
   assert_eq!(b,&mut [5,8])



}


fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32] , &mut [i32]) {

    let len = slice.len();
    let ptr= slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
    
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub extern "C" fn call_from_c(){
    println!("from c")
}


