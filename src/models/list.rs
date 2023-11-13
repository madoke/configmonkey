pub struct List<T> {
    pub items: Vec<T>,
    pub count: i32,
    pub limit: i32,
    pub offset: i32,
    pub next_offset: Option<i32>,
    pub prev_offset: Option<i32>,
}

impl<T> List<T> {
    pub fn from_items(items: Vec<T>, limit: i32, offset: i32) -> List<T> {
        let count = items.len() as i32;
        List {
            items,
            count,
            limit,
            offset,
            next_offset: if count == limit {
                Some(offset + limit)
            } else {
                None
            },
            prev_offset: if offset == 0 {
                None
            } else if offset - limit >= 0 {
                Some(offset - limit)
            } else {
                Some(0)
            },
        }
    }
}
