extern crate nom;

fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}
fn is_alphanumeric(c: char) -> bool {
    c.is_alphanumeric()
}
fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic()
}
//fn id(i: &str) -> nom::IResult<&str, &str> {
    //nom::sequence::tuple(l: List)
    //nom::complete
    //nom::bytes::complete::take_while(is_alphanumeric)
//}
fn main() {
    println!("{:?}", hello_parser("hello"));
    println!("{:?}", hello_parser("hello world"));
    println!("{:?}", hello_parser("goobye hello again"));
}

#[cfg(test)]
mod tests {
    use crate::hello_parser;
    #[test]
    fn it_works() {
        assert_eq!(hello_parser("hello").unwrap(), ("", "hello"));
        assert_eq!(hello_parser("hello world").unwrap(), (" world", "hello"))
    }
    #[test]
    fn startup() -> Result<(), u32> {
        if true { Ok(()) } else { Err(12) }
    }
}