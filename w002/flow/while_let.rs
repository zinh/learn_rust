fn main(){
    let mut op = Some(0);
    while let Some(i) = op{
        if i > 10{
            println!("Greater than ten, quit");
            op = None;
        }else{
            println!("Less than ten, continue");
            op = Some(i + 1);
        }
    }
}
