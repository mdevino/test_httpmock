use httpmock::MockServer;

fn main() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET").path("/health");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"status: "ok"}"#);
    });

    let response = reqwest::blocking::get(server.url("/health")).unwrap();

    // Ensure mock was called
    mock.assert();

    println!("{response:#?}");
}
