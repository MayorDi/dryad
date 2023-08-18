/// `Chemical` defines the basic chemical composition of a segment of the world. <br>
/// There are no specific units of calculus in the fields.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Chemical {
    pub water: f32,
    pub metals: f32,
    pub nitrates: f32,
    pub nitrites: f32,

    // ======== organic ========
    pub glucose: f32,
}