use std::process::Command;

#[repr(C)]
struct User {
    func: fn(),
    data: usize,
}

#[repr(C)]
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
                    let d = Box::new(Data {
                        buf: std::ptr::null_mut(),
                        len: 0,
                    });
                    let data_addr = d.as_ref() as *const Data;
                    data = Some(d);
                    println!("[DEBUG] Data address: {:p}", data_addr);
                    println!("[+] Data created");
                } else {
                    println!("[!] Data already exists");
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