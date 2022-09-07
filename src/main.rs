use salvo::{
    extra::compression,
    extra::serve_static::{DirHandler, FileHandler},
    prelude::*,
};

#[handler]
async fn hello_world() -> &'static str {
    "Hello, World"
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(
            Router::new()
                .path("compressed")
                .hoop(compression::brotli())
                .get(FileHandler::new("public/index.html")),
        )
        .push(
            Router::new()
                .path("public/<*path>")
                .get(DirHandler::new("public")),
        )
        .push(Router::new().path("hello").get(hello_world))
        .push(
            Router::new()
                .path("/<*path>")
                .get(DirHandler::new("hello-world/build")),
        );
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
