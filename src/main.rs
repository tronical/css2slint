use clap::Arg;
use lightningcss::properties::custom::{CustomPropertyName, TokenOrValue};
use lightningcss::properties::Property;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};

fn main() -> std::io::Result<()> {
    let matches = clap::App::new("css2slint")
        .args(&[
            Arg::from_usage("[input] 'an optional input file to use'"),
            Arg::from_usage("[selector] 'selector to match'"),
        ])
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of("input").unwrap()).unwrap();

    let sheet = StyleSheet::parse(&input, ParserOptions::default()).unwrap();

    let mut var_names: Vec<String> = Default::default();
    let mut var_types: Vec<String> = Default::default();
    let mut var_values: Vec<String> = Default::default();

    for rule in sheet.rules.0 {
        let style_rule = match rule {
            CssRule::Style(style_rule) => style_rule,
            _ => continue,
        };
        for decl in style_rule.declarations.declarations {
            let custom_prop = match &decl {
                Property::Custom(custom_prop) => custom_prop,
                _ => continue,
            };

            let prop_name = match &custom_prop.name {
                CustomPropertyName::Custom(dashed_ident) => dashed_ident.to_string(),
                _ => continue,
            };
            //eprintln!("{:#?}, prop_name = {}", decl, prop_name);

            let name_without_prefix = match prop_name.strip_prefix("--") {
                Some(name) => name,
                None => continue,
            };

            let (value, var_type) = match custom_prop.value.0.first() {
                Some(TokenOrValue::Color(color)) => match color {
                    lightningcss::values::color::CssColor::CurrentColor => continue,
                    lightningcss::values::color::CssColor::RGBA(rgba) => (
                        format!(
                            "rgba({}, {}, {}, {})",
                            rgba.red,
                            rgba.green,
                            rgba.blue,
                            rgba.alpha_f32()
                        ),
                        "color",
                    ),
                    lightningcss::values::color::CssColor::LAB(_) => continue,
                    lightningcss::values::color::CssColor::Predefined(_) => continue,
                    lightningcss::values::color::CssColor::Float(_) => continue,
                },
                _ => continue,
            };

            var_names.push(name_without_prefix.into());
            var_types.push(var_type.into());
            var_values.push(value);
        }
    }

    println!("export struct CSSVariables := {{");
    for (property_name, property_type) in var_names.iter().zip(var_types) {
        println!("    {}: {},", property_name, property_type);
    }
    println!("}}");

    println!("export global CSSVariableValues := {{");
    println!("    property<CSSVariables> values: {{");
    for (property_name, property_value) in var_names.iter().zip(var_values) {
        println!("        {}: {},", property_name, property_value);
    }
    println!("    }};");
    println!("}}");

    Ok(())
}
