use crate::world::TypeCell;

pub use crate::constants::genome::*;

/// `Genome` хранит в себе набор типов, которые должны преобрести клетки при делении.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Genome(pub [Gene; COUNT_GENES]);

/// `Gene` хранит набор индивидуальный приобретаемый набор свойств для клетки.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Gene {
    type_cell: TypeCell,
    children: [usize; 4],
}