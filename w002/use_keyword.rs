enum Status{
    Rich,
    Poor
}

enum Work{
    Civilian,
    Soldier
}

fn main(){
    use Status::{Poor, Rich};
    use Work::*;
    let dave = Poor;
    let mark = Civilian;
    match dave{
        Rich => println!("is rich"),
        Poor => println!("is poor")
    }
    match mark{
        Civilian => println!("is a civilian"),
        Soldier => println!("is a soldier"),
    }
}
