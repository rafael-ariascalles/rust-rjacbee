fn main() {
    let n = 10;
    let mut message = "Hello".to_string();

    {
        let n = 50;
        println!("this is the inner number {n}");
    };

    let x = {
        let mut n = 50;
        println!("this is the inner number {n}");
        n = n + 10;
        n
    };

    print_value_plus(n);
    
    apply_template_and_print_message(&mut message);
    println!("this is the outter number {n}");
    println!("this is the second outter number {x}");
    println!("this is the message {message}");

}

//this is copy the value around so no problem with it
fn print_value_plus(mut n : i32) {
    n = n + 10;
    println!("this is the number within the function {n}");
}

//this is not copying the value around , it is been move and it takes ownership of the value. So this will clear the value from the main function
fn apply_template_and_print_message(s: &mut String) {
    *s = format!("{} World with modification ###############", s);
    println!("this is the message within the function {s}");
}