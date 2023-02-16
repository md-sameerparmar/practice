// struct Employee{
//     name: String,
//     age: u8
// }

// fn main(){
//     let mut emp1 = Employee{name: String::from("Sameer"), age: 22};
//     let mut emp2 = Employee{name: String::from("Sagar"), age: 22};
//     println!("name is {} and age is {}", emp1.name,emp1.age);
//     println!("name is {} and age is {}", emp2.name,emp2.age);
//     emp2.age = 23;
//     display(emp1,emp2);

// }

// fn display(emp1:Employee,emp2:Employee){
//     // println!("name is {} and age is {}",emp1.name,emp1.age);
//     if emp1.age >= emp2.age{
//         println!("{} is bigger than {}", emp1.name,emp2.name);
//     }
//     else{
//         println!("{} is bigger than {}",emp2.name,emp1.name);
//     }
// }

// method in struct------------------------------------------------------------------------------------------------

struct Student {
    name: String,
    marks1: u8,
    marks2: u8
}

impl Student{
    fn marks_count(&self) -> u8{
        self.marks1 + self.marks2
    }
}

fn main(){
    let std1 = Student{name: String::from("Sameer"),marks1:10,marks2:5};
    let std2 = Student{name: String::from("Sagar"),marks1:15,marks2:10};
    println!("name is {} and marks1 is {}, marks2 is {} and total is {}", std1.name,std1.marks1,std1.marks2,std1.marks_count());
    println!("name is {} and marks1 is {}, marks2 is {} and total is {}", std2.name,std2.marks1,std2.marks2,std2.marks_count());
}