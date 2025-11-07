fn main(){
    let x=8;  //integer type of u64
    println!("the value is {x}");
    let y = 6.0 ;//the defualt f64
    println!("the value {y}");
    let z:f32=3433422323.0; //the float value of f32
    println!("the value is {z}");

    //---
    let add = 6 + 4;
    let sub = 6-3;
    let mul = 4*8;
    let div = 9/3;
    let rem = 8%4;
    println!("the values goes as {add},{sub},{mul},{div},{rem}");

    //---
    let t = true;
    let f:bool = false; //explicit type annotation 
    println!("the boolean values are {t},{f}");

    //--
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("the values are {c},{z},{heart_eyed_cat}");

    //compound types-->tuples and arrays
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //accessing elements in tuples
    let (a,b,c)=tup;
    println!("the 2nd values is {b},{a},{c}");
    //we can access by period(.) with the index value
    let five_hundred = tup.0;
    let six_point_four=tup.1;
    let one = tup.2;
    println!("{five_hundred},{six_point_four},{one}");


    //arrays
    let array = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let array1:[i32;5]=[1,23,3,4,5];
    let array2=[3;5];
    let value1 =array[0];
    println!("the value is {value1}");

}