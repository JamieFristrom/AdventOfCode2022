use scanf::scanf;
use std::io;

fn main() {
    // input starting stacks
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut done = false;
    while !done {
        let mut input_buffer = String::new();
        match io::stdin().read_line(&mut input_buffer) {
            Ok(_) => { 
                for i in 0..9 {
                    let bytewise = input_buffer.as_bytes();
                    if i*4+1 < bytewise.len() {
                        let byte = bytewise[i*4+1];
                        if !byte.is_ascii_whitespace() {
                            let comparitor: u8 = b'1';
                            if byte==comparitor {
                                done = true;
                                break;
                            }
                            else {
                                stacks[i].push(byte);
                            }
                        }
                    }
                }
            }
            Err(msg) => {
                println!("msg: {msg}");
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
        println!("{:?}", stack);
    }
    // skip line
    let mut garbage_buffer = String::new();
    io::stdin().read_line(&mut garbage_buffer);
    
    // ok, read the moves
    loop {
        let mut number: usize = 0;
        let mut column_from: usize = 0;
        let mut column_to: usize = 0;
        if scanf!("move {} from {} to {}", number, column_from, column_to ).is_ok() {
            column_from -= 1;
            column_to -= 1;
            for _ in 0..number {
                println!("move from {column_from}");
                match stacks[column_from].pop() {
                    Some(block) => {
                        println!("move to {column_to}");
                        stacks[column_to].push(block);
                    }
                    None => {
                        debug_assert!(false);
                    }
                }
            }
        }
        else {
            break;
        }
    }
    for stack in &stacks {
        println!("{:?}", stack);
    }
    for stack in &mut stacks {
        match stack.pop() {
            Some(block) => {
                print!("{}", char::from(block));
            }
            None => {
                break;
            }
        }
    }
}