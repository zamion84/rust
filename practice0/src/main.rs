use std::io;
use std::convert::TryInto;
//use std::string;
//aim for this practice is to intake some text and output the length with error handling
fn main() {
    
    //let input = " ";
    let mut varstr = String::from("");
    io::stdin()
        .read_line(&mut varstr)
        .expect("Failed to read line");

    // let varstr: String =! varstr
    //     .trim()
    //     .expect("Index entered was not a number");
    
        let output = first(varstr);
    
        println!("{}", output);
            
}
fn first(x: String) -> i32 {
    let a = 32;
    (x.chars().count() + a).try_into().unwrap()
}


