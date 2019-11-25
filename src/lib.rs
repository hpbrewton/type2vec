extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate nalgebra as na;
use na::{DVector};
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::Ident as ident;

trait Vectorizer {
    fn vectorize(&self) -> DVector<u8>;
}

impl Vectorizer for u8 {
    fn vectorize(&self) -> DVector<u8> {
        DVector::from_vec(vec![1])
    }
}

#[proc_macro_derive(Vectorizer)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let ast : syn::DeriveInput = syn::parse(item.clone()).unwrap();
    match ast.data {
        syn::Data::Struct(data) => {
            quote! {
                impl Vectorizer for Point {
                    fn vectorize(&self) -> DVector<u8> {
                        
                    }
                }
            }
            for x in data.fields {
                match x.ty {
                    syn::Type::Verbatim(x) => println!("{:?}", x),
                    syn::Type::Path(path) => println!("{:?}", path.path.get_ident()),
                    _ => println!("other")
                }
            }
        },
        _ => {}
    }
    return TokenStream::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, answer());
    }
}
