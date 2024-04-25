use serde::{Deserialize, Serialize};

pub fn empty_string_as_none<'r, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'r>,
{
    let s = Option::<String>::deserialize(de)?;

    let s = s.filter(|s| !s.is_empty());

    Ok(s)
}

#[derive(Deserialize, Debug)]
pub struct PagedQuery {
    pub page: u32,
    pub take: u16,
}

#[derive(Serialize, Debug)]
pub struct PaginatedReponse<T> {
    pub total: u32,
    pub page: u32,
    pub data: Vec<T>,
}
