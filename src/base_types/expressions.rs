use std::{
    fmt::{Debug, Display},
    mem::transmute,
};

use crate::parsing::{Parsable, Token};

use super::numbers::{CountdownNumberBaseType, NumberSystem};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Operator {
    Add = 1,
    Sub = 2,
    Mul = 4,
    Div = 8,
}
impl Parsable for Operator {
    fn parse(tokens: &mut std::collections::VecDeque<Token>) -> Result<Self, String> {
        match tokens
            .pop_front()
            .ok_or_else(|| String::from("Parse Error: Operator found nothing"))?
        {
            Token::Punctuation(a) => match a {
                '+' => Ok(Operator::Add),
                '-' => Ok(Operator::Sub),
                '*' => Ok(Operator::Mul),
                '/' => Ok(Operator::Div),
                a => Err(format!("Expected an operator found {a:?}")),
            },
            Token::Word(a) => match a.to_lowercase().as_str() {
                "add" => Ok(Operator::Add),
                "sub" => Ok(Operator::Sub),
                "mul" => Ok(Operator::Mul),
                "div" => Ok(Operator::Div),
                a => Err(format!("Expected an operator found {a:?}")),
            },
            Token::Number(a) => Err(format!("Expected an operator found {a:?}")),
        }
    }
}
impl Operator {
    pub fn apply<T: CountdownNumberBaseType, E: NumberSystem<T>>(
        &self,
        system: &E,
        one: T,
        other: T,
    ) -> Option<T> {
        match self {
            Operator::Add => system.add(one, other),
            Operator::Sub => system.sub(one, other),
            Operator::Mul => system.mul(one, other),
            Operator::Div => system.div(one, other),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        })
    }
}
impl AsRef<Operators> for Operator {
    fn as_ref(&self) -> &Operators {
        unsafe { transmute(self) }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Operators(u8);
pub struct OperatorIterator {
    current_bit: u8,
    value: u8,
}
impl Operators {
    pub const ALL: Operators = Operators(0xF);
}
impl IntoIterator for Operators {
    type Item = Operator;

    type IntoIter = OperatorIterator;

    fn into_iter(self) -> Self::IntoIter {
        OperatorIterator {
            value: self.0,
            current_bit: 1,
        }
    }
}

impl Iterator for OperatorIterator {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_bit == 0 {
            return None;
        }
        loop {
            let mask = self.current_bit;
            if mask & 0xF == 0 {
                return None;
            }
            self.current_bit <<= 1;
            if (self.value & mask) != 0 {
                // Safety: We know that it will be safe due to
                return Some(unsafe { transmute::<u8, Operator>(self.value & mask) });
            }
        }
    }
}
impl FromIterator<Operator> for Operators {
    fn from_iter<T: IntoIterator<Item = Operator>>(iter: T) -> Self {
        Operators(iter.into_iter().map(|a| a as u8).fold(0, |a, b| a | b))
    }
}

#[derive(Clone)]
pub enum Expression<T: CountdownNumberBaseType> {
    Value(T),
    Application(Operator, Box<Expression<T>>, Box<Expression<T>>, T),
}
impl<T: CountdownNumberBaseType> Debug for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(arg0) => write!(f, "Val {arg0:?}"),
            Self::Application(op, left, right, _) => write!(f, "App {op:?} ({left:?}) ({right:?})"),
        }
    }
}
impl<T: CountdownNumberBaseType> Display for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Value(t) => Display::fmt(t, f),
            Expression::Application(operator, left, right, ..) => {
                if f.alternate() {
                    write!(f, "(")?
                }
                write!(f, "{:#}", left)?;
                write!(f, " ")?;
                Display::fmt(operator, f)?;
                write!(f, " ")?;
                write!(f, "{:#}", right)?;
                if f.alternate() {
                    write!(f, ")")?
                }
                Ok(())
            }
        }
    }
}
impl<T: CountdownNumberBaseType> Expression<T> {
    pub fn get_value(&self) -> &T {
        match self {
            Expression::Value(t) => t,
            Expression::Application(.., t) => t,
        }
    }
    // TODO: 3 Match Talk here
    /// We assume that the children are all valid
    pub fn is_valid(&self) -> bool {
        match self {
            Expression::Value(_) => true,
            Expression::Application(operator, _, expr_right, value) => {
                if *value == T::ZERO {
                    return false;
                }
                match operator {
                    Operator::Add | Operator::Mul => match expr_right.as_ref() {
                        Expression::Value(_) => {}
                        Expression::Application(right_oper, _, _, _) => {
                            if right_oper == operator {
                                return false;
                            }
                        }
                    },
                    Operator::Sub => {}
                    Operator::Div => {}
                }

                true
            }
        }
    }
    pub fn re_eval<N: NumberSystem<T>>(&self, system: &N) -> T {
        match self {
            Expression::Value(t) => *t,
            Expression::Application(operator, left, right, _) => {
                let left = &left.re_eval(system);
                let right = &right.re_eval(system);
                operator.apply(system, *left, *right).unwrap()
            }
        }
    }
}
