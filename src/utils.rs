use proc_macro::{TokenStream, TokenTree};

/// assert in release build
pub (crate) fn hard_assert(condition:bool, code:&str){
    if !condition {
        panic!("easyz3 is grumpy and doesn't understand this: {}",code)
    }
}

/// macro!(a,b) => ["a","b"]
pub (crate) fn extract_variables(input: TokenStream) -> Vec<String> {
    let mut v:Vec<String> = Vec::new();
    for t in input {
        //eprintln!("t: {}", t);
        match t {
            TokenTree::Ident(s) => {
                v.push(s.to_string());            
            }
            TokenTree::Punct(t) => {
                // we expect commas between identifiers  
                hard_assert(t == ',',"expected variable names separated by comma");
            } 
            _ => { 
                panic!("unexpected token. expected variable names separated by comma: {}",&t.to_string()) 
            }
        }
    }
    v
}

/// so we can abstract over z3 types where it makes sense
#[derive(Copy, Clone,Eq,PartialEq,Debug)]
pub(crate) enum Z3Type{
    Int,
    BV(usize)
}
impl Z3Type{
    pub fn is_int(self) -> bool {
        self == Z3Type::Int
    }
    /// rust type it gets converted to and from
    pub fn rust_type_name(self) -> String {
        match self {
            Z3Type::Int => {
                String::from("i64") // this is what we use for our Int type (fast and ususally fine. Z3 is rarely used with bigint)
            }
            Z3Type::BV(width) => {
                hard_assert([8,16,32,64].contains(&width),"unsupported bit width" );
                format!("u{}",width)
            }
        }
    }
    /// Z3 Type we are woreking with
    pub fn z3_type_name(self) -> String {
        match self {
            Z3Type::Int => {
                String::from("z3::ast::Int")
            }
            Z3Type::BV(_) => {
                String::from("z3::ast::BV")
            }
        }
    }
}