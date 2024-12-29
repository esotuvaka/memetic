use crate::{primitives::TYPE_INFO, Error};

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
        let mut total_size = 0;
        let mut max_alignment = 0;

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

            let field_name = parts[0].trim().to_string();
            let field_type = parts[1].trim().replace(',', "").to_string();

            let type_info = TYPE_INFO
                .get(field_type.as_str())
                .ok_or_else(|| Error::ParseError(format!("Unknown type: {}", field_type)))?;

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
            name,
            fields,
            total_size,
        })
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
                        Err(e) => eprintln!("Error parsing struct: {}\n{}", e, current_struct),
                    }
                    current_struct.clear();
                    is_inside_struct = false;
                }
            }
        }

        Ok(structs)
    }
}
