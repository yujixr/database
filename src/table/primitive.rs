use std::cmp;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Boolean(bool),
    Integer(i128),
    Float(f64),
    String(String),
}

impl Eq for Primitive {}

impl Ord for Primitive {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self {
            Primitive::Boolean(a) => match other {
                Primitive::Boolean(b) => a.cmp(b),
                _ => cmp::Ordering::Equal,
            },
            Primitive::Integer(a) => match other {
                Primitive::Integer(b) => a.cmp(b),
                _ => cmp::Ordering::Equal,
            },
            Primitive::Float(a) => match other {
                Primitive::Float(b) => {
                    if a < b {
                        cmp::Ordering::Less
                    } else if a == b {
                        cmp::Ordering::Equal
                    } else {
                        cmp::Ordering::Greater
                    }
                }
                _ => cmp::Ordering::Equal,
            },
            Primitive::String(a) => match other {
                Primitive::String(b) => a.cmp(b),
                _ => cmp::Ordering::Equal,
            },
        }
    }
}
