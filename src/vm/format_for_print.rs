use super::StackValue;

pub fn format_for_print(value: StackValue, newline: bool) -> String {
    match value {
        StackValue::BOOL { value } => {
            if !newline {
                if value {
                    return "1".to_owned();
                } else {
                    return "0".to_owned();
                }
            } else {
                if value {
                    return "1\n".to_owned();
                } else {
                    return "0\n".to_owned();
                }
            }
        }
        StackValue::BIGINT { value } => {
            if !newline {
                return format!("{}", value);
            } else {
                return format!("{}\n", value);
            }
        }
        StackValue::STRING { value } => {
            if !newline {
                return format!("{}", value);
            } else {
                return format!("{}\n", value);
            }
        }
        StackValue::ARRAY { value } => {
            let mut printable_str: String = "[".to_string();
            let mut counter = 0;
            for i in &value {
                let formated: String = format_for_print(i.clone(), false).as_str().to_owned();
                match i {
                    StackValue::STRING { value: _ } => {
                        printable_str = printable_str + "\"" + &formated + "\"";
                    }
                    _ => {
                        printable_str += &formated;
                    }
                }

                if counter != value.len() - 1 {
                    printable_str += ";";
                }
                counter += 1;
            }
            if newline {
                printable_str += "]\n";
            } else {
                printable_str += "]";
            }
            return printable_str;
        }/* 
        StackValue::OBJECT { name, value: fields } => {
            let mut printable_object = format!("{} {{\n", name);
            let mut index = 0;
            let length = &fields.len();
            for field in fields {
                printable_object += &format!(
                    "   {}: {}",
                    field.name,
                    format_for_print(field.value, false)
                );
                if &(index + 2) == length {
                    printable_object += "\n";
                }
                index += 1;
            }
            printable_object += "\n}";
            if !newline {
                return format!("{}", printable_object);
            } else {
                return format!("{}\n", printable_object);
            }
        }*/
        StackValue::FLOAT { value } => {
            if !newline {
                return format!("{}", value.to_string().replace(".", ","));
            } else {
                return format!("{}\n", value.to_string().replace(".", ","));
            }
        }
    }
}