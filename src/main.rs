use rayon::prelude::*;

fn main() {
    let mut  numbers : Vec<i64> = vec![1, 2, 3, 4, 5];
        numbers.par_iter_mut().for_each(|x| *x += *x);
println!("Nombres au carré : {:#?}", numbers);
//let mut  squared_numbers  : &i64 = numbers.iter().map(|x| x * x);
   // println!("Nombres au carré : {:#?}", squared_numbers);
   //  use  test in  this  fonction   
   #[test]
   fn test_squared_numbers() {
       let mut  numbers : Vec<i64> = vec![1, 2, 3, 4, 5];
       numbers.par_iter_mut().for_each(|x| *x += *x);
       assert_eq!(numbers, vec![2, 4, 6, 8, 10]);
   }
}
