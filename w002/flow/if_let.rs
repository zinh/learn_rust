fn main(){
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    if let Some(i) = number {
        println!("Matched: {:?}", i);
    }
    if let Some(i) = letter{
        println!("Matched: {:?}", i);
    }else{
        println!("None");
    }

    let i_like_letter = false;
    if let Some(i) = emoticon{
        println!("Matched: {:?}", i);
    }else if i_like_letter{
        println!("Don't match number");
    }else{
        println!("Emoticon");
    }
}
