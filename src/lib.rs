#[cfg(test)]
mod tests {
    use std::io::{stdin, BufRead};

    use super::*;

    #[test]
    fn first_steps() {
        fuck();

        assert!(true);
    }

    //#[test]
    fn it_works() {
        //HayBCMD::init();
        //HayBCMD::set_output_stream(COMMANDPROMPTSTREAM);
        //HayBCMD::add_cvar("tf_fuck", tf_fuck, "1 or 0"); // - type: boolean - 1/0 is a default description for bool cvars(\n the rest of the description)
        
        let mut input;
        loop {
            input = stdin().lock().lines().next().unwrap().unwrap();
            
            if (input == String::from("quit")) {
                break;
            }
            
            //HayBCMD::parse(USERINPUT);
        }

        assert_eq!(input, "quit");
    }
}
