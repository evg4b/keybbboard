mod events;
use std::collections::HashMap;

pub fn key_binding() -> HashMap {
    let mut key_mapping = HashMap::new();

    // Маппинг русских букв в английские для раскладки QWERTY
    key_mapping.insert('й', 'q');
    key_mapping.insert('ц', 'w');
    key_mapping.insert('у', 'e');
    key_mapping.insert('к', 'r');
    key_mapping.insert('е', 't');
    key_mapping.insert('н', 'y');
    key_mapping.insert('г', 'u');
    key_mapping.insert('ш', 'i');
    key_mapping.insert('щ', 'o');
    key_mapping.insert('з', 'p');
    key_mapping.insert('х', '[');
    key_mapping.insert('ъ', ']');
    key_mapping.insert('ф', 'a');
    key_mapping.insert('ы', 's');
    key_mapping.insert('в', 'd');
    key_mapping.insert('а', 'f');
    key_mapping.insert('п', 'g');
    key_mapping.insert('р', 'h');
    key_mapping.insert('с', 'j');
    key_mapping.insert('м', 'k');
    key_mapping.insert('и', 'l');
    key_mapping.insert('т', ';');
    key_mapping.insert('ь', '\'');
    key_mapping.insert('б', 'z');
    key_mapping.insert('ю', 'x');
    key_mapping.insert('я', 'c');
    key_mapping.insert('ё', '`');
}