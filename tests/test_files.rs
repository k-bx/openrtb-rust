extern crate openrtb;
extern crate serde_json;

use openrtb::v2_5::{AuctionType, BidRequest, BidResponse};
use serde_json::json;

#[test]
fn brandscreen_request_mobile() {
    let test_data = include_str!("files/brandscreen/request-mobile.json");
    let request = test_v2_5_request_serialization_round_trip(test_data);

    // Test some assorted values.
    assert_eq!(request.id, "IxexyLDIIk");
    assert_eq!(request.auction_type, AuctionType::SecondPricePlus);
    assert_eq!(request.user.is_some(), true);
    assert_eq!(request.all_imps, false);
    assert_eq!(request.imp.len(), 1);
    assert_eq!(request.imp[0].id, "1");
}

#[test]
fn brandscreen_request_pc_multi() {
    let test_data = include_str!("files/brandscreen/request-pc-multi.json");
    let request = test_v2_5_request_serialization_round_trip(test_data);

    // Test some assorted values.
    assert_eq!(request.id, "8652a8680db33faabbf3fa76150f35df50a67060");
    assert_eq!(request.auction_type, AuctionType::FirstPrice);
    assert_eq!(request.user.is_some(), true);
    assert_eq!(request.imp.len(), 2);
    assert_eq!(request.imp[0].id, "121-dt1");
    assert_eq!(request.imp[1].id, "121-dt2");
}

#[test]
fn brandscreen_request_pc_single() {
    let test_data = include_str!("files/brandscreen/request-pc-single.json");
    let request = test_v2_5_request_serialization_round_trip(test_data);

    // Test some assorted values.
    assert_eq!(request.id, "80ce30c53c16e6ede735f123ef6e32361bfc7b22");
    assert_eq!(request.auction_type, AuctionType::FirstPrice);
    assert_eq!(request.user.is_some(), true);
    assert_eq!(
        request.user.unwrap().id,
        Some("55816b39711f9b5acf3b90e313ed29e51665623f".to_string())
    );
    assert_eq!(request.currency.len(), 1);
    assert_eq!(request.currency[0], "USD");
}

#[test]
fn brandscreen_response_mobile() {
    let test_data = include_str!("files/brandscreen/response-mobile.json");
    let response = test_v2_5_response_serialization_round_trip(test_data);

    // Test some assorted values.
    assert_eq!(response.id, "IxexyLDIIk");
    assert_eq!(response.seat_bid.len(), 1);
    assert_eq!(response.seat_bid[0].seat, Some("2".to_string()));
    assert_eq!(response.currency, Some("USD".to_string()));
}

#[test]
fn brandscreen_response_pc_multi() {
    let test_data = include_str!("files/brandscreen/response-pc-multi.json");
    let response = test_v2_5_response_serialization_round_trip(test_data);

    // Test some assorted values.
    assert_eq!(
        response.id,
        "BID-4-ZIMP-4b309eae-504a-4252-a8a8-4c8ceee9791a"
    );
    assert_eq!(response.seat_bid.len(), 2);
    assert_eq!(response.seat_bid[0].seat, Some("42".to_string()));
    assert_eq!(response.seat_bid[1].seat, Some("772".to_string()));
    assert_eq!(response.currency, Some("USD".to_string()));
}

#[test]
fn brandscreen_response_pc_win_notifadm() {
    let test_data = include_str!("files/brandscreen/response-pc-win-notifadm.json");
    let response = test_v2_5_response_serialization_round_trip(test_data);

    // Test some assorted values.
    assert_eq!(
        response.id,
        "BID-4-ZIMP-4b309eae-504a-4252-a8a8-4c8ceee9791a"
    );
    assert_eq!(response.seat_bid.len(), 1);
    assert_eq!(response.seat_bid[0].seat, Some("772".to_string()));
    assert_eq!(response.currency, Some("USD".to_string()));
}

#[test]
fn general_files() {
    test_v2_5_request_serialization_round_trip(include_str!(
        "files/rubiconproject/request-app-android-1.json"
    ));
    test_v2_5_request_serialization_round_trip(include_str!(
        "files/rubiconproject/request-app-android-2.json"
    ));
    test_v2_5_request_serialization_round_trip(include_str!(
        "files/rubiconproject/request-web-ie8.json"
    ));
    test_v2_5_request_serialization_round_trip(include_str!(
        "files/rubiconproject/request-web-iphone.json"
    ));
    test_v2_5_request_serialization_round_trip(include_str!(
        "files/rubiconproject/request-web-safari.json"
    ));

    test_v2_5_request_serialization_round_trip(include_str!(
        "files/spotxchange/request-multiple-impr.json"
    ));
    test_v2_5_request_serialization_round_trip(include_str!(
        "files/spotxchange/request-single-impr.json"
    ));
    test_v2_5_response_serialization_round_trip(include_str!(
        "files/spotxchange/response-multiple-vast-inline-simple.json"
    ));
    test_v2_5_response_serialization_round_trip(include_str!(
        "files/spotxchange/response-single-vast-inline-simple.json"
    ));
}

#[test]
fn general_files_failures() {
    assert!(serde_json::from_str::<BidRequest>(include_str!("files/invalid/empty.json")).is_err());
    assert!(serde_json::from_str::<BidResponse>(include_str!("files/invalid/empty.json")).is_err());
    assert!(serde_json::from_str::<BidRequest>(include_str!(
        "files/invalid/all-imps-too-big.json"
    ))
    .is_err());
    assert!(serde_json::from_str::<BidRequest>(include_str!(
        "files/invalid/invalid-currency.json"
    ))
    .is_err());
}

fn test_v2_5_request_serialization_round_trip(data: &str) -> BidRequest {
    let request = match serde_json::from_str(data) {
        Ok(request) => request,
        Err(e) => {
            println!("Failed: {}", e);
            assert!(false);
            panic!();
        }
    };

    // Test that serializing and deserializing give same results. Exterior tests will test some random values.
    let json = json!(request).to_string();
    let deserialized: BidRequest = match serde_json::from_str(json.as_str()) {
        Ok(request) => request,
        Err(e) => {
            println!("Failed: {}", e);
            panic!();
        }
    };

    assert_eq!(deserialized, request);

    request
}

fn test_v2_5_response_serialization_round_trip(data: &str) -> BidResponse {
    let request = match serde_json::from_str(data) {
        Ok(request) => request,
        Err(e) => {
            println!("Failed: {}", e);
            assert!(false);
            panic!();
        }
    };

    // Test that serializing and deserializing give same results. Exterior tests will test some random values.
    let json = json!(request).to_string();
    let deserialized: BidResponse = match serde_json::from_str(json.as_str()) {
        Ok(request) => request,
        Err(e) => {
            println!("Failed: {}", e);
            panic!();
        }
    };

    assert_eq!(deserialized, request);

    request
}
