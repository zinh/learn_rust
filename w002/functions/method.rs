struct Point{
    x: i32,
    y: i32
}

impl Point{
    fn origin() -> Point{
        Point{x: 0, y: 0}
    }
    fn new(x: i32, y: i32) -> Point{
        Point{x: x, y: y}
    }
}

struct Rectangle{
    p1: Point,
    p2: Point
}

impl Rectangle{
    fn area(&self) -> i32{
        let Point{x: x1, y: y1} = self.p1;
        let Point{x: x2, y: y2} = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn translate(&mut self, x: i32, y: i32){
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair{
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self){
        let Pair(first, second) = self;
        println!("Destroy pair {} {}", first, second);
        // first and second get freed when out of scope
    }
}

fn main(){
    let p2 = Point::new(50, 21);
    let r = Rectangle{p1: Point::origin(), p2: p2};
    println!("Area: {}", r.area());
    let mut square = Rectangle{
        p1: Point::new(10,10),
        p2: Point::new(20,20)
    };
    square.translate(1, 1);
    let pair = Pair(Box::new(1), Box::new(10));
    pair.destroy();
    println!("{}", pair.1);
    // pair.destroy();
}
