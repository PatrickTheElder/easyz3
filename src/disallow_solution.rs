use crate::utils::Z3Type;

/// adds a constraint to the solver to disllow the current solution in the future.
/// this forces the solver to come up with a new solution, thus allowing you to iterate through solutions
/// (getting slower and slower in the process, as the number of constraints increases, of course)
///
/// z3constraint!(!((a == a_inner) && (b == b_inner) && (c == c_inner))),
pub(crate) fn disallow(varnames: &[String], z3type: Z3Type) -> String {
    let mut s = String::new();
    for varname in varnames.iter() {
        if !s.is_empty() {
            s.push_str(" && ")
        }
        s.push_str(format!("({} == ::{}_inner)", varname, varname).as_str());
    }
    let s = format!("!({})", s);
    if z3type.is_int() {
        format!("z3_constraint!({});", s)
    } else {
        format!("z3_constraint_{}!({});", z3type.rust_type_name(), s)
    }
}
