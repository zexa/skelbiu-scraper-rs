#![feature(destructuring_assignment)]

mod common_page_scraper;
mod listing;
mod listing_scraper;
mod listing_stream;
mod page_scraper;
mod potential_listing;
mod scraper;
mod scraper_settings;
mod semaphore_share;
mod semaphore_share_result;

pub use common_page_scraper::CommonPageScraper;
pub use listing::Listing;
pub use listing_scraper::ListingScraper;
pub use page_scraper::PageScraper;
pub use potential_listing::PotentialListing;
pub use scraper::Scraper;
pub use scraper_settings::ScraperSettings;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
