mod disallow_solution;
mod formula_source_gen;
mod utils;

extern crate proc_macro;
use proc_macro::{TokenStream};
use std::str::FromStr;

use crate::disallow_solution::disallow;
use crate::formula_source_gen::*;
use crate::utils::{extract_variables, hard_assert, Z3Type};
use syn::{ parse_str, Expr};

/// declare symbolic int variables
fn declare_variables_int(input: TokenStream) -> String {
    extract_variables(input)
        .iter()
        .map(|varname| {
            format!(
                "let {} = z3::ast::Int::new_const(&ctx, \"{}\");",
                &varname, &varname
            )
        })
        .collect::<String>()
}

/// declare symbolic BV variables
fn declare_variables_bv(input: TokenStream, bits: usize) -> String {
    extract_variables(input)
        .iter()
        .map(|varname| {
            format!(
                "let {} = z3::ast::BV::new_const(&ctx, \"{}\",{});",
                &varname, &varname, bits
            )
        })
        .collect::<String>()
}

/// formula about z3 ints or BVs
///    let eq1 = z3_formula!(a * 94 + b * 34 == 8400);
/// => let eq1 = Int::_eq(&Int::add(&ctx, &[&Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, 94)]), &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, 34)])]), &Int::from_i64(&ctx, 8400));
fn z3_formula_generic(input: TokenStream, z3type: Z3Type) -> TokenStream {
    let original_str = input.to_string();
    let ast: Expr = parse_str(original_str.as_str()).unwrap(); // turn token soup into AST
    let generated_src = generate_z3_source(z3type, &ast);
    TokenStream::from_str(&generated_src).unwrap()
}

#[proc_macro]
/// formula about z3 ints:
///    let eq1 = z3_formula!(a * 94 + b * 34 == 8400);
/// => let eq1 = Int::_eq(&Int::add(&ctx, &[&Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, 94)]), &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, 34)])]), &Int::from_i64(&ctx, 8400));
pub fn z3_formula(input: TokenStream) -> TokenStream {
    z3_formula_generic(input, Z3Type::Int)
}
#[proc_macro]
pub fn z3_formula_u8(input: TokenStream) -> TokenStream {
    z3_formula_generic(input, Z3Type::BV(8))
}
#[proc_macro]
pub fn z3_formula_u16(input: TokenStream) -> TokenStream {
    z3_formula_generic(input, Z3Type::BV(16))
}
#[proc_macro]
pub fn z3_formula_u32(input: TokenStream) -> TokenStream {
    z3_formula_generic(input, Z3Type::BV(32))
}
#[proc_macro]
pub fn z3_formula_u64(input: TokenStream) -> TokenStream {
    z3_formula_generic(input, Z3Type::BV(64))
}

/// init z3 in the most common way:
/// z3_init!()
/// => let config = Config::new();
///    let ctx = Context::new(&config);
///    let solver = Solver::new(&ctx);
///
///  also can declare variables like so: z3_init!(a,b,c)
fn z3_init_generic(input: TokenStream, z3type: Z3Type) -> TokenStream {
    // This would be a job for declarative macros
    // but they still cannot coexist in the same crate
    // rust is such a shitty language
    let init_src = "use z3;use z3::ast::Ast;use std::ops::Neg;let config = z3::Config::new();let ctx = z3::Context::new(&config);let solver = z3::Solver::new(&ctx);";
    let var_decl = match z3type {
        Z3Type::Int => declare_variables_int(input),
        Z3Type::BV(width) => declare_variables_bv(input, width),
    };
    let src = format!("{}{}", init_src, &var_decl);
    TokenStream::from_str(&src).unwrap()
}

#[proc_macro]
/// init z3 in the most common way:
/// z3_init!()
/// => let config = Config::new();
///    let ctx = Context::new(&config);
///    let solver = Solver::new(&ctx);
///
///  also can declare variables like so: z3_init!(a,b,c)
pub fn z3_init(input: TokenStream) -> TokenStream {
    z3_init_generic(input, Z3Type::Int)
}

#[proc_macro]
pub fn z3_init_u8(input: TokenStream) -> TokenStream {
    z3_init_generic(input, Z3Type::BV(8))
}
#[proc_macro]
pub fn z3_init_u16(input: TokenStream) -> TokenStream {
    z3_init_generic(input, Z3Type::BV(16))
}
#[proc_macro]
pub fn z3_init_u32(input: TokenStream) -> TokenStream {
    z3_init_generic(input, Z3Type::BV(32))
}
#[proc_macro]
pub fn z3_init_u64(input: TokenStream) -> TokenStream {
    z3_init_generic(input, Z3Type::BV(64))
}

/// declare int variable
///    z3var!(a);
/// => let a = ast::Int::new_const(&ctx, "a");
///
///    z3var!(a,b,c)
#[proc_macro]
pub fn z3_var(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&declare_variables_int(input)).unwrap()
}

