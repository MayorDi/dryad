use nalgebra::Vector2;

use crate::{
    composition::*, 
    tissue::TypeTissue
};

pub struct Cell {
    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,

    pub type_tissue: TypeTissue,
}