use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields,TypePath, Type, PathArguments,GenericArgument};


fn is_option(t : &syn::Type)->bool{
    if let Type::Path(TypePath{
        qself: None,
        path: p  
    }) = t{
        return p.segments.iter().any(|segment| segment.ident == "Option")
    }
    else{
        return false
    }
}

fn extract_option_type(t : &Type)->Type{
    if let Type::Path(TypePath{
        qself: None,
        path: p  
    }) = t{
        let args = &p.segments.iter().next().unwrap().arguments;
        let generic_arg = match args{
                PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                _ => panic!("TODO: error handling"),
        };

        match generic_arg{
            GenericArgument::Type(ty)=>ty.clone(),
            _ => panic!("Failed to get option inner")
        }
    }
    else {
        panic!("Was passed something that doesn't look like an option")
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let syn_tree = parse_macro_input!(input as DeriveInput);
    let name = syn_tree.ident;

    let fields  = if let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) =  syn_tree.data{
        fields.named
    }else{
       panic!("Only supported for structs")
    };

    let option_fields : Vec<_> = fields.iter().filter(|field| is_option(&field.ty)).collect();
    let other_fields : Vec<_> = fields.iter().filter(|field| !is_option(&field.ty)).collect();


    let option_field_names: Vec<proc_macro2::Ident> = option_fields.iter().map(|field| field.ident.clone().unwrap()).collect();
    let option_field_types : Vec<syn::Type> = option_fields.iter().map(|field| extract_option_type(&field.ty)).collect();

    let other_field_names: Vec<proc_macro2::Ident> = other_fields.iter().map(|field| field.ident.clone().unwrap()).collect();
    let other_field_types : Vec<syn::Type> = other_fields.iter().map(|field| field.ty.clone()).collect();


    let builder_name =
        proc_macro2::Ident::new(&format!("{}Builder", name), proc_macro2::Span::call_site());

    let tokens = quote! {
         pub struct  #builder_name{
             #(#option_field_names: Option<#option_field_types>,)*
             #(#other_field_names: Option<#other_field_types>,)*
         }

        impl #name{
            pub fn builder() -> #builder_name{
                #builder_name{
                    #(#other_field_names: None,)*
                    #(#option_field_names: None,)*
                }
            }
        }
        
        impl #builder_name{
            #(fn #other_field_names(&mut self, #other_field_names: #other_field_types) -> &mut Self{
                self.#other_field_names = Some(#other_field_names);
                self
            })*

            #(fn #option_field_names(&mut self, #option_field_names: #option_field_types) -> &mut Self{
                self.#option_field_names = Some(#option_field_names);
                self
            })*

            fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>>{
                #(
                    let #other_field_names = self.#other_field_names.clone().ok_or_else(||"field missing")?;
                 )*
                #(
                    let #option_field_names = self.#option_field_names.clone();
                 )*

                let construct = #name{
                    #(#option_field_names,)*
                    #(#other_field_names,)*
                };
                Ok(construct)
            }
        }
    };
    TokenStream::from(tokens)
}
