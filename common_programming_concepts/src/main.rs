fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // bitwise
    let bitwise = ! 2;

    println!("sum {}, difference {}, product {}, quotient {}, remainder {}, bitwise {}", sum, difference, product, quotient, remainder, bitwise);

    let tup = (500, 6.4, 1);

    let (x, y, _z) = tup;

    println!("{}", y==tup.1);


    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    println!("third value of months : {}", months[2]);
    
    another_function(x);

    let a = {
        let x = 3;
        x + 1
    };
    
    println!("value of a {}", a);


    let x = plus_one(5);

    println!("The value of x is: {}", x);
 

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
