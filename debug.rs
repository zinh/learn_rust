#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main(){
    println!("Debug integer: {:?}", 12);
    println!("Debug name: {1:?} is {0:?} and {name:?}", "Charly", "Mark", name="Buffalo");
    println!("Structure: {:?}", Structure(150));
    println!("Deep debug: {:?}", Deep(Structure(2016)));
}
