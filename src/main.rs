use std::alloc::{alloc, Layout};
use std::process::Command;
use std::ptr;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[repr(C, align(16))]
struct User {
    func: fn(),
    data: usize,
}

#[repr(C, align(16))]
struct Data {
    buf: *mut u8,
    len: usize,
}

fn not_win() {
    println!("[*] Normal function called");
}

fn win() {
    println!("[+] You win!");
    Command::new("/bin/sh").status().expect("Failed to spawn shell");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut user_ptr: *mut User = std::ptr::null_mut();
    let mut data: Option<Box<Data>> = None;
    
    // give user the win function address
    println!("[!] Win function address: {:p}", win as *const ());

    loop {
        println!("1. Create User");
        println!("2. Delete User");
        println!("3. Create Data");
        println!("4. Edit Data");
        println!("5. Call User Function");
        println!("6. Exit");
        print!("> ");
        
        let choice = read_line().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                if user_ptr.is_null() {
                    let u = Box::new(User {
                        func: not_win,
                        data: 0,
                    });
                    user_ptr = Box::into_raw(u);
                    println!("[DEBUG] User address: {:p}", user_ptr);
                    println!("[+] User created");
                } else {
                    println!("[!] User already exists");
                }
            }
            2 => {
                if !user_ptr.is_null() {
                    unsafe { Box::from_raw(user_ptr); }
                    println!("[+] User deleted");
                } else {
                    println!("[!] No user to delete");
                }
            }
            3 => {
                if data.is_none() {
                    // 2. Allocate filler objects to control heap layout
                    //let _filler = vec![Box::new([0u8; 16])];  // Same size as User/Data
                    
                    // 3. Manually allocate memory with User's layout
                    let layout = Layout::new::<User>();
                    let ptr = unsafe { alloc(layout) } as *mut Data;
                    
                    // 4. Initialize Data in same memory location
                    unsafe {
                        ptr::write(ptr, Data {
                            buf: std::ptr::null_mut(),
                            len: 0,
                        });
                    }
                    
                    data = Some(unsafe { Box::from_raw(ptr) });
                    println!("[+] Data forced into User memory at: {:p}", ptr);
                }
            }
            4 => {
                if let Some(d) = &mut data {
                    print!("[?] Enter buffer address (hex): ");
                    let input = read_line();
                    match usize::from_str_radix(&input, 16) {
                        Ok(addr) => {
                            d.buf = addr as *mut u8;
                            println!("[+] Buffer updated to {:p}", d.buf);
                        }
                        Err(_) => println!("[!] Invalid address"),
                    }
                } else {
                    println!("[!] Create data first");
                }
            }
            5 => {
                if !user_ptr.is_null() {
                    unsafe {
                        let user = &*user_ptr;
                        println!("[DEBUG] User address: {:p}", user_ptr);
                        println!("[DEBUG] Calling {:p}", user.func);
                        (user.func)();
                    }
                } else {
                    println!("[!] Create user first");
                }
            }
            6 => break,
            _ => println!("[!] Invalid choice"),
        }
    }
}