#[cfg(test)]
mod tests {
    use std::io::BufRead;

    #[test]
    fn interpreter() {
        haybcmd::init(std::io::stdout());
        
        //haybcmd::add_cvar("tf_fuck", tf_fuck, "1 or 0"); // - type: boolean - 1/0 is a default description for bool cvars(\n the rest of the description)
        
        let input = String::from(r#"
            echo "Hello, World!";
            alias $var echo "This is a variable: $var";
            $var "Hello, Variable!";
            variables;
        "#);

        haybcmd::parse(input);

        assert!(true);
    }
}