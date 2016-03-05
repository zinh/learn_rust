use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "(Min={}, Max={})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point(i32, i32);

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Structure(i32);

impl fmt::Display for Structure{
    // Signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex(i32, i32);
impl fmt::Display for Complex{
    // Signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "(real={},img={})", self.0, self.1)
    }
}


fn main(){
    let s = Structure(150);
    println!("Custom display: {}", s);

    let mimax = MinMax(-300, 400);
    let p = Point(12, 64);
    let c1 = Complex(12, 22);
    println!("Debug minmax: {0:?}", mimax);
    println!("Display minmax: {}", mimax);
    println!("Debug point: {0:?}", p);
    println!("Display point: {}", p);
    println!("Display complex: {}", c1);
}
