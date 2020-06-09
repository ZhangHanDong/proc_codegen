use super::*;

/// gen_struct! {
///     Person -> {name: String, age: u32}
/// }

pub(crate) struct Genstruct {
    pub name: Ident,
    pub brace_token: token::Brace,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for Genstruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let fields;

        let name: Ident = input.parse()?;
        input.parse::<Token![->]>()?;
        let brace_token = braced!(fields in input);
        let fields = fields.parse_terminated(Field::parse_named)?;

        Ok(Genstruct {
            name,
            brace_token,
            fields,
        })
    }
}


pub(crate) fn struct_to_tokenstream(input: TokenStream) -> TokenStream {
    let Genstruct {
        name,
        brace_token: _,
        fields,
    } = parse_macro_input!(input as Genstruct);

    let fields_quote = fields.iter().map(
        |field| {
            let name = &field.ident;
            let ty = &field.ty;
            quote!(#name: #ty)
        }
    );

    let args_quote = fields.iter().map(
        |field| {
            let name = &field.ident;
            let ty = &field.ty;
            quote!(#name: #ty)
        }
    );


    let args_name_quote = fields.iter().map(
        |field| {
            let name = &field.ident;
            quote!(#name)
        }
    );

    let struct_name = util::to_ident(name.to_string().to_camel_case().as_ref());

    let expanded = quote! {

            #[derive(Debug)]
            pub struct #struct_name {
                #( #fields_quote, )*
            }

            impl #struct_name {
                pub fn new( #(#args_quote,)* ) -> Self {
                    #struct_name { #(#args_name_quote,)* }
                 }
            }

    };

    TokenStream::from(expanded)
}


pub(crate) struct ConfPath {
    pub dir: Ident,
    pub file: Ident,
}


// Conf {
//
// basic: Basic { derive: "Debug" },
//
// structs: [
//     StructEntry {
//       name: "person",
//       tagged: "named",
//       fields: Some({"age": String("u32"), "name": String("String")})
//     },
// ] }

pub(crate) fn struct_by_conf_to_tokenstream(input: TokenStream) -> TokenStream {
    let conf = Conf::read_config();

    let derive_name = util::to_ident(conf.basic.derive.as_ref());
    let structs_quote = conf.structs.iter().map(|s| {
        let mut fields_quote = {
            let mut args = vec![];
            for (k, v) in s.fields.as_ref().unwrap().iter() {
                let name = util::to_ident(k);
                let ty = util::to_ident(v.as_str().unwrap());
                args.push(quote!(#name: #ty))
            }
            args
        };

        let mut args_quote = {
            let mut args = vec![];
            for (k, v) in s.fields.as_ref().unwrap().iter() {
                let name = util::to_ident(k);
                let ty = util::to_ident(v.as_str().unwrap());
                args.push(quote!(#name: #ty))
            }
            args
        };

        let mut args_name_quote = {
            let mut args = vec![];
            for (k, _v) in s.fields.as_ref().unwrap().iter() {
                let name = util::to_ident(k);
                args.push(quote!(#name))
            }
            args
        };

        let struct_name = util::to_ident(s.name.to_string().to_camel_case().as_ref());
        quote!{
            #[derive(#derive_name)]
            pub struct #struct_name {
                #( #fields_quote, )*
            }

            impl #struct_name {
                pub fn new( #(#args_quote,)* ) -> Self {
                    #struct_name { #(#args_name_quote,)* }
                }
            }
        }

    });


    let expanded = quote! {
            #( #structs_quote )*
    };

    TokenStream::from(expanded)
}

