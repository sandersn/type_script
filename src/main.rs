extern crate nom;
#[derive(Debug)]
struct Id {
    text: String,
}
#[derive(Debug)]
enum Dec {
    Var { name: Id, init: Exp },
}
#[derive(Debug)]
enum Exp {
    Number(i32),
    Paren(Box<Exp>),
}
type Decs = Vec<Dec>;

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
fn semi(i: &str) -> nom::IResult<&str, (&str, char, &str)> {
    nom::sequence::tuple((
        nom::character::complete::multispace0,
        nom::character::complete::one_of(";"),
        nom::character::complete::multispace0,
    ))(i)
}
fn decs(i: &str) -> nom::IResult<&str, Decs> {
    nom::sequence::terminated(nom::multi::separated_list(semi, var), semi)(i)
}
fn var(i: &str) -> nom::IResult<&str, Dec> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("var"),
        nom::character::complete::multispace1,
        id,
        nom::character::complete::multispace0,
        nom::bytes::complete::tag(":="),
        nom::character::complete::multispace0,
        exp,
    ))(i)
    .map(|(rest, (_, _, name, _, _, _, init))| (rest, Dec::Var { name, init }))
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
    println!("{:?}", decs("var i := 12; var j := 14;"));
}

#[cfg(test)]
mod tests {
    use crate::decs;
    use crate::exp;
    use crate::id;
    use crate::var;
    use crate::Exp::Number;
    use crate::Exp::Paren;
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
        assert_eq!(format!("{:?}", res), "Paren(Number(123))");
    }
    #[test]
    fn var_simple() {
        let (rest, res) = var("var x := 1001").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            format!("{:?}", res),
            "Var { name: Id { text: \"x\" }, init: Number(1001) }"
        )
    }
    #[test]
    fn decs_single() {
        let (rest, res) = decs("var x := 1001;").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            format!("{:?}", res),
            "[Var { name: Id { text: \"x\" }, init: Number(1001) }]"
        )
    }
    #[test]
    fn decs_multi() {
        let (rest, res) = decs("var x := 1001; var y := 10;").unwrap();
        assert_eq!(rest, "");
        assert_eq!(format!("{:?}", res), "[Var { name: Id { text: \"x\" }, init: Number(1001) }, Var { name: Id { text: \"y\" }, init: Number(10) }]")
    }
}
