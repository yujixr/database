use ordered_float::OrderedFloat;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Primitive {
    Boolean(bool),
    Integer(i128),
    Float(OrderedFloat<f64>),
    String(String),
}
