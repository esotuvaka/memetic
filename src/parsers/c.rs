pub fn parse_c_structs(file_content: &str) -> Vec<(String, Vec<(String, String)>)> {
    let mut structs = Vec::new();
    let mut in_struct = false;
    let mut current_struct_name = String::new();
    let mut fields = Vec::new();

    for line in file_content.lines() {
        if line.trim().starts_with("struct") {
            in_struct = true;
            current_struct_name = line.split_whitespace().nth(1).unwrap().to_string();
            fields.clear();
        } else if in_struct && line.trim() == "}" {
            structs.push((current_struct_name.clone(), fields.clone()));
            in_struct = false;
        } else if in_struct {
            let field_parts: Vec<&str> = line.trim().split_whitespace().collect();
            if field_parts.len() == 2 {
                let field_type = field_parts[0].trim().to_string();
                let field_name = field_parts[1].trim().to_string();
                fields.push((field_name, field_type));
            }
        }
    }
    structs
}
