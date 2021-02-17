pub fn inspect(a : & String){
    if a.ends_with("s"){
        println!("plural");
    }
    else{
        println!("singular");
    }
}
pub fn change(a : &mut String){
    if !a.ends_with("s"){
        a.push_str("s");
    }
}
pub fn eat(a: String)->bool{
    if a.starts_with("b") && a.contains("a"){
        return true;
    }
    return false;
}
pub fn add(a: &i32, b: &i32) -> i32{
    return *a+*b;
}