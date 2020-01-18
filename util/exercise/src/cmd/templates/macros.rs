{% macro to_literal(value, use_maplit=false) -%}
    {% if value is object -%}
        {% if use_maplit -%}
            hashmap! {
                {% for k, v in value -%}
                {{ self::to_literal(value=k, use_maplit=use_maplit) }} =>
                {{ self::to_literal(value=v, use_maplit=use_maplit) }},
                {% endfor -%}
            }
        {% else -%}
            {
                let mut hm = ::std::collections::HashMap::new();
                    {% for k, v in value -%}
                    hm.insert(
                        {{ self::to_literal(value=k, use_maplit=use_maplit) }},
                        {{ self::to_literal(value=v, use_maplit=use_maplit) }}
                    );
                    {% endfor -%}
                hm
            }
        {% endif -%}
    {% elif value is iterable -%}
        vec![
            {% for element in value -%}
            {{ self::to_literal(value=element, use_maplit=use_maplit) }},
            {% endfor -%}
        ]
    {% elif value is string -%}
        "{{ value }}"
    {% elif value is number -%}
        {{ value }}
    {% else -%}
        None
    {% endif -%}
{% endmacro -%}
