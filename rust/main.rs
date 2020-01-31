

pub fn main(){
    
    let mut my_string = "Rust and CCI are fun! 😉";

    println!("is_unique('{}') returns {}", my_string, unique::is_unique(my_string));
    println!("is_unique_no_ds('{}') returns {}", my_string, unique::is_unique_no_ds(my_string));

    my_string = "This workz!😉";

    println!("is_unique('{}') returns {}", my_string, unique::is_unique(my_string));

    println!("is_unique_no_ds('{}') returns {}", my_string, unique::is_unique_no_ds(my_string));


}