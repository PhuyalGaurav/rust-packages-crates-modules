use crate::school::admin::Employee;
use crate::school::class::Student;

pub mod school;
fn main() {
    let gaurav: Student = Student {};
    println!("I am {gaurav:?}");
    let ad: Employee = Employee {
        name: String::from("Abishek"),
        salary: 60,
        ethnicity: crate::school::admin::JHAPALI,
    };

    println!("{:?}", ad.ethnicity.name);
}
