mod client;
//mod locale;
mod search_interest;
mod utils;
mod region_interest;
mod related_topics;
mod related_queries;
mod request_builder;

use crate::client::*;
use crate::region_interest::RegionInterest;
use crate::related_queries::RelatedQueries;
use crate::related_topics::RelatedTopics;
use crate::search_interest::*;


//struct RegionInterest {}
//struct RelatedTopics {}
//struct RelatedQueries {}

fn main() {
    let my_cookie = "NID=218=WdklPXCK9nRe7pPNLmdGZ4w_2cfOsRdeD_ESaCAeDB_0MqJrCqDYOll55y_quoQ8I3RQjQmssffaOcQNj3iMhqK3Wep8DOdcJW93-ZxN6Y87Zg5Z_0Ifw7_X4tVYkvdkJoFL9-QfStcvqlGH5wPiBe0ApgskqQllrqtxGK4mEXk";
    let keywords = "baguette";
    let country = "FR";
    let lang = "fr";
    //let start_date: Date<Utc> = Utc.ymd(2014, 7, 8);
    //let end_date: Date<Utc> = Utc.ymd(2020, 12, 20);
    //let time_filter = ""; // 1-H
    //SearchInterest::with_period(google_client.clone(), start_date, end_date); // Need to recompute a token -> Changing date = new token to auth

    let google_client = Client::new(my_cookie, keywords, lang, country);

    let search_interest = SearchInterest::new(google_client.clone());
    //println!("{}", _search_interest);
//
    //let _region_interest = RegionInterest::new(google_client.clone()).get().unwrap();
    //println!("{}", _region_interest);
//
    //let _related_topics = RelatedTopics::new(google_client.clone()).get().unwrap();
    //println!("{}", _related_topics);
//
    //let _related_queries = RelatedQueries::new(google_client.clone()).get().unwrap();
    //println!("{}", _related_queries);

    SearchInterest::get_impl(&search_interest);
}
