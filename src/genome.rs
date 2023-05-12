use crate::cell::TypeCell;

const COUNT_GENES: usize = 40;

/// `Genome` хранит в себе набор типов, которые должны преобрести клетки при делении.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Genome([TypeCell; COUNT_GENES]);