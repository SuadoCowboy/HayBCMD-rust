use std::io;

fn main() {
    let (commands_handler, commands_funcs) = haybcmd::init();

    //haybcmd::add_cvar("tf_fuck", tf_fuck, "1 or 0"); // - type: boolean - 1/0 is a default description for bool cvars(\n the rest of the description)

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        haybcmd::parse(commands_handler.clone(), commands_funcs, input.trim().to_string());
    }

}