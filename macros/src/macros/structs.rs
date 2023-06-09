use proc_macro2::Ident;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path, Type, TypePath};

pub struct Fd {
    pub name: Ident,
    pub ty: Type,
    pub optional: bool,
}

/// 包含了用于实现宏的一些信息
pub struct Context {
    pub name: Ident,
    pub fields: Vec<Fd>,
}

impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        Self {
            name: f.ident.unwrap(),
            ty: ty.to_owned(),
            optional,
        }
    }
}

fn get_option_inner(ty: &Type) -> (bool, &Type) {
    if let Type::Path(TypePath {
                          path: Path { segments, .. },
                          ..
                      }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument")
                    },
                    _ => panic!("Not sure what to do with other PathArgument")
                };

                return (true, t);
            }
        }
    }

    return (false, ty);
}

impl From<DeriveInput> for Context {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
                                             fields: Fields::Named(FieldsNamed { named, .. }),
                                             ..
                                         }) = input.data {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields.into_iter().map(Fd::from).collect();

        Self {
            name, fields: fds
        }
    }
}