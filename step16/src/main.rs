


fn main() {

    a();   

}


fn a(){
    b();
}

fn b(){
    c(22);
}

fn c(number: i32) {
    if number == 22 {
        panic!("Error 22");
    }
}
