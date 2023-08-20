use crate::world::TypeCell;

pub use crate::constants::genome::*;

/// `Genome` stores a set of types that cells should acquire during division.
#[derive(Debug, Clone, PartialEq)]
pub struct Genome(pub [Gene; COUNT_GENES]);

impl Default for Genome {
    // 1 ----+ <-+
    //       |   |
    // 2-+ <-+   |
    //   |       |
    // 3<+       |
    //   |       |
    // 4<+ >-+   |
    //       |   |
    // 5<----+ >-+
    //       |   |
    // 6--+ <+   |
    //    |      |
    // 7<-+------+
    //    |
    // 8<-+
    fn default() -> Self {
        let mut genome = Genome([Gene::default(); COUNT_GENES]);

        genome.0[1] = Gene::new(TypeCell::Producer, [2, 0, 0, 0, 0]);

        genome.0[2] = Gene::new(TypeCell::Builder, [3, 0, 0, 4, 0]);
        genome.0[3] = Gene::new(TypeCell::Consumer, [0; 5]);

        genome.0[4] = Gene::new(TypeCell::Builder, [5, 0, 0, 6, 0]);

        genome.0[5] = Gene::new(TypeCell::Builder, [7, 1, 1, 0, 0]);
        genome.0[7] = Gene::new(TypeCell::Builder, [0; 5]);

        genome.0[6] = Gene::new(TypeCell::Builder, [8, 0, 0, 1, 0]);
        genome.0[8] = Gene::new(TypeCell::Photosynthetic, [0; 5]);

        genome
    }
}

/// `Gene` stores an individual acquired set of properties for a cell.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Gene {
    pub type_cell: TypeCell,

    /// \[self, left, right, top, botton\]
    pub children: [usize; 5],
}

impl Gene {
    pub fn new(type_cell: TypeCell, children: [usize; 5]) -> Self {
        Self {
            type_cell,
            children,
        }
    }

    /// Get all children other than 0.
    pub fn get_count_active_division(&self) -> u8 {
        let mut count = 0;

        self.children.map(|i| {
            if i != 0 {
                count += 1;
            }
        });

        count
    }
}
