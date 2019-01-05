#![cfg(feature = "waf-regional")]

extern crate rusoto_core;
extern crate rusoto_waf_regional;

use rusoto_core::Region;
use rusoto_waf_regional::{ListRulesRequest, WAFRegional, WAFRegionalClient};

#[test]
fn should_list_rules() {
    let client = WAFRegionalClient::new(Region::UsEast1);
    let request = ListRulesRequest::default();

    let result = client.list_rules(request).sync().unwrap();
    println!("{:#?}", result);
}
