pub struct List<T> {
    pub items: Vec<T>,
    pub count: i32,
    pub limit: i32,
    pub offset: i32,
    pub next_offset: Option<i32>,
    pub prev_offset: Option<i32>,
}
