use std::fmt;

struct List(Vec<i32>);
impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let List(ref vec) = *self;
        try!(write!(f, "["));
        for (count, v) in vec.iter().enumerate(){
            if count != 0{try!(write!(f, ",")); }
            try!(write!(f, "{}", v));
        }
        write!(f, "]")
    }
}

fn main(){
    let vector = List(vec![1,2,3,4,5, 10]);
    println!("Vector display: {}", vector);
}
