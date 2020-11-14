#![allow(
    path_statements,
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables,
    clippy::unknown_clippy_lints,
    clippy::missing_safety_doc,
    clippy::match_single_binding,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::needless_lifetimes,
    clippy::borrowed_box,
    clippy::map_clone,
    clippy::toplevel_ref_arg,
    clippy::double_parens,
    clippy::collapsible_if,
    clippy::clone_on_copy,
    clippy::unused_unit,
    clippy::deref_addrof,
    clippy::clone_on_copy,
    clippy::needless_return,
    clippy::op_ref,
    clippy::match_like_matches_macro,
    clippy::comparison_chain,
    clippy::len_zero,
    clippy::extra_unused_lifetimes
)]

//use ::serde::de::DeserializeOwned;
use ::differential_datalog::record::FromRecord;
use ::differential_datalog::record::IntoRecord;
use ::differential_datalog::record::Mutator;
use ::serde::Deserialize;
use ::serde::Serialize;

// `usize` and `isize` are builtin Rust types; we therefore declare an alias to DDlog's `usize` and
// `isize`.
pub type std_usize = u64;
pub type std_isize = i64;

mod ddlog_log;
pub use ddlog_log::*;

pub mod closure;

/* FlatBuffers code generated by `flatc` */
#[cfg(feature = "flatbuf")]
mod flatbuf_generated;

/* `FromFlatBuffer`, `ToFlatBuffer`, etc, trait declarations. */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

pub trait Val:
    Default
    + Eq
    + Ord
    + Clone
    + ::std::hash::Hash
    + PartialEq
    + PartialOrd
    + Serialize
    + ::serde::de::DeserializeOwned
    + 'static
{
}

impl<T> Val for T where
    T: Default
        + Eq
        + Ord
        + Clone
        + ::std::hash::Hash
        + PartialEq
        + PartialOrd
        + Serialize
        + ::serde::de::DeserializeOwned
        + 'static
{
}

pub fn string_append_str(mut s1: String, s2: &str) -> String {
    s1.push_str(s2);
    s1
}

#[allow(clippy::ptr_arg)]
pub fn string_append(mut s1: String, s2: &String) -> String {
    s1.push_str(s2.as_str());
    s1
}

#[macro_export]
macro_rules! deserialize_map_from_array {
    ( $modname:ident, $ktype:ty, $vtype:ty, $kfunc:path ) => {
        mod $modname {
            use super::*;
            use serde::de::{Deserialize, Deserializer};
            use serde::ser::Serializer;
            use std::collections::BTreeMap;

            pub fn serialize<S>(
                map: &crate::ddlog_std::Map<$ktype, $vtype>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_seq(map.x.values())
            }

            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> Result<crate::ddlog_std::Map<$ktype, $vtype>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let v = Vec::<$vtype>::deserialize(deserializer)?;
                Ok(v.into_iter().map(|item| ($kfunc(&item), item)).collect())
            }
        }
    };
}


pub static __STATIC_1: ::once_cell::sync::Lazy<crate::ddlog_std::Vec<crate::ast::Spanned<crate::ast::Name>>> = ::once_cell::sync::Lazy::new(|| crate::ddlog_std::vec_empty());
pub static __STATIC_0: ::once_cell::sync::Lazy<crate::ddlog_std::Vec<crate::ast::Spanned<crate::ast::Name>>> = ::once_cell::sync::Lazy::new(|| crate::ddlog_std::vec_with_capacity((&(1 as u64))));
pub mod ddlog_std;
pub mod internment;
pub mod debug;
pub mod log;
pub mod vec;
pub mod ast;
pub mod utils;
pub mod group;
pub mod inputs;
use ast::{Pattern, Span, Spanned};
use internment::Intern;
use once_cell::sync::Lazy;

/// The implicitly introduced `arguments` variable for function scopes,
/// kept in a global so we only allocate & intern it once
pub static IMPLICIT_ARGUMENTS: Lazy<Intern<Pattern>> = Lazy::new(|| {
    Intern::new(Pattern::SinglePattern {
        name: Some(Spanned {
            data: Intern::new("arguments".to_owned()),
            // TODO: Give this the span of the creating function I guess
            span: Span::new(0, 0),
        })
        .into(),
    })
});

#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct ChainedWith {
    pub object: crate::ast::ExprId,
    pub property: crate::ast::ExprId,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for ChainedWith{}
