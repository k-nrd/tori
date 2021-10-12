use std::fmt;

#[derive(Debug, Clone)]
enum Term {
    Var(String),
    App(Box<Term>, Box<Term>),
    Lam(String, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Term::Var(s) => write!(f, "{}", s),
            Term::App(t1, t2) => {
                let t1_str = match t1.as_ref() {
                    Term::Lam(_, _) => format!("({})", t1),
                    _ => format!("{}", t1),
                };
                let t2_str = match t2.as_ref() {
                    Term::Var(s) => format!(" {}", s),
                    _ => format!("({})", t2),
                };
                write!(f, "{}{}", t1_str, t2_str)
            }
            Term::Lam(n, t) => {
                let t_str = match t.as_ref() {
                    Term::Lam(n2, t2) => format!(" {}{}", n2, t2),
                    e => format!(".{}", e),
                };
                write!(f, "λ{}{}", n, t_str)
            }
        }
    }
}

fn main() {
    let term = Term::Lam(String::from("x"), Box::new(Term::Var(String::from("x"))));
    println!("{}", term);
}
