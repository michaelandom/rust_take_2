fn main() {
   let mut num=5;

   let r1= &num as *const i32;
   let r2= &mut num as *const i32;


   unsafe {

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
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
