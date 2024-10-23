
pub struct User {
    pub name: String,
    pub surname: String,
    pub age: u8,
    pub height: f32,
    pub weight: f32,
    pub gender: Gender,
    pub fitness_level: FitnessLevel,
    pub physical_limitations: String,
    pub time_available: f32,
    pub equipment: String,
}

pub enum Gender {
    Male,
    Female,
}

pub enum FitnessLevel {
    Beginner,
    Intermediate,
    Advanced,
}


