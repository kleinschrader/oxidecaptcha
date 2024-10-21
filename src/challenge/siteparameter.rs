#[derive(Debug, Clone)]
pub struct SiteParameter {
    pub difficulty: u8,
    pub prefixes_to_solve: usize,
    pub solution_length: usize
}