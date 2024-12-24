use crate::Error;

#[derive(Debug)]
pub struct StructFinder {}

#[derive(Debug)]
pub struct MetaField {
    field_name: String,
    field_type: String,
}

#[derive(Debug)]
pub struct MetaStruct {
    name: String,
    fields: Vec<MetaField>,
    // original_size: u8,
}

impl StructFinder {
    pub fn new() -> StructFinder {
        StructFinder {}
    }

    fn parse_struct(&self, input: &str) -> Result<MetaStruct, Error> {
        if !input.trim().starts_with("struct ") {
            panic!("invalid struct shape: does not start with 'struct '");
        }

        let struct_name_end = input.find('{').unwrap();
        let struct_name = &input[..struct_name_end].trim().replace("struct ", ""); // skip "struct "

        let mut fields = Vec::new();
        let fields_region = &input[struct_name_end..];

        // process the fields, handling presence/lack of formatting
        let mut field_str = String::new();
        let mut is_inside_braces = false;
        let mut field_lines = Vec::new();
        for char in fields_region.chars() {
            if char == '{' {
                is_inside_braces = true;
            } else if char == '}' {
                break;
            } else if is_inside_braces {
                field_str.push(char);
            }
        }

        // split the field block into individual lines
        for field_line in field_str.split(',') {
            if !field_line.trim().is_empty() {
                field_lines.push(field_line)
            }
        }

        // extract the name and data type
        for line in field_lines {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                panic!("invalid struct shape: not 'field_name: field_type'");
            }
            let field_name = parts[0].trim().to_string();
            let field_type = parts[1].trim().to_string();
            fields.push(MetaField {
                field_name,
                field_type,
            })
        }

        Ok(MetaStruct {
            name: struct_name.to_string(),
            fields,
        })
    }

    pub fn parse(&self, file_content: String) -> Result<Vec<MetaStruct>, Error> {
        let mut structs = Vec::new();
        let mut struct_str = String::new();
        let mut is_inside_struct = false;
        let mut brace_pair_count = 0;

        for line in file_content.lines() {
            dbg!(&struct_str);

            // Handle struct definition start
            if line.trim().starts_with("struct ") {
                is_inside_struct = true;
                brace_pair_count += 1;
            }

            // Track everything inside struct
            if is_inside_struct {
                struct_str.push_str(line);
                struct_str.push('\n');

                // Update brace count after adding line
                if line.contains('{') {
                    brace_pair_count += line.chars().filter(|&ch| ch == '{').count() - 1;
                    // -1 because we counted the first one
                }
                if line.contains('}') {
                    brace_pair_count -= line.chars().filter(|&ch| ch == '}').count();
                }

                // Only exit and clear when struct is complete
                if brace_pair_count == 0 {
                    match self.parse_struct(&struct_str) {
                        Ok(ps) => structs.push(ps),
                        Err(e) => eprintln!("Error parsing struct: {}\n{}", e, struct_str),
                    }
                    struct_str.clear();
                    is_inside_struct = false;
                }
            }
        }

        dbg!(&structs);
        Ok(structs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_struct() {
        let finder = StructFinder::new();
        let file_content = r#"
            struct Test {
                field1: u32,
                field2: String,
            }
        "#
        .to_string();

        let result = finder.parse(file_content).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Test");
        assert_eq!(result[0].fields.len(), 2);
        assert_eq!(result[0].fields[0].field_name, "field1");
        assert_eq!(result[0].fields[0].field_type, "u32");
        assert_eq!(result[0].fields[1].field_name, "field2");
        assert_eq!(result[0].fields[1].field_type, "String");
    }

    #[test]
    fn test_parse_multiple_structs() {
        let finder = StructFinder::new();
        let file_content = r#"
            struct Test1 {
                field1: u32,
            }

            struct Test2 {
                field2: String,
            }
        "#
        .to_string();

        let result = finder.parse(file_content).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Test1");
        assert_eq!(result[0].fields.len(), 1);
        assert_eq!(result[0].fields[0].field_name, "field1");
        assert_eq!(result[0].fields[0].field_type, "u32");

        assert_eq!(result[1].name, "Test2");
        assert_eq!(result[1].fields.len(), 1);
        assert_eq!(result[1].fields[0].field_name, "field2");
        assert_eq!(result[1].fields[0].field_type, "String");
    }

    #[test]
    fn test_parse_nested_structs() {
        let finder = StructFinder::new();
        let file_content = r#"
            struct Outer {
                inner: Inner,
            }

            struct Inner {
                field: u32,
            }
        "#
        .to_string();

        let result = finder.parse(file_content).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Outer");
        assert_eq!(result[0].fields.len(), 1);
        assert_eq!(result[0].fields[0].field_name, "inner");
        assert_eq!(result[0].fields[0].field_type, "Inner");

        assert_eq!(result[1].name, "Inner");
        assert_eq!(result[1].fields.len(), 1);
        assert_eq!(result[1].fields[0].field_name, "field");
        assert_eq!(result[1].fields[0].field_type, "u32");
    }
}
