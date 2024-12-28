use crate::{primitives::TYPE_INFO, Error};

#[derive(Debug)]
pub struct StructFinder {}

#[derive(Debug)]
pub struct MetaField {
    field_name: String,
    field_type: String,
    size: u8,
    alignment: u8,
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

    fn parse_struct(&self, struct_str: &str) -> Result<MetaStruct, Error> {
        // Find struct name between "struct" and "{"
        let struct_name = struct_str
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

        // Rest of parsing logic for fields
        let mut fields = Vec::new();
        let mut total_size = 0;
        let mut max_alignment = 0;

        for line in struct_str.lines() {
            let line = line.trim();
            if line.is_empty()
                || line.starts_with("struct")
                || line.starts_with('{')    // struct open 
                || line.starts_with('}')    // struct close
                || line.starts_with('#')    // derive macros
                || line.starts_with("//")
            // comments + docs
            {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }

            let field_name = parts[0].trim().to_string();
            let field_type = parts[1].trim().replace(',', "").to_string();

            let type_info = TYPE_INFO
                .get(field_type.as_str())
                .ok_or_else(|| Error::ParseError(format!("Unkown type: {}", field_type)))?;

            max_alignment = max_alignment.max(type_info.nat_align);
            total_size += type_info.size;

            fields.push(MetaField {
                field_name,
                field_type,
                size: type_info.size,
                alignment: type_info.nat_align,
            });
        }

        Ok(MetaStruct {
            name: struct_name,
            fields,
        })
    }

    pub fn parse(&self, file_content: String) -> Result<Vec<MetaStruct>, Error> {
        let mut structs = Vec::new();
        let mut struct_str = String::new();
        let mut is_inside_struct = false;
        let mut brace_pair_count = 0;

        for line in file_content.lines() {
            // Handle struct definition start
            if line.trim().replace("pub", "").starts_with("struct ") {
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
    fn test_parse_cursed() {
        let finder = StructFinder::new();
        let file_content = r#"
            struct   Test1    {
        field1: u32,
              }

        struct   Test2 {

                field2: String,
            }
        // TEST COMMENT
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
