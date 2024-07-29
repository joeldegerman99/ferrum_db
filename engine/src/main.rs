use std::path::Path;

use backend::{node_type::Offset, page::Page, page_layout::PAGE_SIZE, pager::Pager};

fn main() {
    let mut pager = Pager::new(Path::new("testdb")).expect("Failed to create pager");

    // write a page with the only data is a integer valued at 5
    let offset1 = pager.write_page(&Page::new([5; PAGE_SIZE])).expect("Failed to write page");
    let offset2 = pager.write_page(&Page::new([4; PAGE_SIZE])).expect("Failed to write page");


    println!("Offset1: {:?}", offset1);
    println!("Offset2: {:?}", offset2);

    let page = pager.read_page(&Offset(4096)).expect("Failed to read page");

    page.get_data().iter().for_each(|byte| print!("{:02x} ", byte));
}
