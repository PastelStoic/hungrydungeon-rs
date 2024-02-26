pub struct Organ {
    pub health_current: i32,
    pub health_max: i32,
    pub attack: i32,
    pub defense: i32,
    pub capacity: i32,
    pub fullness_current: i32,
}

/// Determines specific behavior of an organ
pub enum OrganType {
    Generic,
    Womb,
    Breast,
    // slime organ, allowing control of which specific actors inside it are digested.
}
