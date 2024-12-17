use convert_case::{Boundary, Case, Casing};
use std::{
    fmt::Display,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::webview2_path::*;

pub fn output_functions() -> crate::Result<bool> {
    let source_path = generate_bindings()?;
    format_bindings(&source_path)?;

    let source = read_bindings(&source_path)?;

    let mut dest_path = get_webview2_com_dir()?;
    dest_path.push("src");
    dest_path.push("webview2.rs");
    let dest = read_bindings(&dest_path)?;

    if source != dest {
        fs::copy(&source_path, &dest_path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Unrecognized {
    ReturnType,
    FnArg,
    Generic,
    Parse(syn::Error),
}

impl From<syn::Error> for Unrecognized {
    fn from(value: syn::Error) -> Self {
        Self::Parse(value)
    }
}

#[derive(Debug)]
enum FnReturnType {
    Empty,
    PWSTR,
    Other(String),
}

impl Display for FnReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " -> windows_core::Result<()>"),
            Self::PWSTR => write!(f, " -> windows_core::Result<String>"),
            Self::Other(ty) => write!(f, " -> windows_core::Result<{ty}>"),
        }
    }
}

impl TryFrom<&syn::ReturnType> for FnReturnType {
    type Error = Unrecognized;

    fn try_from(value: &syn::ReturnType) -> Result<Self, Self::Error> {
        let syn::ReturnType::Type(_, return_type) = value else {
            return Err(Unrecognized::ReturnType);
        };

        let syn::Type::Path(syn::TypePath { qself: None, path }) = return_type.as_ref() else {
            return Err(Unrecognized::ReturnType);
        };

        let segments = &path.segments;
        let path = path_to_string(path);
        if path != "windows_core::Result" {
            return Err(Unrecognized::ReturnType);
        }

        segments
            .iter()
            .rev()
            .next()
            .and_then(|s| match &s.arguments {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) => Some(args),
                _ => None,
            })
            .and_then(|args| args.iter().next())
            .and_then(|arg| match arg {
                syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) => {
                    let path = path_to_string(path);
                    if path == "windows_core::PWSTR" {
                        Some(Self::PWSTR)
                    } else {
                        None
                    }
                }
                syn::GenericArgument::Type(syn::Type::Tuple(syn::TypeTuple { elems, .. }))
                    if elems.is_empty() =>
                {
                    Some(Self::Empty)
                }
                _ => None,
            })
            .ok_or(Unrecognized::ReturnType)
    }
}

#[derive(Debug)]
struct FnGenericArg {
    name: String,
    bound: String,
}

impl Display for FnGenericArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name}: {bound}", name = self.name, bound = self.bound)
    }
}

#[derive(Debug)]
struct FnGenerics {
    args: Vec<FnGenericArg>,
}

impl Display for FnGenerics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .args
            .iter()
            .map(|arg| format!("{arg}"))
            .collect::<Vec<_>>()
            .join(", ");
        if !args.is_empty() {
            write!(f, "<{args}>", args = args)
        } else {
            Ok(())
        }
    }
}

impl TryFrom<&syn::Generics> for FnGenerics {
    type Error = Unrecognized;

    fn try_from(value: &syn::Generics) -> Result<Self, Self::Error> {
        let where_clause = value.where_clause.as_ref().ok_or(Unrecognized::Generic)?;
        let args = where_clause
            .predicates
            .iter()
            .map(|predicate| match predicate {
                syn::WherePredicate::Type(syn::PredicateType {
                    bounded_ty, bounds, ..
                }) => {
                    let name = match bounded_ty {
                        syn::Type::Path(syn::TypePath { qself: None, path }) => {
                            Ok(path_to_string(path))
                        }
                        _ => Err(Unrecognized::Generic),
                    }?;
                    let bound = bounds
                        .iter()
                        .map(|bound| match bound {
                            syn::TypeParamBound::Trait(syn::TraitBound { path, .. }) => {
                                match (
                                    path_to_string(path).as_str(),
                                    path.segments.iter().rev().next(),
                                ) {
                                    ("windows_core::Param", Some(segment)) => {
                                        match &segment.arguments {
                                            syn::PathArguments::AngleBracketed(
                                                syn::AngleBracketedGenericArguments {
                                                    args, ..
                                                },
                                            ) => match args.iter().next() {
                                                Some(syn::GenericArgument::Type(
                                                    syn::Type::Path(syn::TypePath { path, .. }),
                                                )) => {
                                                    let path = path_to_string(path);
                                                    Ok(format!("windows_core::Param<{path}>"))
                                                }
                                                _ => Err(Unrecognized::Generic),
                                            },
                                            _ => Err(Unrecognized::Generic),
                                        }
                                    }
                                    _ => Err(Unrecognized::Generic),
                                }
                            }
                            _ => Err(Unrecognized::Generic),
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .join(" + ");
                    Ok(FnGenericArg { name, bound })
                }
                _ => Err(Unrecognized::Generic),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { args })
    }
}

#[derive(Debug)]
struct FnArg {
    name: String,
    ty: Option<String>,
}

impl Display for FnArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.name, &self.ty) {
            (name, Some(ty)) => write!(f, "{name}: {ty}"),
            (name, None) => write!(f, "{name}"),
        }
    }
}

impl TryFrom<&syn::FnArg> for FnArg {
    type Error = Unrecognized;

