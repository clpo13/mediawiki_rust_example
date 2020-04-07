fn main() {
    // select API endpoint
    let api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php").unwrap();

    // query parameters
    let params = api.params_into(&[
        ("action", "query"),
        ("prop", "categories"),
        ("titles", "Albert Einstein"),
        ("cllimit", "500"),
    ]);

    // run query
    let res = api.get_query_api_json_all(&params).unwrap();

    // parse result
    let categories: Vec<&str> = res["query"]["pages"]
        .as_object()
        .unwrap()
        .iter()
        .flat_map(|(_page_id, page)| {
            page["categories"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c["title"].as_str().unwrap())
        })
        .collect();

    dbg!(&categories);
}
