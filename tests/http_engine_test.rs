use neodymium::engines::RenderingEngine;
use neodymium::http_engine::HttpEngine;

#[test]
fn test_http_engine_integration() {
    let engine = HttpEngine::new();

    // Test name
    assert_eq!(engine.name(), "HTTP Engine");

    // Test render with a connection error (avoiding real network calls)
    // We use a non-routable address to ensure immediate failure or timeout
    let result = engine.render("http://localhost:1");
    assert!(result.starts_with("Error connecting: "));
}

#[test]
fn test_http_engine_success() {
    let mut server = mockito::Server::new();
    let url = server.url();
    let _m = server
        .mock("GET", "/hello")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body("world")
        .create();

    let engine = HttpEngine::new();
    let result = engine.render(&format!("{}/hello", url));

    assert_eq!(result, "world");
}
