extern crate rust1;

use rust1::f1;


//#[cfg(windows)] 
extern crate winapi;
use std::io::Error;

//#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    //use winapi::um::winuser::{MB_OK, MessageBoxW};

    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        winapi::um::winuser::MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), winapi::um::winuser::MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}


#[test]
pub fn t1() 
{
    let ret;
    unsafe {
        ret = f1_c();
    }    
    assert_eq!( 2,  ret);

    let ret2;
    unsafe {
        ret2 = hb1_c();
    }
    assert_eq!( 55, ret2);
}

fn main() {
    println!("using f1 from lib");
    unsafe {
     f1_c();
    }

    println!("using hb1 from lib");
    unsafe {
     hb1_c();
    }

    print_message("hello windows from rust").unwrap();
}

extern {
    fn f1_c() -> i32;
    fn hb1_c() -> i32;
}