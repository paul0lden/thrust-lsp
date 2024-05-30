use thrust_lsp::Backend;
use tower_service::Service;
use tower_lsp::jsonrpc::Request;
use tower_lsp::LspService;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_initialize() {
    let (mut service, _messages) = LspService::new(|client| Backend { client });

    let initialize_params = serde_json::json!({
        "processId": 12345,
        "clientInfo": {
            "name": "test_client",
            "version": "1.0.0"
        },
        "rootUri": "file:///test/root",
        "capabilities": {},
        "workspaceFolders": null
    });

    let initialize_request = Request::build("initialize")
        .params(initialize_params)
        .id(1)
        .finish();

    let response = service.call(initialize_request).await.unwrap();

    println!("Response: {:?}", response);

    let binding = response.expect("result");

    let result = binding.result().expect("Expected a susccesful response");

    println!("Result: {:?}", result);

    if let Some(capabilities) = result.get("capabilities") {
        assert!(capabilities.is_object());
    } else {
        panic!("error");
    }

    sleep(Duration::from_secs(1)).await;
}
