#[derive(Debug)]
struct Employee {
    name: String,
    salary: u32,
    empid: u32,
    emptype: String,
}

impl Employee {
    fn update_salary(&mut self) {
        if self.emptype == "JUNIOR ENGINEER" {
            self.salary = 50000;
        }
        else {
            self.salary = 60000;
        }
    }
}

fn main() {

    let mut emp1 = Employee {
        name: String::from("Sid"),
        salary: 0,
        empid: 1,
        emptype: String::from("JUNIOR ENGINEER"),
    };

    let mut emp2 = Employee {
        name: String::from("Ming"),
        salary: 0,
        empid: 2,
        emptype: String::from("SENIOR ENGINEER"),
    };

    let mut emp3 = Employee {
        name: String::from("Faith"),
        salary: 0,
        empid: 3,
        emptype: String::from("SENIOR ENGINEER"),
    };

    emp1.update_salary();
    emp2.update_salary();
    emp3.update_salary();

    println!("{:?}",emp1);
    println!("{:?}",emp2);
    println!("{:?}",emp3);
}
