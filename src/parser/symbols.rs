pub mod expr;

pub struct Program {
    pub fns: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub content: Vec<Stat>,
}

pub enum Stat {
    ExprStat, // means `expr;` ex) `1 + 3;`, `i = 5;` `do_some();`

              // If,
              // While,
              // Declar, // `int x;`
              // Init, // `int x = 3;`
}
