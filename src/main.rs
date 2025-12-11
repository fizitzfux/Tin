// disable standard lib because it is useless
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tin::test_runner)]
#![reexport_test_harness_main = "test_main"]

use tin::println;
use tin::print;
use tin::memory;
use core::panic::PanicInfo;


// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tin::test_panic_handler(info)
}


// main function here
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Worlqeeee{}", "!");

    tin::init();


    // let mut i: usize = 0x1000;
    // let mut addr: usize = 0;
    // const BLKS: u16 = 385;
    // const Blk_size: u16 = BLKS*8;
    // while addr == 0 {
    //     if memory::block_free(i, Blk_size) {
    //         addr = i;
    //         print!(":");
    //         for u in (0..Blk_size).step_by(8) {
    //             print!("{:X}", memory::read_addr_u64(addr + u as usize));
    //         }
    //     }
    //     else {
    //         i += Blk_size as usize;
    //     };
    // }

    // println!("{}", memory::read_addr_u64(102000usize));

    let mut i: u32 = 0;
    while true {
        let blk = memory::alloc_block();
        if blk.id == 0 {break}
        i+=1;
        print!("{}", i);
    }

    // memory::alloc_block();

    // println!("{}", memory::read_addr_u8(0x1000));
    // for i in 0..0xFFF {
    // memory::write_addr_u8(0x1000+i, 69u8);
    // print!("{} ", i);
    // }
    // println!("{}", memory::read_addr_u8(0x1001));

    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    // const addr : usize = 0x9000;
    // let mut i : usize = 0;
    // const u : usize = 128;
    // const size : usize = 1;
    // let mut arr : [u8;u/size] = [0;u/size];
    // println!("Mem range from {:X} - {:X}", addr, addr+u);
    // while i < u {
    // arr[i/size] = memory::read_addr_u8(addr+(i*size));
    // i += size;
    // }
    // println!("{:02X?}", arr);

    #[cfg(test)]
    test_main();

    println!("SUCCESSS");

    // let mut i:u8 = 0;
    // while true {
    //     if i % 16 == 0 {
    //         println!();
    //     }
    //     print!("{} ", i as char);
    //     if i == 127 {
    //         break;
    //     };
    //     i += 1;
    // };
    // println!();

    loop {}
}