::differential_datalog::decl_struct_from_record!(ChainedWith["ChainedWith"]<>, ["ChainedWith"][3]{[0]object["object"]: crate::ast::ExprId, [1]property["property"]: crate::ast::ExprId, [2]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(ChainedWith, ["ChainedWith"]<>, object, property, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(ChainedWith, <>, object: crate::ast::ExprId, property: crate::ast::ExprId, file: crate::ast::FileId);
impl ::std::fmt::Display for ChainedWith {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::ChainedWith{object,property,file} => {
                __formatter.write_str("ChainedWith{")?;
                ::std::fmt::Debug::fmt(object, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(property, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for ChainedWith {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct ChildScope {
    pub parent: crate::ast::ScopeId,
    pub child: crate::ast::ScopeId,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for ChildScope{}
::differential_datalog::decl_struct_from_record!(ChildScope["ChildScope"]<>, ["ChildScope"][3]{[0]parent["parent"]: crate::ast::ScopeId, [1]child["child"]: crate::ast::ScopeId, [2]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(ChildScope, ["ChildScope"]<>, parent, child, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(ChildScope, <>, parent: crate::ast::ScopeId, child: crate::ast::ScopeId, file: crate::ast::FileId);
impl ::std::fmt::Display for ChildScope {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::ChildScope{parent,child,file} => {
                __formatter.write_str("ChildScope{")?;
                ::std::fmt::Debug::fmt(parent, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(child, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for ChildScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct FunctionLevelScope {
    pub scope: crate::ast::ScopeId,
    pub nearest: crate::ast::ScopeId,
    pub file: crate::ast::FileId,
    pub id: crate::ast::AnyId
}
impl abomonation::Abomonation for FunctionLevelScope{}
::differential_datalog::decl_struct_from_record!(FunctionLevelScope["FunctionLevelScope"]<>, ["FunctionLevelScope"][4]{[0]scope["scope"]: crate::ast::ScopeId, [1]nearest["nearest"]: crate::ast::ScopeId, [2]file["file"]: crate::ast::FileId, [3]id["id"]: crate::ast::AnyId});
::differential_datalog::decl_struct_into_record!(FunctionLevelScope, ["FunctionLevelScope"]<>, scope, nearest, file, id);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(FunctionLevelScope, <>, scope: crate::ast::ScopeId, nearest: crate::ast::ScopeId, file: crate::ast::FileId, id: crate::ast::AnyId);
impl ::std::fmt::Display for FunctionLevelScope {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::FunctionLevelScope{scope,nearest,file,id} => {
                __formatter.write_str("FunctionLevelScope{")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(nearest, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(id, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for FunctionLevelScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct IsExported {
    pub file: crate::ast::FileId,
    pub id: crate::ast::AnyId
}
impl abomonation::Abomonation for IsExported{}
::differential_datalog::decl_struct_from_record!(IsExported["IsExported"]<>, ["IsExported"][2]{[0]file["file"]: crate::ast::FileId, [1]id["id"]: crate::ast::AnyId});
::differential_datalog::decl_struct_into_record!(IsExported, ["IsExported"]<>, file, id);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(IsExported, <>, file: crate::ast::FileId, id: crate::ast::AnyId);
impl ::std::fmt::Display for IsExported {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::IsExported{file,id} => {
                __formatter.write_str("IsExported{")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(id, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for IsExported {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct NameInScope {
    pub file: crate::ast::FileId,
    pub name: crate::ast::Name,
    pub scope: crate::ast::ScopeId,
    pub span: crate::ddlog_std::Option<crate::ast::Span>,
    pub declared_in: crate::ast::AnyId,
    pub implicit: bool
}
impl abomonation::Abomonation for NameInScope{}
::differential_datalog::decl_struct_from_record!(NameInScope["NameInScope"]<>, ["NameInScope"][6]{[0]file["file"]: crate::ast::FileId, [1]name["name"]: crate::ast::Name, [2]scope["scope"]: crate::ast::ScopeId, [3]span["span"]: crate::ddlog_std::Option<crate::ast::Span>, [4]declared_in["declared_in"]: crate::ast::AnyId, [5]implicit["implicit"]: bool});
::differential_datalog::decl_struct_into_record!(NameInScope, ["NameInScope"]<>, file, name, scope, span, declared_in, implicit);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(NameInScope, <>, file: crate::ast::FileId, name: crate::ast::Name, scope: crate::ast::ScopeId, span: crate::ddlog_std::Option<crate::ast::Span>, declared_in: crate::ast::AnyId, implicit: bool);
impl ::std::fmt::Display for NameInScope {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::NameInScope{file,name,scope,span,declared_in,implicit} => {
                __formatter.write_str("NameInScope{")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(span, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared_in, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(implicit, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for NameInScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct NoUndef {
    pub name: crate::ast::Name,
    pub scope: crate::ast::ScopeId,
    pub span: crate::ast::Span,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for NoUndef{}
::differential_datalog::decl_struct_from_record!(NoUndef["NoUndef"]<>, ["NoUndef"][4]{[0]name["name"]: crate::ast::Name, [1]scope["scope"]: crate::ast::ScopeId, [2]span["span"]: crate::ast::Span, [3]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(NoUndef, ["NoUndef"]<>, name, scope, span, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(NoUndef, <>, name: crate::ast::Name, scope: crate::ast::ScopeId, span: crate::ast::Span, file: crate::ast::FileId);
impl ::std::fmt::Display for NoUndef {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::NoUndef{name,scope,span,file} => {
                __formatter.write_str("NoUndef{")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(span, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for NoUndef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct TypeofUndef {
    pub whole_expr: crate::ast::ExprId,
    pub undefined_expr: crate::ast::ExprId,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for TypeofUndef{}
::differential_datalog::decl_struct_from_record!(TypeofUndef["TypeofUndef"]<>, ["TypeofUndef"][3]{[0]whole_expr["whole_expr"]: crate::ast::ExprId, [1]undefined_expr["undefined_expr"]: crate::ast::ExprId, [2]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(TypeofUndef, ["TypeofUndef"]<>, whole_expr, undefined_expr, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(TypeofUndef, <>, whole_expr: crate::ast::ExprId, undefined_expr: crate::ast::ExprId, file: crate::ast::FileId);
impl ::std::fmt::Display for TypeofUndef {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::TypeofUndef{whole_expr,undefined_expr,file} => {
                __formatter.write_str("TypeofUndef{")?;
                ::std::fmt::Debug::fmt(whole_expr, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(undefined_expr, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for TypeofUndef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct UnusedVariables {
    pub name: crate::ast::Name,
    pub declared: crate::ast::AnyId,
    pub span: crate::ast::Span,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for UnusedVariables{}
::differential_datalog::decl_struct_from_record!(UnusedVariables["UnusedVariables"]<>, ["UnusedVariables"][4]{[0]name["name"]: crate::ast::Name, [1]declared["declared"]: crate::ast::AnyId, [2]span["span"]: crate::ast::Span, [3]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(UnusedVariables, ["UnusedVariables"]<>, name, declared, span, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(UnusedVariables, <>, name: crate::ast::Name, declared: crate::ast::AnyId, span: crate::ast::Span, file: crate::ast::FileId);
impl ::std::fmt::Display for UnusedVariables {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::UnusedVariables{name,declared,span,file} => {
                __formatter.write_str("UnusedVariables{")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(span, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for UnusedVariables {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct UseBeforeDef {
    pub name: crate::ast::Name,
    pub used: crate::ast::ExprId,
    pub used_in: crate::ast::Span,
    pub declared: crate::ast::AnyId,
    pub declared_in: crate::ast::Span,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for UseBeforeDef{}
::differential_datalog::decl_struct_from_record!(UseBeforeDef["UseBeforeDef"]<>, ["UseBeforeDef"][6]{[0]name["name"]: crate::ast::Name, [1]used["used"]: crate::ast::ExprId, [2]used_in["used_in"]: crate::ast::Span, [3]declared["declared"]: crate::ast::AnyId, [4]declared_in["declared_in"]: crate::ast::Span, [5]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(UseBeforeDef, ["UseBeforeDef"]<>, name, used, used_in, declared, declared_in, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(UseBeforeDef, <>, name: crate::ast::Name, used: crate::ast::ExprId, used_in: crate::ast::Span, declared: crate::ast::AnyId, declared_in: crate::ast::Span, file: crate::ast::FileId);
impl ::std::fmt::Display for UseBeforeDef {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::UseBeforeDef{name,used,used_in,declared,declared_in,file} => {
                __formatter.write_str("UseBeforeDef{")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(used, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(used_in, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared_in, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for UseBeforeDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct VariableUsages {
    pub file: crate::ast::FileId,
    pub name: crate::ast::Name,
    pub scope: crate::ast::ScopeId,
    pub declared_in: crate::ast::AnyId
}
impl abomonation::Abomonation for VariableUsages{}
::differential_datalog::decl_struct_from_record!(VariableUsages["VariableUsages"]<>, ["VariableUsages"][4]{[0]file["file"]: crate::ast::FileId, [1]name["name"]: crate::ast::Name, [2]scope["scope"]: crate::ast::ScopeId, [3]declared_in["declared_in"]: crate::ast::AnyId});
::differential_datalog::decl_struct_into_record!(VariableUsages, ["VariableUsages"]<>, file, name, scope, declared_in);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(VariableUsages, <>, file: crate::ast::FileId, name: crate::ast::Name, scope: crate::ast::ScopeId, declared_in: crate::ast::AnyId);
impl ::std::fmt::Display for VariableUsages {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::VariableUsages{file,name,scope,declared_in} => {
                __formatter.write_str("VariableUsages{")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared_in, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for VariableUsages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct WithinTypeofExpr {
    pub type_of: crate::ast::ExprId,
    pub expr: crate::ast::ExprId,
    pub file: crate::ast::FileId
}
impl abomonation::Abomonation for WithinTypeofExpr{}
::differential_datalog::decl_struct_from_record!(WithinTypeofExpr["WithinTypeofExpr"]<>, ["WithinTypeofExpr"][3]{[0]type_of["type_of"]: crate::ast::ExprId, [1]expr["expr"]: crate::ast::ExprId, [2]file["file"]: crate::ast::FileId});
::differential_datalog::decl_struct_into_record!(WithinTypeofExpr, ["WithinTypeofExpr"]<>, type_of, expr, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(WithinTypeofExpr, <>, type_of: crate::ast::ExprId, expr: crate::ast::ExprId, file: crate::ast::FileId);
impl ::std::fmt::Display for WithinTypeofExpr {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            crate::WithinTypeofExpr{type_of,expr,file} => {
                __formatter.write_str("WithinTypeofExpr{")?;
                ::std::fmt::Debug::fmt(type_of, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(expr, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for WithinTypeofExpr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
::differential_datalog::decl_ddval_convert!{crate::ChainedWith}
::differential_datalog::decl_ddval_convert!{crate::ChildScope}
::differential_datalog::decl_ddval_convert!{crate::FunctionLevelScope}
::differential_datalog::decl_ddval_convert!{crate::IsExported}
::differential_datalog::decl_ddval_convert!{crate::NameInScope}
::differential_datalog::decl_ddval_convert!{crate::NoUndef}
::differential_datalog::decl_ddval_convert!{crate::TypeofUndef}
::differential_datalog::decl_ddval_convert!{crate::UnusedVariables}
::differential_datalog::decl_ddval_convert!{crate::UseBeforeDef}
::differential_datalog::decl_ddval_convert!{crate::VariableUsages}
::differential_datalog::decl_ddval_convert!{crate::WithinTypeofExpr}
::differential_datalog::decl_ddval_convert!{crate::ast::FileId}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::ClassId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::ExprId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FileId, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::FuncId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::ScopeId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::Span, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple2<crate::ast::StmtId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::ast::ScopeId, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::ScopeId, crate::ast::ScopeId, crate::ast::FileId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple3<crate::ast::StmtId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::StmtId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::ast::ScopeId, crate::internment::Intern<String>, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::StmtId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::ImportId, crate::ast::FileId, crate::ast::ImportClause, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::FileId, crate::ast::StmtId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::ImportId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::StmtId, crate::ast::FileId, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple4<crate::ast::StmtId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::ExprId, crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::AnyId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::FuncId, crate::ast::FileId, crate::internment::Intern<crate::ast::Pattern>, bool, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple5<crate::ast::Spanned<crate::internment::Intern<String>>, crate::ast::FuncId, crate::ast::FileId, bool, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::ExprId, crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::ExprId, crate::ast::FileId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::ExprId, crate::ast::FileId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::ClassId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple6<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<String>, crate::ast::Span, crate::ast::AnyId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::ClassId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::ExprId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::FuncId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple7<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::StmtId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::ExprId, crate::ast::FileId, crate::ast::ExprId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::StmtId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::AnyId, crate::ast::Span, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::ExprId, crate::ast::FileId, crate::ast::ScopeId, crate::ast::Span, crate::internment::Intern<String>, crate::ast::ClassId, crate::ast::Span, crate::ast::ScopeId>}
::differential_datalog::decl_ddval_convert!{crate::ddlog_std::tuple8<crate::ast::ExprId, crate::ast::FileId, crate::internment::Intern<String>, crate::ast::ScopeId, crate::ast::Span, crate::ast::AnyId, crate::ast::ScopeId, crate::ast::Span>}
::differential_datalog::decl_ddval_convert!{crate::inputs::Array}
::differential_datalog::decl_ddval_convert!{crate::inputs::Arrow}
::differential_datalog::decl_ddval_convert!{crate::inputs::ArrowParam}
::differential_datalog::decl_ddval_convert!{crate::inputs::Assign}
::differential_datalog::decl_ddval_convert!{crate::inputs::Await}
::differential_datalog::decl_ddval_convert!{crate::inputs::BinOp}
::differential_datalog::decl_ddval_convert!{crate::inputs::BracketAccess}
::differential_datalog::decl_ddval_convert!{crate::inputs::Break}
::differential_datalog::decl_ddval_convert!{crate::inputs::Call}
::differential_datalog::decl_ddval_convert!{crate::inputs::Class}
::differential_datalog::decl_ddval_convert!{crate::inputs::ClassExpr}
::differential_datalog::decl_ddval_convert!{crate::inputs::ConstDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::Continue}
::differential_datalog::decl_ddval_convert!{crate::inputs::DoWhile}
::differential_datalog::decl_ddval_convert!{crate::inputs::DotAccess}
::differential_datalog::decl_ddval_convert!{crate::inputs::EveryScope}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprBigInt}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprBool}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprNumber}
::differential_datalog::decl_ddval_convert!{crate::inputs::ExprString}
::differential_datalog::decl_ddval_convert!{crate::inputs::Expression}
::differential_datalog::decl_ddval_convert!{crate::inputs::File}
::differential_datalog::decl_ddval_convert!{crate::inputs::FileExport}
::differential_datalog::decl_ddval_convert!{crate::inputs::For}
::differential_datalog::decl_ddval_convert!{crate::inputs::ForIn}
::differential_datalog::decl_ddval_convert!{crate::inputs::Function}
::differential_datalog::decl_ddval_convert!{crate::inputs::FunctionArg}
::differential_datalog::decl_ddval_convert!{crate::inputs::If}
::differential_datalog::decl_ddval_convert!{crate::inputs::ImplicitGlobal}
::differential_datalog::decl_ddval_convert!{crate::inputs::ImportDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::InlineFunc}
::differential_datalog::decl_ddval_convert!{crate::inputs::InlineFuncParam}
::differential_datalog::decl_ddval_convert!{crate::inputs::InputScope}
::differential_datalog::decl_ddval_convert!{crate::inputs::Label}
::differential_datalog::decl_ddval_convert!{crate::inputs::LetDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::NameRef}
::differential_datalog::decl_ddval_convert!{crate::inputs::New}
::differential_datalog::decl_ddval_convert!{crate::inputs::Property}
::differential_datalog::decl_ddval_convert!{crate::inputs::Return}
::differential_datalog::decl_ddval_convert!{crate::inputs::Statement}
::differential_datalog::decl_ddval_convert!{crate::inputs::Switch}
::differential_datalog::decl_ddval_convert!{crate::inputs::SwitchCase}
::differential_datalog::decl_ddval_convert!{crate::inputs::Template}
::differential_datalog::decl_ddval_convert!{crate::inputs::Ternary}
::differential_datalog::decl_ddval_convert!{crate::inputs::Throw}
::differential_datalog::decl_ddval_convert!{crate::inputs::Try}
::differential_datalog::decl_ddval_convert!{crate::inputs::UnaryOp}
::differential_datalog::decl_ddval_convert!{crate::inputs::VarDecl}
::differential_datalog::decl_ddval_convert!{crate::inputs::While}
::differential_datalog::decl_ddval_convert!{crate::inputs::With}
::differential_datalog::decl_ddval_convert!{crate::inputs::Yield}