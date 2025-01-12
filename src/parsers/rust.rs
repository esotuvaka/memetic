use crate::Error;

use super::base::{MetaField, MetaStruct, StructParser};

pub struct RustParser;

impl RustParser {
    pub fn new() -> RustParser {
        RustParser {}
    }
}

impl StructParser for RustParser {
    fn extract(&self, current_struct: &str) -> Result<MetaStruct, crate::Error> {
        // find struct name between 'struct' and '{'
        let name = current_struct
            .lines()
            .find(|line| line.contains("struct"))
            .and_then(|line| {
                line.split("struct")
                    .nth(1)?
                    .split('{')
                    .next()?
                    .trim()
                    .to_string()
                    .into()
            })
            .ok_or_else(|| Error::ParseError("Could not find struct name".to_string()))?;

        let mut fields = Vec::new();

        for line in current_struct.lines() {
            let line = line.trim();
            if line.is_empty()
                || line.starts_with("struct")
                || line.starts_with('{')
                || line.starts_with('}')
                || line.starts_with('#')
                || line.starts_with("//")
            {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }

            let field_name = parts[0].trim().replace("pub ", "").to_string();
            let field_type = parts[1].trim().replace(',', "").to_string();

            fields.push(MetaField {
                field_name,
                field_type,
            });
        }

        Ok(MetaStruct { name, fields })
    }

    fn parse(&self, file_content: String) -> Result<Vec<MetaStruct>, crate::Error> {
        let mut structs = Vec::new();
        let mut current_struct = String::new();
        let mut is_inside_struct = false;
        let mut brace_pair_count = 0;

        for line in file_content.lines() {
            if line.trim().replace("pub", "").starts_with("struct ") {
                is_inside_struct = true;
                brace_pair_count += 1;
            }

            if is_inside_struct {
                current_struct.push_str(line);
                current_struct.push('\n');

                if line.contains('{') {
                    brace_pair_count += line.chars().filter(|&ch| ch == '{').count() - 1;
                }
                if line.contains('}') {
                    brace_pair_count -= line.chars().filter(|&ch| ch == '}').count();
                }

                // Only exit and clear when struct is complete
                if brace_pair_count == 0 {
                    match self.extract(&current_struct) {
                        Ok(ps) => structs.push(ps),
                        Err(e) => eprintln!("parsing struct: {}\n{}", e, current_struct),
                    }
                    current_struct.clear();
                    is_inside_struct = false;
                }
            }
        }

        Ok(structs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_field_struct() {
        let parser = RustParser::new();
        let input = "struct Simple {\n  value: u32\n}";

        let result = parser.extract(input).unwrap();

        assert_eq!(result.name, "Simple");
        assert_eq!(result.fields.len(), 1);
        assert_eq!(result.fields[0].field_name, "value");
        assert_eq!(result.fields[0].field_type, "u32");
    }

    #[test]
    fn test_multiple_fields_struct() {
        let parser = RustParser::new();
        let input = "struct Point {\n    x: i32,\n    y: i32,\n    z: i32\n}";

        let result = parser.extract(input).unwrap();

        assert_eq!(result.name, "Point");
        assert_eq!(result.fields.len(), 3);
    }

    #[test]
    fn test_mixed_types_struct() {
        let parser = RustParser::new();
        let input = "struct Mixed {\n    flag: bool,\n    count: u64,\n    value: f32\n}";

        let result = parser.extract(input).unwrap();

        assert_eq!(result.fields.len(), 3);
    }

    #[test]
    fn test_parse_multiple_structs() {
        let parser = RustParser::new();
        let input = String::from("struct First {\n    x: u8\n}\n\nstruct Second {\n    y: u16\n}");

        let result = parser.parse(input).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "First");
        assert_eq!(result[1].name, "Second");
    }

    #[test]
    fn test_nested_structs() {
        let parser = RustParser::new();
        let input = r#"
            struct Child {
                x: u8,
                y: i32,
            }

            struct Parent {
                name: u32,
                data: f64,
                child: Child,
                numbers: Option<u32>,
            }
        "#;

        let result = parser.parse(input.to_string()).unwrap();

        // Verify Child struct
        let child = &result[0];
        assert_eq!(child.name, "Child");
        assert_eq!(child.fields.len(), 2);
        assert_eq!(child.fields[0].field_type, "u8");
        assert_eq!(child.fields[1].field_type, "i32");

        // Verify Parent struct
        let parent = &result[1];
        assert_eq!(parent.name, "Parent");
        assert_eq!(parent.fields.len(), 4);
        assert_eq!(parent.fields[0].field_type, "u32");
        assert_eq!(parent.fields[1].field_type, "f64");
        assert_eq!(parent.fields[2].field_type, "Child");
        assert_eq!(parent.fields[3].field_type, "Option<u32>");
    }

    #[test]
    fn test_custom_type() {
        let parser = RustParser::new();
        let input = "struct Custom {\n    field: CustomFieldType\n}";

        let result = parser.parse(input.to_string()).unwrap();
        assert_eq!(result[0].name, "Custom");
    }
}
