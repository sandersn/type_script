extern crate nom;
#[derive(Debug)]
struct Id {
    text: String,
}
enum Exp {
    Number(i32),
    Paren(Box<Exp>),
}
fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}
fn id(i: &str) -> nom::IResult<&str, Id> {
    nom::sequence::pair(
        nom::character::complete::alpha0,
        nom::character::complete::alphanumeric0,
    )(i)
    .map(|(rest, (id1, id2))| {
        (
            rest,
            Id {
                text: format!("{}{}", id1, id2),
            },
        )
    })
}
fn expnum(i: &str) -> nom::IResult<&str, Exp> {
    match nom::character::complete::digit1(i) {
        Ok((rest, sn)) => match sn.parse::<i32>() {
            Ok(i) => Ok((rest, Exp::Number(i))),
            Err(_) => Err(nom::Err::Error((
                "number of out range probably",
                nom::error::ErrorKind::TooLarge,
            ))),
        },
        Err(e) => Err(e),
    }
}
fn expparen(i: &str) -> nom::IResult<&str, Exp> {
    match nom::sequence::tuple((
        nom::character::complete::char('('),
        exp,
        nom::character::complete::char(')'),
    ))(i)
    {
        Ok((rest, (_, e, _))) => Ok((rest, Exp::Paren(Box::new(e)))),
        Err(e) => Err(e),
    }
}
fn exp(i: &str) -> nom::IResult<&str, Exp> {
    nom::branch::alt((expnum, expparen))(i)
}
fn main() {
    println!("{:?}", hello_parser("hello"));
    println!("{:?}", hello_parser("hello world"));
    println!("{:?}", hello_parser("goobye hello again"));
}

#[cfg(test)]
mod tests {
    use crate::exp;
    use crate::hello_parser;
    use crate::id;
    use crate::Exp::Number;
    use crate::Exp::Paren;
    #[test]
    fn it_works() {
        assert_eq!(hello_parser("hello").unwrap(), ("", "hello"));
        assert_eq!(hello_parser("hello world").unwrap(), (" world", "hello"))
    }
    #[test]
    fn id_basic() {
        let (rest, res) = id("ab1m1").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res.text, String::from("ab1m1"));
    }
    #[test]
    fn exp_number() {
        let (rest, res) = exp("123").unwrap();
        assert_eq!(rest, "");
        match res {
            Number(n) => assert_eq!(n, 123),
            Paren(_) => panic!("didn't expect to see a parenthesis here"),
        }
    }
    #[test]
    fn exp_paren() {
        let (rest, res) = exp("(123)").unwrap();
        assert_eq!(rest, "");
        match res {
            Number(_) => panic!("Didn't expect a number here"),
            Paren(e) => match *e {
                Number(n) => assert_eq!(n, 123),
                Paren(_) => panic!("Didn't expect a paren here"),
            },
        }
    }
}
