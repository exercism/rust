{% import "macros.rs" as macros -%}

#[test]
{% if not first_test_case -%}
#[ignore]
{% endif -%}
/// {{ description }}
{% if comments -%}
    ///
    {% for comment in comments %}
        /// {{ comment }}
    {% endfor %}
{% endif -%}
fn test_{{ format_description(description=description) }}() {
    process_{{ format_property(property=property) }}_case(
        {{ macros::to_literal(value=input, use_maplit=use_maplit) }},
        {{ macros::to_literal(value=expected, use_maplit=use_maplit) }}
    );
}
