#[derive(Debug)]
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


impl User {
    pub fn new(name: String, surname: String, age: u8, height: f32, weight: f32, gender: Gender, fitness_level: FitnessLevel, physical_limitations: String, time_available: f32, equipment: String) -> Self {
        User { name, surname, age, height, weight, gender, fitness_level, physical_limitations, time_available, equipment }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.name, self.surname)
    }
}


#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}


impl Gender {
    pub fn to_string(&self) -> String {
        match self {
            Gender::Male => "male".to_string(),
            Gender::Female => "female".to_string(),
        }
    }
}


#[derive(Debug)]
pub enum FitnessLevel {
    Beginner,
    Intermediate,
    Advanced,
}


impl FitnessLevel {
    pub fn to_string(&self) -> String {
        match self {
            FitnessLevel::Beginner => "beginner".to_string(),
            FitnessLevel::Intermediate => "intermediate".to_string(),
            FitnessLevel::Advanced => "advanced".to_string(),
        }
    }
}