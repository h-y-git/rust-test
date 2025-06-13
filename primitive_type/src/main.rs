// 足し算
fn main() {
    let str = String::from("value add");
    let num = add_num(10,20);

    println!("{}={}",str ,num);
}

fn add_num(lhs: i32, rhs: i32) -> i32 
{
    lhs + rhs
}

