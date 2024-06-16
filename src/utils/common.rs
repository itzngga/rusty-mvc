pub fn calculate_total_pages(total_data: i32, limit_per_page: i32) -> i32 {
    let total_pages = (total_data as f64 / limit_per_page as f64).ceil() as i32;
    std::cmp::max(total_pages, 1)
}
