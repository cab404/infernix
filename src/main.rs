use std::{collections::HashMap, ops::Add};

use rnix::{
    self,
    types::{ParsedType, TypedNode, Wrapper},
    value::Value as ParsedValue,
};

pub enum Type {
    /// Really is a number
    Integer,
    /// Really is a number
    Float,
    /// Really is a number
    Path,

    /// A string
    String,
    /// Either with many arms
    SumType(Vec<Type>),
    /// Map with many types
    Attrset(fn() -> Vec<String>, fn(String) -> Type),
    /// Thing we can’t do without more computations
    Lazy(fn() -> Type),
    /// Scheise!
    InferenceError(String),

    Lambda(Vec<Type>, fn() -> Type),
}

pub struct Ident {}

pub struct Context {
    pub ctx: HashMap<Ident, Type>,
}

impl Context {
    // TODO: Store type context
}

impl Add for Context {
    type Output = Context;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

fn infer(ctx: Context, node: &rnix::SyntaxNode) -> Type {
    let trytype: Result<ParsedType, _> = node.clone().try_into();
    if let Ok(oktype) = trytype {
        match oktype {
            // Obvious stuff
            ParsedType::Assert(f) => infer(ctx, f.node()),
            ParsedType::Str(_) => Type::String,

            ParsedType::Value(v) => {
                match v.to_value() {
                    Ok(val) => {
                        match val {
                            ParsedValue::Float(_) => Type::Float,
                            ParsedValue::Integer(_) => Type::Integer,
                            ParsedValue::String(_) => Type::String,
                            ParsedValue::Path(_, _) => Type::Path,
                        }
                    }
                    Err(e) => Type::InferenceError(format!("that shouldn‘t have happened. tried getting value type for {:?}, got error {}", v, e).to_string()),
                }
            }

            // these are trivial containers
            ParsedType::IfElse(_) => todo!(),
            ParsedType::List(_) => todo!(),
            // TODO: remove unwrap
            ParsedType::Paren(f) => infer(ctx, &f.inner().unwrap()),
            ParsedType::With(_) => todo!(),
            ParsedType::Ident(_) => todo!(),
            ParsedType::LetIn(_) => todo!(),
            ParsedType::LegacyLet(_) => todo!(),
            ParsedType::Root(_) => todo!(),
            ParsedType::Error(e) => Type::InferenceError(format!("Parsing error: {}", e.node())),

            // operations on trivial types
            ParsedType::BinOp(_) => todo!(),
            ParsedType::OrDefault(_) => todo!(),
            ParsedType::UnaryOp(_) => todo!(),

            // Non-trivial hell
            ParsedType::Lambda(_) => todo!(),
            ParsedType::AttrSet(_) => todo!(),

            // IDK
            ParsedType::Apply(_) => todo!(),
            ParsedType::Select(_) => todo!(),
            ParsedType::Dynamic(_) => todo!(),


            // Those can’t be inferred at the top level
            ParsedType::Pattern(_) => todo!(),
            ParsedType::PathWithInterpol(_) => todo!(),
            ParsedType::Inherit(_) => todo!(),
            ParsedType::InheritFrom(_) => todo!(),
            ParsedType::PatBind(_) => todo!(),
            ParsedType::PatEntry(_) => todo!(),
            ParsedType::KeyValue(_) => todo!(),
            ParsedType::Key(_) => todo!(),
        }
    } else {
        Type::InferenceError("mowwow nothing like that exists".to_string())
    }
}

fn printnode(node: rnix::SyntaxNode, offset: usize) {
    let trytype: Result<ParsedType, _> = node.clone().try_into();

    println!(
        "{}{:?} ({}): {:?}",
        " -".repeat(offset),
        node,
        node,
        trytype
    );

    for ch in node.children() {
        printnode(ch, offset + 1);
    }
}

fn main() {
    printnode(
        rnix::parse(r#"[ (12 + 12) "mowwow" { a = 12; } ./test [ 1 2 3 4 ] (with {}; {}) ]"#)
            .node(),
        0,
    );

    printnode(rnix::parse("let a = 12; in ({ a }: a)").node(), 0)
}
