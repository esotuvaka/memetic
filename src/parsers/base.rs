use crate::Error;

#[derive(Debug)]
pub struct MetaField {
    pub field_name: String,
    pub field_type: String,
    pub size: u8,
    pub alignment: u8,
}

#[derive(Debug)]
pub struct MetaStruct {
    pub name: String,
    pub fields: Vec<MetaField>,
    pub total_size: u8,
}

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
}

pub trait StructParser {
    fn extract(&self, current_struct: &str) -> Result<MetaStruct, Error>;
    fn parse(&self, file_content: String) -> Result<Vec<MetaStruct>, Error>;
}

pub fn parse<T>(parser: &T, file_content: String) -> Result<Vec<MetaStruct>, Error>
where
    T: StructParser,
{
    parser.parse(file_content)
}
