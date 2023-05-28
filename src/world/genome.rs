use crate::world::TypeCell;

pub use crate::constants::genome::*;

/// `Genome` хранит в себе набор типов, которые должны преобрести клетки при делении.
#[derive(Debug, Clone, Copy, PartialEq)]
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

        genome.0[1] = Gene::new(TypeCell::Producer, [2, 0, 0, 0]);

        genome.0[2] = Gene::new(TypeCell::Builder, [3, 0, 0, 4]);
        genome.0[3] = Gene::new(TypeCell::Consumer, [0; 4]);

        genome.0[4] = Gene::new(TypeCell::Builder, [5, 0, 0, 6]);

        genome.0[5] = Gene::new(TypeCell::Builder, [7, 1, 1, 0]);
        genome.0[7] = Gene::new(TypeCell::Builder, [0, 0, 0, 0]);

        genome.0[6] = Gene::new(TypeCell::Builder, [8, 0, 0, 1]);
        genome.0[8] = Gene::new(TypeCell::Photosynthetic, [0, 0, 0, 0]);

        genome
    }
}

/// `Gene` хранит индивидуальный приобретаемый набор свойств для клетки.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Gene {
    pub type_cell: TypeCell,
    
    /// \[self, left, right, top\]
    pub children: [usize; 4],
}

impl Gene {
    pub fn new(type_cell: TypeCell, children: [usize; 4]) -> Self {
        Self { type_cell, children }
    }

    pub fn get_count_active_division(&self) -> u8 {
        let mut count = 0;

        self.children.map(|i| {
            if i != 0 { count += 1; }
        });
        
        count
    }
}