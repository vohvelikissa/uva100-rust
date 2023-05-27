fn trenplusone_vec(nnumber:i32) -> Vec<i32> {
    let mut trenlist:Vec<i32> = Vec::new();
    let mut current_number = nnumber;
    while current_number >= 1 {
        println!("{}",current_number);
        if current_number%2==0 {
            current_number = current_number/2;
        } else {
            current_number = 3 * current_number + 1;
        }
        trenlist.push(current_number);
        if current_number == 1 {
            return trenlist;
        }
    }
    trenlist
}
fn main() {
    println!("{}",trenplusone_vec(99).len()+1);
}
