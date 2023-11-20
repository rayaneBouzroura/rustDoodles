fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is {x}");
    }
   println!("the value of x in the outer scope is {x}");
   //floating point datatype
   
   let x = 2.0;

   let y : f32 = 3.0;

   //numeric operations

   //addition
   let sum = 5+10;

   //subtraction
   let difference = 95.5 -4.3;

   //multiplication
   let product = 4 * 10;

   //division
   let quotient = 57.7 / 32.2;
   let truncated = -5 / 3;

   //remainder
   let remainder = 43 % 5;

   //boolean types
   let t = true;

   let f:bool = false; //explicit type annotation

   //char type
   let c = 'z';
   let z:char = 'Z';//explicit typing
   let heart_eyed_cat = '😻';//emojis are four bytes so yay

   //creating tuples (fixed size arrays)
   let tup : (i32 , f64 , u8) = (500,6.4,1);
   //binding them to variables
   let (x,y,z) = tup;
   println!("x is {x}"); //etc

   //accesing a tuple via index
    let x : (i32 , f64 , u8) = (500,6.4,1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    //array types (fixed num , singular datatype inside)
    let a = [1,2,3,4,5,6];

    //array w element repeating n times
    let a = [3;5];

    //accessing array elements
    let first = a[1];
    
    
   



}

