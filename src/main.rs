fn trenplusone_vec(nnumber:i32) -> Vec<i32> {
    let mut trenlist:Vec<i32> = Vec::new();
    let mut current_number = nnumber;
    while current_number >= 1 {
        //println!("{}",current_number);
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
fn trentrenxd(xmin:i32,xmax:i32) -> i32 {
    let mut biggest_value:i32 = i32::MIN;
    for n in xmin..=xmax {
        if biggest_value <= (trenplusone_vec(n).len()+1).try_into().unwrap() {
            biggest_value = (trenplusone_vec(n).len()+1).try_into().unwrap();
        }
    }
    return biggest_value;
}
fn main() {
    println!("{}",trentrenxd(1,10));
    println!("{}",trentrenxd(100,200));
    println!("{}",trentrenxd(201,210));
    println!("{}",trentrenxd(900,1000));
}