    fn try_from(value: &syn::FnArg) -> Result<Self, Self::Error> {
        let arg = match value {
            syn::FnArg::Receiver(syn::Receiver {
                reference: Some(_),
                mutability: Some(_),
                ..
            }) => Self {
                name: "&mut self".to_owned(),
                ty: None,
            },
            syn::FnArg::Receiver(syn::Receiver {
                reference: Some(_),
                mutability: None,
                ..
            }) => Self {
                name: "&self".to_owned(),
                ty: None,
            },
            syn::FnArg::Receiver(_) => Self {
                name: "self".to_owned(),
                ty: None,
            },
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                let name = match pat.as_ref() {
                    syn::Pat::Ident(syn::PatIdent { ident, .. }) => Ok(ident.to_string()),
                    _ => Err(Unrecognized::FnArg),
                }?;
                let ty = visit_arg_type(ty.as_ref()).ok_or(Unrecognized::FnArg)?;
                Self { name, ty: Some(ty) }
            }
        };

        Ok(arg)
    }
}

struct GlobalFn {
    name: String,
    generics: FnGenerics,
    args: Vec<FnArg>,
    return_type: Option<FnReturnType>,
}

impl Display for GlobalFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let generics = &self.generics;
        let args = self
            .args
            .iter()
            .map(|arg| format!("{arg}"))
            .collect::<Vec<_>>()
            .join(", ");
        let return_type = self
            .return_type
            .as_ref()
            .map(|return_type| format!("{return_type}"))
            .unwrap_or_default();

        writeln!(f, "pub fn {name}{generics}({args}){return_type} {{")?;
        writeln!(f, "    todo!(\"Implement {name}\");")?;
        writeln!(f, "}}")
    }
}

impl TryFrom<&syn::ItemFn> for GlobalFn {
    type Error = Unrecognized;

    fn try_from(value: &syn::ItemFn) -> Result<Self, Self::Error> {
        let name = snake_case(&value.sig.ident);

        let generics = FnGenerics::try_from(&value.sig.generics)?;
        let mut args: Vec<_> = value
            .sig
            .inputs
            .iter()
            .map(|arg| FnArg::try_from(arg))
            .collect::<Result<_, _>>()?;
        let mut return_type = FnReturnType::try_from(&value.sig.output)?;

        if let (Some(last_arg), FnReturnType::Empty) = (args.last(), &return_type) {
            if let Some(ty) = &last_arg.ty {
                if ty.starts_with("*mut ") {
                    let path = ty.trim_start_matches("*mut ");
                    return_type = if path == "windows_core::PWSTR" {
                        FnReturnType::PWSTR
                    } else {
                        FnReturnType::Other(path.to_owned())
                    };
                    args.pop();
                }
            }
        }

        Ok(Self {
            name,
            generics,
            args,
            return_type: Some(return_type),
        })
    }
}

fn visit_item(file: &mut fs::File, item: &syn::Item) -> crate::Result<()> {
    match item {
        syn::Item::Mod(item_mod) => {
            if let Some((_, content)) = &item_mod.content {
                // let name = snake_case(&item_mod.ident);
                // writeln!(file, "pub mod {name} {{")?;

                for item in content {
                    visit_item(file, item)?;
                }

                // writeln!(file, "}}")?;
            }
        }
        syn::Item::Fn(item_fn) => {
            let original_name = &item_fn.sig.ident;
            if let Ok(item_fn) = GlobalFn::try_from(item_fn) {
                writeln!(file, "/// [`{original_name}`]")?;
                writeln!(file, "{item_fn}")?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn visit_arg_type(arg: &syn::Type) -> Option<String> {
    match arg {
        syn::Type::Path(syn::TypePath { path, .. }) => Some(path_to_string(path)),
        syn::Type::Reference(syn::TypeReference {
            mutability: Some(_),
            elem,
            ..
        }) => Some(format!("&mut {}", visit_arg_type(elem.as_ref())?)),
        syn::Type::Reference(syn::TypeReference {
            mutability: None,
            elem,
            ..
        }) => Some(format!("&{}", visit_arg_type(elem.as_ref())?)),
        syn::Type::Ptr(syn::TypePtr {
            const_token: Some(_),
            mutability: None,
            elem,
            ..
        }) => Some(format!("*const {}", visit_arg_type(elem.as_ref())?)),
        syn::Type::Ptr(syn::TypePtr {
            const_token: None,
            mutability: Some(_),
            elem,
            ..
        }) => Some(format!("*mut {}", visit_arg_type(elem.as_ref())?)),
        _ => None,
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.leading_colon
        .map(|_| "::".to_string())
        .into_iter()
        .chain(
            path.segments
                .iter()
                .map(|segment| segment.ident.to_string()),
        )
        .collect::<Vec<_>>()
        .join("::")
}

fn snake_case(value: &impl Display) -> String {
    value
        .to_string()
        .from_case(Case::UpperCamel)
        .without_boundaries(&[Boundary::LowerDigit])
        .to_case(Case::Snake)
        .replace("_core_web_view2_", "_")
}

fn parse_bindings() -> crate::Result<syn::File> {
    let mut src_path = get_bindings_dir()?;
    src_path.push("src");
    src_path.push("Microsoft.rs");
    let src = fs::read_to_string(src_path)?;
    Ok(syn::parse_file(&src)?)
}

fn generate_bindings() -> crate::Result<PathBuf> {
    let mut source_path = get_out_dir();
    source_path.push("webview2.rs");
    let mut source_file = fs::File::create(source_path.clone())?;

    writeln!(
        &mut source_file,
        "use webview2_com_sys::Microsoft::Web::WebView2::Win32::*;\n"
    )?;

    let bindings = parse_bindings()?;
    for item in &bindings.items {
        visit_item(&mut source_file, item)?;
    }

    Ok(source_path)
}

fn read_bindings(source_path: &Path) -> crate::Result<String> {
    let mut source_file = fs::File::open(source_path)?;
    let mut updated = String::default();
    source_file.read_to_string(&mut updated)?;
    Ok(updated)
}

fn format_bindings(source_path: &Path) -> crate::Result<()> {
    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(source_path);
    cmd.output()?;
    Ok(())
}
