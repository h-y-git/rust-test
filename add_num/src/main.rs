// 足し算
fn main() {
    let str = String::from("value add");
    let num = add_num(10,20);

    println!("{}={}",str ,num);

    let out: &str = "AAAA";
    line_out_char_num(out, 100);
}

fn add_num(lhs: i32, rhs: i32) -> i32 
{
    lhs + rhs
}

fn line_out_char_num(str :&str, num: i32)
{
    println!("String = {}, num = {}",str, num)
}
