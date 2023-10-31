use std::io;

fn get_input_from_keyboard(text_to_print: &str) -> String {
    let mut _input_string = String::new();
    if !text_to_print.is_empty() && text_to_print.is_ascii() {
        io::stdin().read_line(&mut _input_string).expect("FAILED TO READ LINE");
    } else {
        Err("ERROR: TEXT TO PRINT CAN'T BE EMPTY VALUE.")
    }
    return _input_string;
}

fn gen_rand_al() {
    let _al = "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm";
}

fn gen_rand_num() {
    let _num = "0123456789";
}

fn get_rand_spec() {
    let _spec = "-_!=";
}

fn main() {
    let _website_name = get_input_from_keyboard("INSERT THE SERVICE'S NAME.");
    println!("{}", _website_name)
}