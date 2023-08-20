/// `Chemical` defines the basic chemical composition of a segment of the world. <br>
/// There are no specific units of calculus in the fields.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Chemical {
    pub water: u32,    // 1mg
    pub metals: u32,   // 1mg
    pub nitrates: u32, // 1mg
    pub nitrites: u32, // 1mg

    // ======== organic ========
    pub glucose: u32, // 1mg
}
