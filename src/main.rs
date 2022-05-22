extern crate dotenv;
mod imap;
mod smtp;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    imap::fetch_inbox_top();
    // smtp::send_test_mail()
}
