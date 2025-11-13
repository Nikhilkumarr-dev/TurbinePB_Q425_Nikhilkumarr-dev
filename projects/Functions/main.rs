fn main(){
    another_function();
    measurement(4,'c');
}   
fn another_function(){
    let x = 7;
    println!("This is the another function");
    println!("the value of  x is {x}");
}

fn measurement(label:i32,unit_label:char){
    println!("the label and char are {label} {unit_label}");
}