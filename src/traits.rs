use crate::{app::SDL, world::*};

/// Turning a `Segment` into a specific type, to simplify the `if let` construction.
pub trait ToBlock {
    /// Returns `Ok(Block)` if the conversion was successful.
    ///
    /// ```
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
    /// let segment = Segment::Dirt(Cell::default());
    /// let cell = segment.to_cell().unwrap();
    /// ```
    fn to_cell(self) -> Result<Cell, ()>;
}

/// Turning a `Segment` into a specific type, to simplify the `if let` construction.
pub trait ToAir {
    /// Returns `Ok(Air)` if the conversion was successful.
    ///
    /// ```
    /// let segment = Segment::Dirt(Air::default());
    /// let air = segment.to_air().unwrap();
    /// ```
    fn to_air(self) -> Result<Air, ()>;
}

/// Trait for simple behavior.
pub trait Behaviour {
    fn update(&mut self, world_read: &World, world: &mut World);
}

pub trait Render {
    fn render(&self, sdl: &mut SDL);
}

/// Provides methods for getting a position in a one-dimensional array.
pub trait Position {
    /// Gets the `self` position depending on the index.
    fn get_position(&self) -> VectorWrapper<usize>;

    /// Gets the `self` index depending on the position. \
    /// Has a default implementation.
    fn get_index(&self) -> usize {
        let (x, y) = self.get_position().into();
        get_index(x, y, SIZE_WORLD[0])
    }
}

/// Provides various methods for implementing mutations.
pub trait Mutation {
    /// Performs a classic random mutation.
    fn mutate(&mut self);
}

/// Provides methods for working with glucose.
pub trait Glucose {
    fn synthesize_glucose(&mut self, light: f32);
    fn get_glucose(&self) -> f32;
    fn set_glucose(&mut self, value: f32);
    fn glucose_to_energy(&mut self) {}
}
