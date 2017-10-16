extern crate python_ast as ast;
extern crate cpython;

use ast::*;

use cpython::*;

use std::borrow::Borrow;
use std::convert;

#[derive(Debug)]
pub struct ParseError(String);

type ParseResult = Result<Program, ParseError>;

// Allow automatic conversion from PyErr result types to ParseError result types
impl convert::From<PyErr> for ParseError {
    fn from(pe: PyErr) -> Self {
        ParseError(format!("{:?}", pe))
    }
}

// Allow automatic conversion from String result types to ParseError result types
impl convert::From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError(s)
    }
}

/// Parse a program (string) into an ast result
pub fn parse_program_fallback(input: &str) -> ParseResult {
    let gil = Python::acquire_gil();
    let python = gil.python();

    let cxmodule = python.import("compiler")?;

    let prog_pyobj = PyString::new(python, input).into_object();

    let args: PyTuple = PyTuple::new(python, &[prog_pyobj]);

    let result = cxmodule.call(python, "parse", args, None)?;

    Ok(Program { module: destructure_module(result, python)? })
}

/// Convert a PyObject representing a Python AST module into an ast Module native to this
/// Rust program
fn destructure_module(module: PyObject, py: Python) -> Result<Module, ParseError> {
    let mut v = vec![];

    // Validate that received python object has 'Module' type
    let py_type = module.getattr(py, "__class__")?.getattr(py, "__name__")?;
    let py_type_string: &PyString = match py_type.cast_as(py) {
        Ok(ps) => Ok(ps),
        _ => Err(String::from(
            "Failed to convert class type attribute to Python String",
        )),
    }?;
    if py_type_string.to_string(py)? != "Module" {
        return Err(ParseError(String::from(
            "Module does not refer to Python Module Object",
        )));
    }

    // Destructure the module.node.nodes attribute into a PyList, which we will process
    // into the vector v above
    let attr = module.getattr(py, "node")?.getattr(py, "nodes")?;
    let stmts: &PyList = match attr.cast_as(py) {
        Ok(pl) => Ok(pl),
        _ => Err(String::from(
            "Failed to convert module.node.nodes to list type",
        )),
    }?;

    for stmt in stmts.iter(py) {
        v.push(destructure_stmt(stmt, py)?);
    }

    Ok(Module { statements: v })
}

/// Convert a PyObject representing a Python AST Statement into an ast Statement native
/// to this Rust program
fn destructure_stmt(stmt: PyObject, py: Python) -> Result<Statement, ParseError> {
    // Get type name for module members
    let py_type = stmt.getattr(py, "__class__")?.getattr(py, "__name__")?;
    let py_type_string: &PyString = match py_type.cast_as(py) {
        Ok(ps) => Ok(ps),
        _ => Err(format!(
            "{:?}: Couldn't convert class type attribute to PyString",
            stmt
        )),
    }?;

    // Construct appropriate Statement type for matched module statement type
    let rstmt = match py_type_string.to_string(py)?.borrow() {
        "Assign" => {
            // We use a helper here because assignments in Python are a living nightmare
            destructure_assignment(stmt, py)?
        }
        "Discard" => Statement::Expression(destructure_expr(stmt.getattr(py, "expr")?, py)?),
        "Printnl" => {
            // Very much assuming that the print only refers to a single value TODO
            Statement::Print(destructure_expr(stmt.getattr(py, "expr")?, py)?)
        }
        s => {
            return Err(ParseError(
                format!("Unhandled type {} in converting stmt", s),
            ));
        }
    };

    Ok(rstmt)
}


/// Slay me
fn destructure_assignment(assig: PyObject, py: Python) -> Result<Statement, ParseError> {
    // TODO: I'm assuming we never perform a multiple assignment, or assignment
    // to a tuple, or any form of destructuring
    let nodes = assig.getattr(py, "nodes")?;
    let lvalues: &PyList = match nodes.cast_as(py) {
        Ok(pl) => pl,
        _ => {
            return Err(ParseError(format!(
                "{:?}: Could not convert assignment nodes to list",
                assig
            )));
        }
    };

    if lvalues.len(py) <= 0 {
        return Err(ParseError(
            String::from("Assignment does not have at least one node"),
        ));
    }

    let target = lvalues.get_item(py, 0);
    let target_name_pyo = target.getattr(py, "name")?;
    let target_name_pys: &PyString = match target_name_pyo.cast_as(py) {
        Ok(ps) => ps,
        _ => {
            return Err(ParseError(format!(
                "{:?}: Could not convert assigned name to list",
                target_name_pyo
            )));
        }
    };

    let target_name_cowstring = target_name_pys.to_string(py)?;
    let target_name_string: &str = target_name_cowstring.borrow();

    let expr = assig.getattr(py, "expr")?;
    Ok(Statement::Assign(
        Target::Name(String::from(target_name_string)),
        destructure_expr(expr, py)?,
    ))
}

// This function became heinously complicated
fn destructure_expr(expr: PyObject, py: Python) -> Result<Expression, ParseError> {
    // Get type name for expr
    let py_type = expr.getattr(py, "__class__")?.getattr(py, "__name__")?;
    let py_type_string: &PyString = match py_type.cast_as(py) {
        Ok(ps) => Ok(ps),
        _ => Err(format!(
            "{:?}: Couldn't convert class type attribute to PyString",
            expr
        )),
    }?;

    let rexpr = match py_type_string.to_string(py)?.borrow() {
        "CallFunc" => {
            let func = expr.getattr(py, "node")?.getattr(py, "name")?;
            let func_pstr: &PyString = match func.cast_as(py) {
                Ok(ps) => ps,
                _ => {
                    return Err(ParseError(
                        format!("{:?}: could not convert name to PyString", func),
                    ));
                }
            };

            let func_cowstr = func_pstr.to_string(py)?;
            let func_str: &str = func_cowstr.borrow();
            if func_str != "input" {
                return Err(ParseError(
                    format!("{:?}: unhandled function name", func_str),
                ));
            }

            let args = expr.getattr(py, "args")?;
            let arg_plist: &PyList = match args.cast_as(py) {
                Ok(pl) => pl,
                _ => {
                    return Err(ParseError(
                        format!("{:?}: could not convert args to PyList", args),
                    ));
                }
            };

            if arg_plist.len(py) != 0 {
                return Err(ParseError(String::from(
                    "Functions with multiple arguments are not supported",
                )));
            }

            Expression::Input
        }
        "Add" => {
            Expression::Add(
                Box::new(destructure_expr(expr.getattr(py, "left")?, py)?),
                Box::new(destructure_expr(expr.getattr(py, "right")?, py)?),
            )
        }
        "Const" => {
            let val = expr.getattr(py, "value")?;
            let ctype = val.get_type(py);
            let ctname = ctype.name(py);
            let tname: &str = ctname.borrow();

            if tname != "int" {
                return Err(ParseError(
                    String::from("Only integer constants are supported"),
                ));
            }

            let n: i32 = val.extract::<i32>(py)?;

            Expression::DecimalI32(n)
        }
        "Name" => {
            let name = expr.getattr(py, "name")?;
            let name_pstr: &PyString = match name.cast_as(py) {
                Ok(ps) => ps,
                _ => {
                    return Err(ParseError(
                        format!("{:?}: failed to convert name to PyString", name),
                    ));
                }
            };
            let name_cowstr = name_pstr.to_string(py)?;
            let name_str: &str = name_cowstr.borrow();

            Expression::Target(Target::Name(String::from(name_str)))
        }
        e => {
            return Err(ParseError(format!("{:?}: Unhandled expression type", e)));
        }
    };

    Ok(rexpr)
}
