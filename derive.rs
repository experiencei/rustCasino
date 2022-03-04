#[derive(Debug, Clone , Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone , Copy)]
struct Employee {
    position: Position,
    work_hour : i64
}


fn print_employee(emp : Employee) {
    println!("{:?}" , emp)
}

fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hour: 40,
    };
    print_employee(me);
    print_employee(me);

}

//derive is  used instead of match for varying the items
// copy and clone make another copy of the struct or enum instead of passing ownership
// NOTE: that always use copy/clone for an entries of up to five.