use thrust_lsp::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    println!("starting up");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}

