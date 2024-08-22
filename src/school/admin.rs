#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub salary: u8,
    pub ethnicity: Ethnicity,
}

#[derive(Debug)]
pub struct Ethnicity {
    pub name: &'static str,
    location: &'static str,
    pub population: u64,
}

pub const JHAPALI: Ethnicity = Ethnicity {
    name: "Jhapali",
    location: "Jhapa",
    population: 1212,
};
