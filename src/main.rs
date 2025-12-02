use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);
    match divide(5.0, 0.0) {
        Ok(v) => println!("{:#?}", v),
        Err(e) => println!("{}", e),
    }

    match validate_email(String::from("maitreymishra23@gmail.com")) {
        Ok(..) => println!("email is valid"),
        Err(e) => println!("Not a valid email address{}", e),
    }
}

fn validate_email(email: String) -> Result <(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}
fn divide (a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cannot divide by 0"))
    } else {
        Ok(a / b)
    }

}
