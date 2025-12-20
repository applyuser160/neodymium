use neodymium::engines::{HttpEngine, RenderingEngine};

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
