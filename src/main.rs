pub struct Men {
    name: String,
    age: String,
}

trait Human {
    //fn speak() -> String;
    //fn eat() -> String;
    fn new() -> Self;
    fn name() -> String;
    fn age() -> String;
}

impl Human for Men {
    fn new() -> Self{
        Men{
            name: Men::name(),
            age: Men::age()
        }
    }
    fn name() -> String{
        String::from("Johnny Depp")
    }
    fn age() -> String{
        String::from("25")
    }
}


fn main() {
    let jacksparrow = Men::new();
    println!("My name is {}.", jacksparrow.name);
    println!("I'm {} years old.", jacksparrow.age);
}

#[cfg(test)]
mod test {
    use crate::{Men, Human};

    #[test]
    fn cast(){
        let result = Men::name();
        let expect = String::from("Johnny Depp");
        assert_eq!(result, expect);
    }
}