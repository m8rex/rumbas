use futures_util::future;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use std::io::Error as IoError;
use std::path::Path;

async fn handle_request<B>(req: Request<B>) -> Result<Response<Body>, IoError> {
    let root = Path::new(rumbas::OUTPUT_FOLDER); //.join("");

    // First, resolve the request. Returns a future for a `ResolveResult`.
    let result = hyper_staticfile::resolve(&root, &req)
        .await
        .unwrap();
    println!("{} {:?}",  req.uri().path(), result);
    if let hyper_staticfile::ResolveResult::IsDirectory = result {
        let real_path = format!("{}{}", rumbas::OUTPUT_FOLDER, req.uri().path());
        println!("Handling {}", real_path);

        let index_path = format!("{}/index.html", real_path);
        let index_path = std::path::Path::new(&index_path[..]);
        if !index_path.exists(){
        let items = read_folder(&real_path[..]);
        let file = format!(r"
<html>
    <head>
    </head>
    <body>
        <ul>
            {}
        </ul>
    </body>
</html>
", items.into_iter().map(|i| format!(r#"<li><a href="{}">{}</a></li>"#, i.file_name().unwrap().to_str().unwrap(), i.file_name().unwrap().to_str().unwrap())).collect::<Vec<_>>().join("\n"));
        std::fs::write(index_path, file).expect("valid file"); // TODO
            }
    }

    // Then, build a response based on the result.
    // The `ResponseBuilder` is typically a short-lived, per-request instance.
    let response = hyper_staticfile::ResponseBuilder::new()
        .request(&req)
        .build(result)
        .unwrap();
    Ok(response)
}

fn read_folder(path: &str)  -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
     for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap(); // TODO
            let path = entry.path();
            if path.is_dir() {
                paths.push(path.to_path_buf());
            } 
    }
    paths
}

#[tokio::main]
pub async fn serve(_matches: &clap::ArgMatches) {
    let make_service = make_service_fn(|_| {
        future::ok::<_, hyper::Error>(service_fn(handle_request))
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = hyper::Server::bind(&addr).serve(make_service);
    println!("Serving rumbas output on http://{}/", addr);
    server.await.expect("Failied serving rumbas output");
}

