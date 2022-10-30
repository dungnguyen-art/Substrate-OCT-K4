use std::io;
fn main() {
    let input = "adbcdaDd".to_owned();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line.to_lowercase();

    
    let mut cnt = 0;
    let mut result = String::new();
    let mut tmp = String::new();
    for i in 0..input.len(){
        tmp = (input.as_bytes()[i] as char).to_lowercase().to_string();
        if tmp == line{
            cnt += 1;
        }
        else{
            result.push(input.as_bytes()[i] as char);
        }
    }
    println!("{},{}", cnt, result);

    
}
