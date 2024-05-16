pub mod output;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod command;

pub fn init() -> (command::CommandsHandler, command::CommandsFuncs) {
    let mut commands_handler = command::CommandsHandler::new();
    let mut commands_funcs = command::CommandsFuncs::new();

    command::init_base_commands(&mut commands_handler, &mut commands_funcs);

    (commands_handler, commands_funcs)
}

pub fn parse(commands_handler: command::CommandsHandler, commands_funcs: command::CommandsFuncs, str: String) {
    let lexer = lexer::Lexer::new(str);
    let mut parser_var = parser::Parser::new(Box::new(lexer), Box::new(commands_handler), commands_funcs);
    parser_var.parse();
}