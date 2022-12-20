use {crate::ast::value::DomainValueAttr,
     convert_case::{Case,
                    Casing},
     proc_macro2::Ident,
     quote::{format_ident,
             ToTokens},
     syn::{Data,
           DeriveInput,
           Field,
           Fields,
           FieldsNamed,
           GenericArgument,
           PathArguments,
           Type}};

pub struct DomainDefAst {
  pub root_name_ident: Ident,
  pub fields_named:    FieldsNamed,
}

impl DomainDefAst {
  pub fn new(derive_input: DeriveInput) -> Self {
    let name = derive_input.ident.clone();
    let name_string = name.to_string();
    if !name_string.ends_with("__") || !name_string.starts_with("__") {
      panic!("DomainDef should started and ended with __");
    }
    let root_name = &name_string[2 .. (name_string.len() - 2)];
    let root_name_ident = format_ident!("{}", root_name);
    match derive_input.data {
      | Data::Struct(data) => match data.fields {
        | Fields::Named(ref fields) => {
          let fields_named = fields.clone();
          DomainDefAst { root_name_ident,
                         fields_named }
        }
        | _ => panic!("DomainDTO should be fields-named"),
      },
      | _ => panic!("DomainDTO should be Struct"),
    }
  }
}

pub fn is_type_option(ty: &Type) -> bool {
  let path = if let Type::Path(ref type_path) = ty {
    if type_path.qself.is_some() {
      return false;
    }
    &type_path.path
  } else {
    return false;
  };
  if let Some(segment) = path.segments.last() {
    if segment.ident != "Option" {
      return false;
    }
    let generic_params = if let PathArguments::AngleBracketed(generic_params) = &segment.arguments {
      generic_params
    } else {
      return false;
    };
    return if let GenericArgument::Type(_) = generic_params.args.first().unwrap() {
      true
    } else {
      false
    };
  } else {
    return false;
  }
}

pub fn type_in_option_or_itself(ty: Type) -> Type {
  let path = if let Type::Path(ref type_path) = ty {
    if type_path.qself.is_some() {
      return ty;
    }
    &type_path.path
  } else {
    return ty;
  };
  if let Some(segment) = path.segments.last() {
    if segment.ident != "Option" {
      return ty;
    }
    let generic_params = if let PathArguments::AngleBracketed(generic_params) = &segment.arguments {
      generic_params
    } else {
      return ty;
    };
    if let GenericArgument::Type(ty) = generic_params.args.first().unwrap() {
      return ty.clone();
    } else {
      return ty;
    }
  } else {
    return ty;
  }
}

pub fn value_type(root_name_ident: &Ident, f: &Field) -> Box<dyn ToTokens> {
  let name = &f.ident;
  let name = name.as_ref().unwrap();
  let skip = DomainValueAttr::parse_from(f).skip;
  let value_type: Box<dyn ToTokens> = if skip {
    Box::new(f.ty.clone())
  } else {
    Box::new(format_ident!("{}{}", root_name_ident, name.to_string().to_case(Case::Pascal)))
  };
  value_type
}
