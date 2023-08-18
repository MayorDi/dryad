use crate::{app::SDL, world::*};

/// Turning a `Segment` into a specific type, to simplify the `if let` construction.
pub trait ToBlock {
    /// Returns `Ok(Block)` if the conversion was successful.
    ///
    /// ```
    /// use dryad::world::{ Segment, Block };
    /// use dryad::traits::ToBlock;
    /// 
    /// let segment = Segment::Dirt(Block::default());
    /// let block = segment.to_block().unwrap();
    /// ```
    fn to_block(self) -> Result<Block, ()>;
}

/// Turning a `Segment` into a specific type, to simplify the `if let` construction.
pub trait ToCell {
    /// Returns `Ok(Cell)` if the conversion was successful.
    ///
    /// ```
    /// use dryad::world::{ Segment, Cell };
    /// use dryad::traits::ToCell;
    /// 
    /// let segment = Segment::Cell(Cell::default());
    /// let cell = segment.to_cell().unwrap();
    /// ```
    fn to_cell(self) -> Result<Cell, ()>;
}

/// Turning a `Segment` into a specific type, to simplify the `if let` construction.
pub trait ToAir {
    /// Returns `Ok(Air)` if the conversion was successful.
    ///
    /// ```
    /// use dryad::world::{ Segment, Air };
    /// use dryad::traits::ToAir;
    /// 
    /// let segment = Segment::Air(Air::default());
    /// let air = segment.to_air().unwrap();
    /// ```
    fn to_air(self) -> Result<Air, ()>;
}

/// Trait for simple behavior.
pub trait Behaviour {
    /// ```
    /// use dryad::world::{ World, Segment, Block };
    /// use dryad::traits::{ ToBlock, Behaviour };
    /// 
    /// let mut world = World::new();
    /// let world_read = world.clone();
    /// 
    /// let segment: Segment = Segment::Dirt(Block::default());
    /// segment.to_block().unwrap().update(&world_read, &mut world);
    /// ```
    fn update(&mut self, world_read: &World, world: &mut World);
}

/// Required for rendering implementation.
pub trait Render {
    fn render(&self, sdl: &mut SDL);
}

/// Provides methods for getting a position in a one-dimensional array.
pub trait Position {
    /// Gets the `self` position depending on the index.
    /// 
    /// ```
    /// use dryad::world::{ Cell, TypeCell, VectorWrapper };
    /// use dryad::constants::world::SIZE_WORLD;
    /// use nalgebra::Vector2;
    /// use dryad::traits::Position;
    /// 
    /// let cell: Cell = Cell::new(Vector2::new(10, 2), TypeCell::default(), 1);
    /// assert_eq!(VectorWrapper::from((10, 2)), cell.get_position());
    /// ```
    fn get_position(&self) -> VectorWrapper<usize>;



    /// Gets the `self` index depending on the position. \
    /// Has a default implementation.
    /// 
    /// ```
    /// use dryad::world::{ Cell, TypeCell, VectorWrapper };
    /// use dryad::constants::world::SIZE_WORLD;
    /// use nalgebra::Vector2;
    /// use dryad::traits::Position;
    /// 
    /// let cell: Cell = Cell::new(Vector2::new(10, 2), TypeCell::default(), 1);
    /// assert_eq!(SIZE_WORLD[0] * 2 + 10, cell.get_index());
    /// ```
    fn get_index(&self) -> usize {
        let (x, y) = self.get_position().into();
        get_index(x, y, SIZE_WORLD[0])
    }
}

/// Provides various methods for implementing mutations.
pub trait Mutation {
    /// Performs a classic random mutation.
    /// 
    /// ```
    /// use dryad::world::{ Cell, TypeCell, VectorWrapper };
    /// use dryad::traits::Mutation;
    /// use nalgebra::Vector2;
    /// 
    /// let mut cell: Cell = Cell::new(Vector2::new(10, 2), TypeCell::default(), 1);
    /// cell.mutate();
    /// ```
    fn mutate(&mut self);
}

/// Provides methods for working with glucose.
pub trait Glucose {
    /// ```
    /// use dryad::world::{ Cell, TypeCell, VectorWrapper };
    /// use dryad::traits::Glucose;
    /// use nalgebra::Vector2;
    /// 
    /// let mut cell: Cell = Cell::new(Vector2::new(10, 2), TypeCell::default(), 1);
    /// cell.synthesize_glucose(1.0);
    /// ```
    fn synthesize_glucose(&mut self, light: f32);



    /// ```
    /// use dryad::world::{ Cell, TypeCell, VectorWrapper };
    /// use dryad::traits::Glucose;
    /// use nalgebra::Vector2;
    /// 
    /// let mut cell: Cell = Cell::new(Vector2::new(10, 2), TypeCell::default(), 1);
    /// 
    /// cell.set_glucose(10.0);
    /// let val = cell.get_glucose();
    /// 
    /// assert_eq!(10.0, val);
    /// ```
    fn get_glucose(&self) -> f32;



    /// ```
    /// use dryad::world::{ Cell, TypeCell, VectorWrapper };
    /// use dryad::traits::Glucose;
    /// use nalgebra::Vector2;
    /// 
    /// let mut cell: Cell = Cell::new(Vector2::new(10, 2), TypeCell::default(), 1);
    /// 
    /// cell.set_glucose(10.0);
    /// let val = cell.get_glucose();
    /// 
    /// assert_eq!(10.0, val);
    /// ```
    fn set_glucose(&mut self, value: f32);



    fn glucose_to_energy(&mut self) {}
}
