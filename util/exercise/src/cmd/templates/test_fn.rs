{% import "macros.rs" as macros %}
#[test]
#[ignore]
/// {{ description }}
{% if comments -%}
    ///
    {% for comment in comments %}
        /// {{ comment }}
    {% endfor %}
{% endif -%}
fn test_{{ description | slugify | replace(from="-", to="_") }}() {
    process_{{ property | slugify | replace(from="-", to="_")}}_case(
        {{ macros::to_literal(value=input, use_maplit=use_maplit) }},
        {{ macros::to_literal(value=expected, use_maplit=use_maplit) }}
    );
}
