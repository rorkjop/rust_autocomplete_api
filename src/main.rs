use actix_web::http::header::{self, HeaderValue};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TermEntry {
    kommunenummer: u16,
    #[serde(rename = "adresseTekst")]
    adressetekst: String,
}

#[derive(Deserialize)]
struct SearchInfo {
    prefix: String,
    kommunenummer: u16,
}

// Gunzip file at file_path and write to a temporary file and return the path to the temporary file
fn gunzip(file_path: &str) -> NamedTempFile {
    let file = std::fs::File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    let mut decoder = flate2::read::GzDecoder::new(file);
    decoder
        .read_to_end(&mut buffer)
        .expect("Failed to read file");
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    temp_file
        .write_all(&buffer)
        .expect("Failed to write to temporary file");
    temp_file.flush().expect("Failed to flush temporary file");
    temp_file
}

fn load_csv_data(file_path: &str) -> Result<HashMap<u16, Vec<String>>, Box<dyn std::error::Error>> {
    let csv_file = gunzip(file_path);
    let mut rdr = csv::Reader::from_path(csv_file.path())?;
    let mut data: HashMap<u16, Vec<String>> = HashMap::new();
    for result in rdr.deserialize() {
        let record: TermEntry = result?;
        data.entry(record.kommunenummer)
            .or_insert_with(Vec::new)
            .push(record.adressetekst);
    }
    Ok(data)
}

async fn autocomplete(
    search_info: web::Query<SearchInfo>,
    data: web::Data<HashMap<u16, Vec<String>>>,
) -> impl Responder {
    let mut results = Vec::new();
    let prefix = search_info.prefix.to_lowercase();
    let kommunenummer = search_info.kommunenummer.to_owned();

    let start = std::time::Instant::now();

    if let Some(terms) = data.get(&kommunenummer) {
        results = terms
            .iter()
            .filter(|term| term.to_lowercase().starts_with(&prefix))
            .take(10)
            .cloned()
            .collect();
    }

    // Allow content type negotiation with text/plain and application/json
    let accept = "application/json";

    // Respond based on accept
    let mut response = match accept {
        "application/json" => HttpResponse::Ok().json(results),
        _ => HttpResponse::Ok().body(results.join("\n")),
    };
    let headers = response.headers_mut();
    headers.append(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.append(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET"),
    );
    headers.append(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("accept"),
    );
    headers.append(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("Content-Type,Content-Length,Content-Range"),
    );

    println!("Search took: {:?}", start.elapsed());

    return response;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print process ID
    println!("Process ID: {}", std::process::id());

    let start = std::time::Instant::now();
    println!("Loading data...");
    let data = web::Data::new(
        load_csv_data("data/adresser.filtered.csv.gz").expect("Failed to load CSV data"),
    );
    println!("Data loaded in: {:?}", start.elapsed());

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/autocomplete", web::get().to(autocomplete))
    })
    .workers(2)
    .max_connections(500)
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
