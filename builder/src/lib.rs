use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let syn_tree = parse_macro_input!(input as DeriveInput);
    let name = syn_tree.ident;
    let builder_name =
        proc_macro2::Ident::new(&format!("{}Builder", name), proc_macro2::Span::call_site());
    let tokens = quote! {
        pub struct  #builder_name{
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>
        }
        impl #name{
            pub fn builder() -> #builder_name{
                #builder_name{
                    executable: None,
                    args: None,
                    env:None,
                    current_dir:None
                }
            }
        }

        use std::error;
        impl #builder_name{
            fn executable(&mut self, executable: String)-> &mut Self{
                self.executable = Some(executable);
                self
            }
            fn current_dir(&mut self, current_dir: String)-> &mut Self{
                self.current_dir= Some(current_dir);
                self
            }
            fn args(&mut self, args: Vec<String>)->&mut Self{
                self.args = Some(args);
                self
            }
            fn env(&mut self, env: Vec<String>)->&mut Self{
                self.env= Some(env);
                self
            }
            fn build(&mut self) -> Result<#name, Box<dyn error::Error>>{
                let executable = self.executable.clone().ok_or_else(|| "Executable was false")?;
                let args = self.args.clone().ok_or_else(|| "Executable was false")?;
                let env= self.env.clone().ok_or_else(|| "Executable was false")?;
                let current_dir = self.current_dir.clone().ok_or_else(|| "Executable was false")?;
                let construct = #name{
                    executable,
                    args,
                    env,
                    current_dir
                };
                Ok(construct)
            }
        }
    };
    TokenStream::from(tokens)
}
