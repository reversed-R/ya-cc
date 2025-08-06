use crate::validator::Type;

#[derive(Debug)]
pub struct VarDec {
    pub typ: Type,
    pub name: String,
}
