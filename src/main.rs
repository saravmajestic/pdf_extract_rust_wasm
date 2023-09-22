use pdf_extract::extract_text_from_mem;
use reqwest::Error;
use std::time::Instant;

async fn fetch_remote_pdf(url: &str) -> Result<Vec<u8>, Error> {
    let response = reqwest::get(url).await?; // Use '?' to propagate errors

    let bytes = response.bytes().await?;

    // Convert the bytes to a vector
    let byte_vec = bytes.to_vec();

    Ok(byte_vec)
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let pdf_url = "https://www3.weforum.org/docs/WEF_Future_of_Jobs_2020.pdf"; // Replace with your PDF URL
    let pdf_bytes = match fetch_remote_pdf(pdf_url).await {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Failed to fetch PDF: {:?}", err);
            return;
        }
    };

    let text = match std::panic::catch_unwind(move || extract_text_from_mem(&pdf_bytes)) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Error extracting text: {:?}", err);
            return;
        }
    };

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Extracted Text:\n{}", text.unwrap());
}
