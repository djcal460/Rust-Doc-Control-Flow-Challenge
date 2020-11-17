use std::io;

fn main() {
    

    //prompt for Fahrenheit
    println!("Enter the temperature in Fahrenheit: ");
    let f = prompt();

    //prompt for Celcius
    println!("Enter the temperature in Celsius: ");
    let c = prompt();

    //prompt for Celcius
    println!("Generate the nth Fib number: ");
    let fib = prompt();

    //convert f to c
    f_to_c(&f);

    //convert c to f
    c_to_f(&c);

    //print nth fib num
    fibonacci(fib);

    //print lyrics to 12 days of christmas
    let a: [String;12] = ["A Partridge in a Pear Tree".to_string(),"2 Turtle Doves".to_string(),
    "3 French Hens".to_string(),"4 Calling Birds".to_string(),"5 Golden Rings".to_string(),
    "6 Geese a Laying".to_string(),"7 Swans a Swimming".to_string(),"8 Maids a Milking".to_string(),
    "9 Ladies Dancing".to_string(),"10 Lords a Leaping".to_string(),"11 Pipers Piping".to_string(),
    "12 Drummers Drumming".to_string()];

    //christmas song
    println!("Do you want to hear a stupid song? Yes[1] No[0]");
    let song = prompt();
    if song == 1{
        stupid_song(&a);
    }else if song == 0{
        println!("Good, it's not worth your time.");
    }else{
        println!("Incorrect response");
    }
    


}

pub fn prompt() -> i32{
        //user input to stdin
        let f: i32 = loop{

            //get input
            let mut x= String::new();
            io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
    
       
            //parse user input
            let _x: i32 = match x.trim().parse(){
                Ok(num) => {
                    break num
                },
                Err(_) => continue, 
            };
    
        };//end loop

        //return f
        f
}

pub fn f_to_c(f: &i32){

    let c = (f - 32) * (5/9);
    println!("{} degrees Fahrenheit is {} degrees Celsius",f,c);

}

pub fn c_to_f(c: &i32){

    //need to do some typecasting
    let f = (*c as f64 * 1.8) + 32 as f64;
    let f: i32 = f as i32;
    println!("{} degrees Celsius is {} degrees Fahrenheit",c,f);

}

pub fn fibonacci(fib: i32){

    let x: f64 = 5.0;
    let z = fib as f64;
    let result = (1.0 / x.sqrt()) * ((1.0 + x.sqrt()) / 2.0).powf(z) - ((1.0 - x.sqrt()) / 2.0).powf(z); 
    let result = result.round() as i32;
    println!("The {} Fibonacci number is: {}",fib as i32,result);
}

pub fn stupid_song(arr: &[String]){
    
    println!("On the twelfth day of Christmas my true love gave to me:");
    for x in arr.iter(){
        println!("{}",x);
    }

}