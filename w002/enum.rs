enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info{name: String, height: i32}
}

fn inspect(p: Person){
    match p{
        Person::Skinny => println!("is skinny"),
        Person::Fat => println!("is fat"),
        Person::Height(h) => println!("height: {}", h),
        Person::Weight(w) => println!("weight: {}", w),
        Person::Info{name, height} => {
            println!("name: {}, height: {}", name, height);
        }
    }
}

fn main(){
    let person = Person::Height(18);
    let danny = Person::Weight(10);
    let dave = Person::Info{name: "Dave".to_owned(), height: 180};
    let john = Person::Skinny;
    let larry = Person::Fat;
    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}
