//! Expressions are the base of the countdown problem.
use std::{
    fmt::{Debug, Display},
    mem::transmute,
};

#[cfg(feature = "parsing")]
use crate::parsing::{Parsable, Token};

use super::numbers::{NumberSystem, NumberType};

///  Operator type represents an arithmetic binary operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Operator {
    /// This is addition `(+)`
    Add = 1,
    /// This is subtraction `(-)`
    Sub = 2,
    /// This is multiplication `(*)`
    Mul = 4,
    /// This is division `(/)`
    Div = 8,
}

#[cfg(feature = "parsing")]
impl Parsable for Operator {
    fn parse(
        tokens: &mut std::collections::VecDeque<Token>,
    ) -> Result<Self, String> {
        match tokens.pop_front().ok_or_else(|| {
            String::from("Parse Error: Operator found nothing")
        })? {
            | Token::Punctuation(a) => {
                match a {
                    | '+' => Ok(Operator::Add),
                    | '-' => Ok(Operator::Sub),
                    | '*' => Ok(Operator::Mul),
                    | '/' => Ok(Operator::Div),
                    | a => Err(format!("Expected an operator found {a:?}")),
                }
            }
            | Token::Word(a) => {
                match a.to_lowercase().as_str() {
                    | "add" => Ok(Operator::Add),
                    | "sub" => Ok(Operator::Sub),
                    | "mul" => Ok(Operator::Mul),
                    | "div" => Ok(Operator::Div),
                    | a => Err(format!("Expected an operator found {a:?}")),
                }
            }
            | Token::Number(a) => {
                Err(format!("Expected an operator found {a:?}"))
            }
        }
    }
}
impl Operator {
    /// Apply the operator to the operands using the given number system.
    #[inline(always)]
    pub fn apply<T: NumberType, E: NumberSystem<T>>(
        &self,
        system: &E,
        one: T,
        other: T,
    ) -> Option<T> {
        match self {
            | Operator::Add => system.add(one, other),
            | Operator::Sub => system.sub(one, other),
            | Operator::Mul => system.mul(one, other),
            | Operator::Div => system.div(one, other),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            | Operator::Add => "+",
            | Operator::Sub => "-",
            | Operator::Mul => "*",
            | Operator::Div => "/",
        })
    }
}
impl AsRef<Operators> for Operator {
    fn as_ref(&self) -> &Operators {
        // Safety: Operator has the same size and representation as Operators
        unsafe { transmute(self) }
    }
}
/// represents a collection of operators.
#[derive(Debug, Clone, Copy)]
pub struct Operators(u8);
/// an iterator through different operators
pub struct OperatorIterator {
    current_bit: u8,
    value: u8,
}
impl Operators {
    /// A value representing all the operators
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
                // Safety: We know that it will be safe due to the definition
                return Some(unsafe {
                    transmute::<u8, Operator>(self.value & mask)
                });
            }
        }
    }
}

impl FromIterator<Operator> for Operators {
    fn from_iter<T: IntoIterator<Item = Operator>>(iter: T) -> Self {
        Operators(iter.into_iter().map(|a| a as u8).fold(0, |a, b| a | b))
    }
}

/// A type representing an expression which can either be a value or an
/// application of an operator over two expressions.

#[derive(Clone)]
pub enum Expression<T: NumberType> {
    /// Stores a value
    Value(T),
    /// Stores the application of an operator over two expressions and the
    /// result there-of
    Application(T, Operator, Box<Expression<T>>, Box<Expression<T>>),
}
impl<T: NumberType> Debug for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | Self::Value(arg0) => write!(f, "Val {arg0:?}"),
            | Self::Application(_, op, left, right) => {
                write!(f, "App {op:?} ({left:?}) ({right:?})")
            }
        }
    }
}
impl<T: NumberType> Display for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | Expression::Value(t) => Display::fmt(t, f),
            | Expression::Application(_, operator, left, right) => {
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
impl<T: NumberType> Expression<T> {
    /// Gets the value of the expression
    pub fn get_value(&self) -> &T {
        match self {
            | Expression::Value(t) => t,
            | Expression::Application(t, ..) => t,
        }
    }
    pub fn is_valid(&self) -> bool {
        match self {
            | Expression::Value(_) => true,
            | Expression::Application(value, operator, _, expr_right) => {
                if *value == T::ZERO {
                    return false;
                }
                match operator {
                    | Operator::Add | Operator::Mul => {
                        match expr_right.as_ref() {
                            | Expression::Value(_) => {}
                            | Expression::Application(_, right_oper, _, _) => {
                                if right_oper == operator {
                                    return false;
                                }
                            }
                        }
                    }
                    | Operator::Sub => {}
                    | Operator::Div => {}
                }

                true
            }
        }
    }
    /// Re evaluate the value of the expression
    pub fn re_eval<N: NumberSystem<T>>(&self, system: &N) -> T {
        match self {
            | Expression::Value(t) => *t,
            | Expression::Application(_, operator, left, right) => {
                let left = &left.re_eval(system);
                let right = &right.re_eval(system);
                operator.apply(system, *left, *right).unwrap()
            }
        }
    }
    /// A test helper to check whether evaluation is correct
    pub fn check<N: NumberSystem<T>>(&self, system: &N) -> bool {
        self.re_eval(system) == *self.get_value()
    }
}