/// declare bv variables
#[proc_macro]
pub fn z3_var_u8(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&declare_variables_bv(input, 8)).unwrap()
}
#[proc_macro]
pub fn z3_var_u16(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&declare_variables_bv(input, 16)).unwrap()
}
#[proc_macro]
pub fn z3_var_u32(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&declare_variables_bv(input, 32)).unwrap()
}
#[proc_macro]
pub fn z3_var_u64(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&declare_variables_bv(input, 64)).unwrap()
}

/// add constraint on int formula:
///     z3constraint!(a * 94 + b * 34 == 8400);
/// =>  solver.assert(&Int::_eq(&Int::add(&ctx, &[&Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, 94)]), &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, 34)])]), &Int::from_i64(&ctx, 8400)));
#[proc_macro]
pub fn z3_constraint(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&format!(
        "solver.assert(z3_formula!({}));",
        &input.to_string().trim().to_string()
    ))
    .unwrap()
}
#[proc_macro]
pub fn z3_constraint_u8(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&format!(
        "solver.assert(z3_formula_u8!({}));",
        &input.to_string().trim().to_string()
    ))
    .unwrap()
}
#[proc_macro]
pub fn z3_constraint_u16(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&format!(
        "solver.assert(z3_formula_u16!({}));",
        &input.to_string().trim().to_string()
    ))
    .unwrap()
}
#[proc_macro]
pub fn z3_constraint_u32(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&format!(
        "solver.assert(z3_formula_u32!({}));",
        &input.to_string().trim().to_string()
    ))
    .unwrap()
}
#[proc_macro]
pub fn z3_constraint_u64(input: TokenStream) -> TokenStream {
    TokenStream::from_str(&format!(
        "solver.assert(z3_formula_u64!({}));",
        &input.to_string().trim().to_string()
    ))
    .unwrap()
}

/// add constraint on int formula:
///     z3constraint!(a * 94 + b * 34 == 8400);
/// =>  solver.assert(&Int::_eq(&Int::add(&ctx, &[&Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, 94)]), &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, 34)])]), &Int::from_i64(&ctx, 8400)));
#[proc_macro]
pub fn z3_distinct(input: TokenStream) -> TokenStream {
    let variables = extract_variables(input);
    // convert to "&a,&b,&c"
    let vars_as_str = variables
        .iter()
        .map(|varname| format!("&{}", varname))
        .collect::<Vec<String>>()
        .join(",");
    let s = TokenStream::from_str(&format!(
        "solver.assert(&z3::ast::Int::distinct(&ctx, &[{}]));",
        &vars_as_str
    ))
    .unwrap();
    //eprintln!("z3constraint({}) => {}",inner,s);
    s
}

/// usage:
/// if let Some((a,b)) = z3solve!(a,b) {
///     println("Solution: a:{}  b:{}",a,b);
/// } else{
///     println!("unsat :-(");
/// }
///
/// this is the Int version
fn z3_solve_generic(input: TokenStream, z3type: Z3Type) -> TokenStream {
    //println!("z3_solve_generic({:?})", z3type);
    let varnames = extract_variables(input);
    hard_assert(
        !varnames.is_empty(),
        "usage: if let Some((a,b)) = z3solve!(a,b)",
    );
    // we build from inner to outer
    let src = disallow(&varnames, z3type);
    let mut src = format!(
        "{};Some(({}))",
        src,
        varnames
            .iter()
            // if BV, we cast tu corresponding rust type
            .map(|v| {
                let cast = if z3type.is_int() {
                    String::new()
                } else {
                    format!("as {}", &z3type.rust_type_name())
                };
                format!("{}_inner {}", v, &cast)
            })
            .collect::<Vec<String>>()
            .join(",")
    ); // what we return
       // build extraction of variables
    for var in varnames {
        src = "if let Some($_inner) = model.eval(&$, true) { if let Some($_inner) = $_inner.as_~() {#} else {None}} else {None}"
            .replace("~",if z3type.is_int() {"i64"} else {"u64"})
            .replace("$",&var)
            .replace("#",&src);
        //eprintln!("src: {}", src);
    }
    // outer layer
    src = "if solver.check() == z3::SatResult::Sat {if let Some(model) = solver.get_model() {#} else {None}} else {None}".replace("#",&src);
    //eprintln!("src: {}", src);
    TokenStream::from_str(&src).unwrap()
}

#[proc_macro]
/// usage:
/// if let Some((a,b)) = z3solve!(a,b) {
///     println("Solution: a:{}  b:{}",a,b);
/// } else{
///     println!("unsat :-(");
/// }
pub fn z3_solve(input: TokenStream) -> TokenStream {
    z3_solve_generic(input, Z3Type::Int)
}

#[proc_macro]
pub fn z3_solve_u8(input: TokenStream) -> TokenStream {
    z3_solve_generic(input, Z3Type::BV(8))
}
#[proc_macro]
pub fn z3_solve_u16(input: TokenStream) -> TokenStream {
    z3_solve_generic(input, Z3Type::BV(16))
}
#[proc_macro]
pub fn z3_solve_u32(input: TokenStream) -> TokenStream {
    z3_solve_generic(input, Z3Type::BV(32))
}
#[proc_macro]
pub fn z3_solve_u64(input: TokenStream) -> TokenStream {
    z3_solve_generic(input, Z3Type::BV(64))
}
