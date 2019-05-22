use std::mem;
use std::ptr;

pub fn run() {
    println!("======== address ======");
    let var: i8 = 1;
    println!("address of var: {:p}", &var);

    // get the memory address of a variable
    let address: usize = 0x7ffc8f303130;
    unsafe {
        let val = *(address as *const usize);
        println!("value at {}: {:?}", address, val);
    }

    // set the value at a certain memory address
    unsafe {
        *(0x7ffc8f303130 as *mut usize) = 1;
        // Note that this invokes undefined behavior if 0x7ffc8f303130 is uninitialized. In that case, std::ptr::write should be used.
        std::ptr::write(0x7ffc8f303130 as *mut usize, 1);
    }

    let v1 = vec![vec![1,2,3]; 10];
    println!("Original address: {:p}", &v1);
    let mut v2;
    // Override rust protections on reading from uninitialized memory
    unsafe {v2 = mem::uninitialized();} 
    let addr = &mut v2 as *mut _;
 
    // ptr::write() though it takes v1 by value, v1s destructor is not run when it goes out of
    // scope, which is good since then we'd have a vector of free'd vectors
    unsafe {ptr::write(addr, v1)}
    println!("New address: {:p}", &v2);
}