use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let syn_tree = parse_macro_input!(input as DeriveInput);
    let name = syn_tree.ident;

    let fields  = if let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) =  syn_tree.data{
        fields.named
    }else{
       panic!("Only supported for structs")
    };

    let field_name: Vec<proc_macro2::Ident> = fields.iter().map(|field| field.ident.clone().unwrap()).collect();
    let field_type : Vec<syn::Type> = fields.iter().map(|field| field.ty.clone()).collect();
    // eprintln!("INPUT: {:#?}", field_name );

    let builder_name =
        proc_macro2::Ident::new(&format!("{}Builder", name), proc_macro2::Span::call_site());

    let tokens = quote! {
         pub struct  #builder_name{
             #(#field_name: Option<#field_type>,)*
         }

        impl #name{
            pub fn builder() -> #builder_name{
                #builder_name{
                    #(#field_name: None,)*
                }
            }
        }
        
        impl #builder_name{
            #(fn #field_name(&mut self, #field_name: #field_type) -> &mut Self{
                self.#field_name = Some(#field_name);
                self
            })*

            fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>>{
                #(
                    let #field_name = self.#field_name.clone().ok_or_else(||"field missing")?;
                 )*
                // let executable = self.executable.clone().ok_or_else(|| "Executable was false")?;
                // let args = self.args.clone().ok_or_else(|| "Executable was false")?;
                // let env= self.env.clone().ok_or_else(|| "Executable was false")?;
                // let current_dir = self.current_dir.clone().ok_or_else(|| "Executable was false")?;
                let construct = #name{
                    #(#field_name,)*
                };
                Ok(construct)
            }
        }
    };
    TokenStream::from(tokens)
}
