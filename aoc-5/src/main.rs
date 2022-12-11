use scanf::scanf;
use std::io;

fn get_blocks_to_grab( stack_from: &Vec<u8>, remaining: usize ) -> Vec<u8> {
    let (_, blocks_to_grab) = stack_from.split_at(remaining);

    blocks_to_grab.to_vec()
}

fn print_stacks(stacks: &Vec<Vec<u8>>) {
    for stack in stacks {
        println!("{:?}", stack);
    }
}

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
    }
    print_stacks(&stacks);
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
            let remaining = stacks[column_from].len()-number;
            println!("number: {number} remaining: {remaining}");
            let blocks_to_grab = get_blocks_to_grab(&stacks[column_from],remaining);
            //let stack_from = &stacks[column_from];
            //let blocks_to_grab_clone = blocks_to_grab.clone();
            println!("Moving {:?} from {} to {}", blocks_to_grab, column_from, column_to);
            let stack_to = &mut stacks[column_to];
            stack_to.extend_from_slice(&blocks_to_grab);
            stacks[column_from].truncate(remaining);
            print_stacks(&stacks);
        }
        else {
            break;
        }
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