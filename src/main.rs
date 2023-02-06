
mod println;
mod get_input;
mod eval_statement;
fn main() {
// keep calculator up
loop{

    // gets input evaluates it then prints to sceen
    println!("enter a mathematical expression");
    let text= get_input::return_input();
    let res= eval_statement::eval_statement(text);
    println::print_to_terminal("resultant is \n"  );
    println::print_to_terminal(&res);
    println::print_to_terminal("\n"  );

}



}
