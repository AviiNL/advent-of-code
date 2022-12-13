use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone)]
pub enum Expression {
    // Literals
    Number(NumericLiteral),
    Array(ArrayLiteral),
    Pair(Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Array(a) => write!(f, "{}", a),
            Expression::Pair(p) => {
                let mut s = String::new();
                for (i, expr) in p.iter().enumerate() {
                    s.push_str(&format!("{}: {}\n", i, expr));
                }
                write!(f, "({})", s)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub value: i64,
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub(crate) elements: VecDeque<Expression>,
}

impl ArrayLiteral {
    pub fn new(elements: Vec<Expression>) -> Self {
        ArrayLiteral {
            elements: VecDeque::from(elements),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, element) in self.elements.iter().enumerate() {
            write!(f, "{}", element)?;
            if i < self.elements.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Expression>,
}
