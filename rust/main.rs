

pub fn main(){
    
    let mut my_string = "Rust and CCI are fun! 😉";

    println!("s_unique('{}') returns {}", my_string, unique::is_unique(my_string));

    my_string = "This workz!😉";

    println!("s_unique('{}') returns {}", my_string, unique::is_unique(my_string));


}