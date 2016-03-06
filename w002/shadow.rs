fn main(){
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner: {}", short_lived_binding);
        let long_lived_binding = 10_f32;
        println!("shadow: {}", long_lived_binding);
    }
    println!("Outer: {}", long_lived_binding);
    let long_lived_binding = 'a';
    println!("Shadow outer: {}", long_lived_binding);
}
