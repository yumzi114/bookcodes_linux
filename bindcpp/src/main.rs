use autocxx::{generate, prelude::*, safety};
include_cpp!(
    #include "input.h"
    safety!(unsafe_ffi)
    generate!("x3")    
    generate!("Test")
);





fn main() {
    println!(": {}",ffi::x3(122));;

    let mut test = ffi::Test::new().within_box();

    test.as_mut().inc();
    test.as_mut().inc();
    println!("{}",test.to_string().as_ref().unwrap().to_string_lossy());
}

