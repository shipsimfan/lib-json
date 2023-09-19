use crate::{keyword::Keyword, Generator, Struct};
use proc_macro::{Delimiter, Spacing, TokenStream};

pub(super) fn generate(r#struct: Struct) -> TokenStream {
    let mut generator = Generator::new();
    generate_into_value(&r#struct, &mut generator);
    generate_to_json(&r#struct, &mut generator);
    generator.to_token_stream()
}

fn generate_into_value(r#struct: &Struct, generator: &mut Generator) {
    generator.push_keyword(Keyword::Impl);
    r#struct
        .generic()
        .map(|generic| generic.generate(generator));

    generator.push_ident_str("Into");
    generator.push_punct('<', Spacing::Alone);
    generator.push_type_path(&["json", "Value"], true);
    generator.push_punct('>', Spacing::Alone);

    generator.push_keyword(Keyword::For);

    generator.push_ident(r#struct.ident().clone());
    r#struct
        .generic()
        .map(|generic| generic.generate_without_qualifiers(generator));

    let mut impl_body_generator = Generator::new();
    impl_body_generator.push_keyword(Keyword::Fn);
    impl_body_generator.push_ident_str("into");

    let mut parameters_generator = Generator::new();
    parameters_generator.push_ident_str("self");
    impl_body_generator.push_group(
        parameters_generator.to_token_stream(),
        Delimiter::Parenthesis,
    );

    impl_body_generator.push_punct('-', Spacing::Joint);
    impl_body_generator.push_punct('>', Spacing::Alone);

    impl_body_generator.push_type_path(&["json", "Value"], true);

    let mut function_body_generator = Generator::new();
    generate_into_fn_body(r#struct, &mut function_body_generator);
    impl_body_generator.push_group(function_body_generator.to_token_stream(), Delimiter::Brace);

    generator.push_group(impl_body_generator.to_token_stream(), Delimiter::Brace);
}

fn generate_into_fn_body(r#struct: &Struct, generator: &mut Generator) {
    todo!("Generate into() body");
}

fn generate_to_json(r#struct: &Struct, generator: &mut Generator) {
    todo!("Generate struct ToJSON")
}

/*

struct Test;

impl Into<crate::Value> for Test {
    fn into(self) -> crate::Value {}
}

*/
