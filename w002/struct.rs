struct Nil;

struct Pair(i32, f64);

struct Point{
    x: f64,
    y: f64,
    }

struct Rectangle{
    p1: Point,
    p2: Point,
    }

fn main(){
    let point: Point = Point{x: 0.3, y: 0.4};
    println!("(x={}, y={})", point.x, point.y);
    let Point{x: m_x, y: m_y} = point;
    println!("(x={}, y={})", m_x, m_y);
    let rec = Rectangle{p1: Point{x: m_x, y: m_y}, p2: point};
    let _nil = Nil;
    let pair = Pair(12, 13.3);
    let Pair(a, b) = pair;
    println!("({},{})", a, b);
}
