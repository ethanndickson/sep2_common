use syn::{Data, Fields, GenericArgument, Ident, PathArguments, Type};

pub fn extract_list(data: &Data) -> Option<(Ident, Type)> {
    if let Data::Struct(data) = data {
        if let Fields::Named(fields) = &data.fields {
            if let Some(field) = &fields.named.first() {
                let ident = field.ident.clone()?;
                if let Type::Path(vec_typepath) = &field.ty {
                    if let Some(segment) = vec_typepath.path.segments.first() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(out_type)) = args.args.first() {
                                return Some((ident, out_type.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
