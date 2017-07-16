extern crate factordb;

#[cfg(test)]
extern crate mockito;

use mockito::mock;

#[test]
fn test_factordb() {
    let _m = mock("GET", "/?query=8190")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"8190","status":"FF","factors":[["2",1],["3",2],["5",1],["7",1],["13",1]]}"#)
        .create();

    let result = factordb::request(8190).unwrap();

    assert_eq!(
        result.factor_list(),
        vec![2, 3, 3, 5, 7, 13]
    );

    assert_eq!(
        result.json(),
        r#"{"id":"8190","status":"FF","factors":[["2",1],["3",2],["5",1],["7",1],["13",1]]}"#
    );
}
