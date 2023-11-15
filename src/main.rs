use std::ffi::c_int;
use std::io;
use std::fs::File;
use std::io::Write;
use rand::Rng;


fn get_input_from_keyboard(text_to_print: &str) -> String {
    let mut input_string = String::new();
    if !text_to_print.is_empty() && text_to_print.is_ascii() {
        println!("{}", text_to_print);
        io::stdin().read_line(&mut input_string).expect("FAILED TO READ LINE.");
    } else {
        panic!("ERROR: TEXT TO PRINT CAN'T BE EMPTY VALUE.");
    }
    input_string.trim().to_string()
}

fn gen_rand_al() -> u8 {
    let mut rng = rand::thread_rng();
    let _al = "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm";
    let rand_num = rng.gen_range(0..=_al.len());
    _al.as_bytes()[rand_num]
}

fn gen_rand_num() -> u8 {
    let mut rng = rand::thread_rng();
    let _num = "0123456789";
    let rand_num = rng.gen_range(0..=_num.len());
    _num.as_bytes()[rand_num]
}

fn get_rand_spec() -> u8 {
    let mut rng = rand::thread_rng();
    let _spec = "-_!=";
    let rand_num = rng.gen_range(0..=_spec.len());
    _spec.as_bytes()[rand_num]
}

fn gen_write_file(file_name: String, passwd: String) -> Result<(), std::io::Error> {
    let mut _file_to_write = File::create(file_name.to_owned() + ".txt")?;
    _file_to_write.write_all(passwd.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let _website_name = get_input_from_keyboard("INSERT THE SERVICE'S NAME.");
    let mut _changeable_string = String::new();
    println!("You entered {}", _website_name);
    for _ in 0..8 {
        _changeable_string.push(gen_rand_al() as char);
    }

    for _ in 0..2 {
        _changeable_string.push(get_rand_spec() as char);
    }

    for _ in 0..2 {
        _changeable_string.push(gen_rand_num() as char);
    }

    println!("Password generated for {} is {}", _website_name, _changeable_string);
    gen_write_file(_website_name, _changeable_string)?;
    Ok(())
}
