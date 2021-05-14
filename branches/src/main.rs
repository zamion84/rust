// fn main() {
//     //let number = 3;
//     let number = 7;
//     if number < 5 {
//         println!("condition was true");
//     } else{
//         println!("condition was false");
//     }
// }
// incorrect condition in if statement example
// fn main(){
//     let number = 3;
//     if number {
//         println!("number was three");
//     }
    
// }

// corrected if condition in if statement example
// fn main (){
//         let number =3;
//         if number != 0 {
//             println!("number was something other than zero");
//         }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     }   else if number % 3 == 0{
//         println!("number is divisible by 3");
//     }   else if number % 2 == 0{
//         println!("number is divisible by 2");
//     }   else {
//         println!("number is not divisible by 4, 3, or 2");
//     } 
// }

// fn main() {
//     let condition = true;
//     // let number = if condition { 5 } else { 6 }; 
//     //if expression must return the same type for each arm, example below of incorrect if expression
//     let number = if condition { 5 } else { "six" };
//     println!("the value of the number: {}", number);
// }
// loop example
// fn main() {
//     loop {
//         println!("again!");
//         break;
//     }
// }
// counter loop example
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter +=1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {}", result);
// }
//while loop example
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1 ;
//     }
//     println!("LIFTOFF!!!");
// }
// inefficient for loop example - looping through an array
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5{
//         println!("the value is: {}", a[index]);
//         index +=1;
//     }
// }
// efficient for loop example - looping through an array
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!{"LIFTOFFF!!!"}
}