use object::Object;
use proc_macro::TokenStream;

mod object;

#[proc_macro_derive(ToJSON)]
pub fn to_json(input: TokenStream) -> TokenStream {
    for token in input.clone() {
        println!("{:?}", token);
    }

    let derive = Object::parse(input, "ToJSON");

    todo!("Generate TryInto<Value> implementation");
}
