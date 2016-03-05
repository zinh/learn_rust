use std::fmt;
fn reverse(pair: (i32,bool)) -> (bool,i32){
    let (i, b) = pair;
    (b, i)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({} {})({} {})", self.0, self.1, self.2, self.3)
    }
}

fn tranpose(m: Matrix) -> Matrix{
    let Matrix(a,b,c,d) = m;
    Matrix(a,c,b,d)
}


fn main(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("Extract by index: {}", long_tuple.4);
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Debug tuple: {:?}", tuple_of_tuples);

    let pair = (12, false);
    println!("Debug pair: {:?}", pair);
    println!("Pair reverse: {:?}", reverse(pair));
    println!("Tuple with 1 element: {:?}", (12i32,));
    println!("Integer: {:?}", (12i32));
    let m = Matrix(1.2, 10.9, 0.12, 7.8);
    println!("Matrix: {}", m);
    println!("Tranpose matrix: {}", tranpose(m));

}
