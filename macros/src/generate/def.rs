use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct SeaOrmArgs {
    pub table_name: String,
}

#[derive(Debug)]
pub struct GenStructContext{
    pub source_path:String,
    pub target_path:String,
    pub struct_name:String
}