extern crate hyper;
use self::hyper::Client;
use self::hyper::header::{Headers, UserAgent};


use std::io::Read;

extern crate select;
use self::select::document::Document;
use self::select::predicate::{Attr};

pub fn open_nhnl_web() -> String {

    let mut headers = Headers::new();
    headers.set(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36".to_owned()));

    let url = "https://www.barchart.com/stocks/highs-lows/summary#/viewName=main";
    let client = Client::new();

    let mut response = client
        .get(url)
        .headers(headers)
        .send()
        .unwrap();

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    body
}

pub fn scrape<'a>(html: &'a str) -> [(&str, i32); 4] {

    let document = Document::from(html);

    let mut high_nyse: i32 = -1;
    let high_nyse_href = "/stocks/highs-lows/highs#/screener=nyse&timeFrame=3m";
    for node in document.find(Attr("href", high_nyse_href)).iter() {
        let text = node.text();
        let to_cast = text.trim_matches(' ');
        high_nyse = to_cast.parse::<i32>().unwrap();
    }

    let mut high_nasdaq: i32 = -1;
    let high_nasdaq_href = "/stocks/highs-lows/highs#/screener=nasdaq&timeFrame=3m";
    for node in document.find(Attr("href", high_nasdaq_href)).iter() {
        let text = node.text();
        let to_cast = text.trim_matches(' ');
        high_nasdaq = to_cast.parse::<i32>().unwrap();
    }


    let mut low_nyse: i32 = -1;
    let low_nyse_href = "/stocks/highs-lows/lows#/screener=nyse&timeFrame=3m";
    for node in document.find(Attr("href", low_nyse_href)).iter() {
        let text = node.text();
        let to_cast = text.trim_matches(' ');
        low_nyse = to_cast.parse::<i32>().unwrap();
    }

    let mut low_nasdaq: i32 = -1;
    let low_nasdaq_href = "/stocks/highs-lows/lows#/screener=nasdaq&timeFrame=3m";
    for node in document.find(Attr("href", low_nasdaq_href)).iter() {
        let text = node.text();
        let to_cast = text.trim_matches(' ');
        low_nasdaq = to_cast.parse::<i32>().unwrap();
    }

    [
        ("High NYSE", high_nyse),
        ("High Nasdaq", high_nasdaq),
        ("Low NYSE", low_nyse),
        ("Low Nasdaq", low_nasdaq)
    ]
}

pub fn check_result(input: [(&str, i32); 4]) -> Vec<&str> {

    let mut res = Vec::new();
    for field in input.iter() {
        let &(val, i) = field;

        if i == -1 {
            res.push(val);
        }
    }

    res
}


#[test]
fn it_scrapes() {

    let body = r#"
        <a href="/stocks/highs-lows/lows#/screener=nyse&timeFrame=3m"> 228 </a>
        <a href="/stocks/highs-lows/highs#/screener=nyse&timeFrame=3m"> 64 </a>
        <a href="/stocks/highs-lows/highs#/screener=nasdaq&timeFrame=3m"> 75 </a>
        <a href="/stocks/highs-lows/lows#/screener=nasdaq&timeFrame=3m"> 323 </a>
    "#;

    let res = scrape(body);

    assert_eq!(res, [
        ("High NYSE", 64),
        ("High Nasdaq", 75),
        ("Low NYSE", 228),
        ("Low Nasdaq", 323)
    ]);
}

#[test]
fn it_detects_scrape_errors() {

    let input_with_error = [
        ("High NYSE", 64),
        ("High Nasdaq", -1),
        ("Low NYSE", 228),
        ("Low Nasdaq", 323)
    ];

    assert_eq!(check_result(input_with_error), ["High Nasdaq"]);
}

#[test]
fn it_doesnt_collect_if_all_is_well() {

    let input_without_error = [
        ("High NYSE", 64),
        ("High Nasdaq", 1),
        ("Low NYSE", 228),
        ("Low Nasdaq", 323)
    ];

    // let vector: Vec<&str> = vec![];
    // short: Vec::<&str>::new()
    assert_eq!(check_result(input_without_error), Vec::<&str>::new());
}
