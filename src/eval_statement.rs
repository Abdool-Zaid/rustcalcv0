use eval::eval;

pub fn eval_statement(input:String)->std::string::String{
   let res = eval(&input).unwrap();
   res.to_string()
}