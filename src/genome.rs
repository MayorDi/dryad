use crate::cell::TypeCell;

const COUNT_GENES: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Genome([TypeCell; COUNT_GENES]);