enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
  match expr {
    Expr::Value(val) => *val,
    Expr::Add(a, b) => eval(a) + eval(b),
    Expr::Mul(a, b) => eval(a) * eval(b)
  }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Value(2)),
        Box::new(Expr::Mul(
            Box::new(Expr::Value(3)),
            Box::new(Expr::Value(4)),
        )),
    );
    assert_eq!(eval(&expr), 14);
}
