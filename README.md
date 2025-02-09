# Rust UAF 

This demonstrates a classic UAF vulnerability.

When a `User` object is deleted and `Data` is created, since they're both the same size of 16 bytes, the memory allocator will typically reuse the freed `User` chunk for the new `Data` allocation. **NOTE:** the reason we use jemalloc is so that `User`'s chunk is reused for the `Data` allocation.

The User struct's `func` pointer overlaps with Data's `buf` pointer, both of which are at offset 0. By modifying Data's `buf`, we directly overwrite what the program thinks is the User's function pointer. Then, when we call the User's function, the `win` function should be invoked.

# Solution

The program prints the address of the `win` function for us.

From here, we
1. Create a user (option 1)
2. Delete the user (option 2)
3. Create data (option 3) - reuses freed User memory
4. Edit data (option 4) - overwrite buf with address of `win` function
5. Call user's function (option 5) - triggers execution of `win`

# Debugging

```
rust-lldb target/debug/rust-uaf
```

Can set breakpoints at specific line numbers using
```
b main.rs:68
```

Once set, you can list breakpoints with
```
breakpoint list
```
