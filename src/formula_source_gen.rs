use quote::ToTokens;
use syn::{BinOp, Expr, UnOp};
use crate::utils::Z3Type;

/// work on AST recursively
/// 
/// for both Int and BV
/// 
/// transform to rust source that uses z3 api
pub (crate) fn generate_z3_source(z3type:Z3Type,e: &Expr) ->String{
    //println!("generate_z3_source({:?},{})",z3type,e.to_token_stream().to_string().as_str());
    let binary = |op_str: &str, lhs: &Expr, rhs: &Expr| -> String {
        format!("&{}::{}(&ctx, {}, {})", z3type.z3_type_name(),op_str, &generate_z3_source(z3type, lhs), &generate_z3_source(z3type, rhs))
    };
    let binary_lst = |op_str: &str, lhs: &Expr, rhs: &Expr| -> String {
            format!("&{}::{}(&ctx, &[{}, {}])", z3type.z3_type_name(), op_str, &generate_z3_source(z3type, lhs), &generate_z3_source(z3type, rhs))
    };
    let binary_bvonly = |op_str: &str, lhs: &Expr, rhs: &Expr| -> String {
        if z3type.is_int(){
            panic!("sadly, z3 does not implement bitwise opertions (XOR, SHL, SHR,  '|' , '|' ) on regular ints. use bitvecs for that")            
        } else{
            format!("&{}::{}(&{}, &{})", z3type.z3_type_name(), op_str, &generate_z3_source(z3type, lhs), &generate_z3_source(z3type, rhs))
        }
    };
    let binary_noctx = |op_str: &str, lhs: &Expr, rhs: &Expr| -> String {
        format!("&{}::{}({}, {})", z3type.z3_type_name(), op_str, &generate_z3_source(z3type, lhs), &generate_z3_source(z3type, rhs))
    };

    match e {
        // 42
        Expr::Lit(l) => {
            match z3type{
                Z3Type::Int => {
                    format!("&z3::ast::Int::from_i64(&ctx, {})",l.to_token_stream())
                }
                Z3Type::BV(width) => {
                    format!("&z3::ast::BV::from_u64(&ctx, {} as u64,{})",l.to_token_stream(),width)
                }
            }
        }
        // ()
        Expr::Paren(e) => {
            format!("({})",&generate_z3_source(z3type,&e.expr))
        }
        Expr::Unary(e) => {
             match e.op {
                 UnOp::Neg(_) => {
                     format!("&({}).neg()", &generate_z3_source(z3type,&e.expr))
                 }
                 UnOp::Not(_) => {
                     format!("&({}).not()", &generate_z3_source(z3type,&e.expr))
                 }
                 _ => {panic!("Sorry, not implemented");}
             }
        }
        // "+"
        Expr::Binary(e) => {
            match e.op{
                BinOp::Add(_) => {
                    match z3type{
                        Z3Type::Int => {
                            binary_lst("add",&e.left,&e.right)
                        }
                        Z3Type::BV(_width) => {
                            binary_noctx("bvadd",&e.left,&e.right)
                        }
                    }
                }
                BinOp::Sub(_) => {
                    binary("sub",&e.left,&e.right)
                }
                BinOp::Mul(_) => {
                    binary_lst("mul",&e.left,&e.right)
                }
                BinOp::Div(_) => {
                    binary("div",&e.left,&e.right)
                }
                BinOp::And(_) => {
                    format!("&z3::ast::Bool::and(&ctx, &[{}, {}])", &generate_z3_source(z3type,&e.left), &generate_z3_source(z3type,&e.right))
                }
                 BinOp::Or(_) => {
                     format!("&z3::ast::Bool::or(&ctx, &[{}, {}])", &generate_z3_source(z3type,&e.left), &generate_z3_source(z3type,&e.right))
                 }
                BinOp::Eq(_) => {
                    binary_noctx("_eq",&e.left,&e.right)
                }
                BinOp::Lt(_) => {
                    binary_noctx("lt",&e.left,&e.right)
                }
                BinOp::Le(_) => {
                    binary_noctx("le",&e.left,&e.right)
                }
                BinOp::Ne(_) => {
                    format!("&z3::ast::Bool::not(&{})",binary_noctx("_eq",&e.left,&e.right))
                }
                BinOp::Ge(_) => {
                    binary_noctx("ge",&e.left,&e.right)
                }
                BinOp::Gt(_) => {
                    binary_noctx("gt",&e.left,&e.right)
                }
                BinOp::BitXor(_) => {
                    binary_bvonly("bvxor", &e.left, &e.right)
                }
                BinOp::BitAnd(_) => {
                    binary_bvonly("bvand", &e.left, &e.right)
                }
                BinOp::BitOr(_) => {
                    binary_bvonly("bvor", &e.left, &e.right)
                }
                BinOp::Shl(_) => {
                    binary_bvonly("bvshl", &e.left, &e.right)
                }
                BinOp::Shr(_) => {
                    binary_bvonly("bvshr", &e.left, &e.right)
                }
                _ => panic!("did not expect {:?}", e.to_token_stream())
            }

        }
        // "Path" is the rust name for identifiers
        Expr::Path(e) => {
            let name = e.to_token_stream().to_string();
            //println!("**************************** {}",name);
            if name.starts_with(":: "){
                match z3type{
                    Z3Type::Int => {
                        format!("&z3::ast::Int::from_i64(&ctx, {})",&name[3..])    
                    }
                    Z3Type::BV(width) => {
                        format!("&z3::ast::BV::from_u64(&ctx, {} as u64,{})",&name[3..],width)                        
                    }
                }
            } else {
                format!("&{}",name)
            }
        }
        _ => {
            panic!("easyz3 did not expect {:?}", e.to_token_stream())
        }
    }
}
