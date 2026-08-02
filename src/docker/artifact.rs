use reqwest::{
    StatusCode,
    blocking::Client,
    header::{ACCEPT_ENCODING, CONTENT_RANGE, RANGE},
};
use std::{
    error::Error,
    fs::File,
    io::{Seek, SeekFrom, Write},
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

// One thread per chunk, so chunks must not be tiny.
const CHUNK_SIZE: u64 = 64 * 1024 * 1024; // 64 MiB

pub fn get_artifact(deployment_type: &str, version: &str) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();

    let url =
        format!("https://bcartifacts-exdbf9fwegejdqak.b02.azurefd.net/sandbox/25.5.30849.50175/us");

    let client = Client::new();

    let response = client
        .head(&url)
        .header(ACCEPT_ENCODING, "identity")
        .send()?
        .error_for_status()?;

    let total_size = response
        .content_length()
        .ok_or("Content-Length header missing")?;

    println!("File size: {total_size} bytes");

    let file = File::create("./artifact.zip")?;
    file.set_len(total_size)?;

    let file = Arc::new(Mutex::new(file));
    let mut handles = Vec::new();

    for start in (0..total_size).step_by(CHUNK_SIZE as usize) {
        let end = (start + CHUNK_SIZE - 1).min(total_size - 1);
        let expected_length = end - start + 1;

        let client = client.clone();
        let file = Arc::clone(&file);
        let url = url.clone();

        let handle = thread::spawn(move || -> Result<(), String> {
            let response = client
                .get(&url)
                .header(RANGE, format!("bytes={start}-{end}"))
                .header(ACCEPT_ENCODING, "identity")
                .send()
                .map_err(|error| format!("Request for {start}-{end} failed: {error}"))?;

            if response.status() != StatusCode::PARTIAL_CONTENT {
                return Err(format!(
                    "Expected 206 Partial Content for {start}-{end}, got {}",
                    response.status()
                ));
            }

            let expected_content_range = format!("bytes {start}-{end}/{total_size}");

            let actual_content_range = response
                .headers()
                .get(CONTENT_RANGE)
                .ok_or_else(|| format!("Content-Range missing for {start}-{end}"))?
                .to_str()
                .map_err(|error| format!("Invalid Content-Range: {error}"))?;

            if actual_content_range != expected_content_range {
                return Err(format!(
                    "Wrong Content-Range: expected \
                     '{expected_content_range}', got \
                     '{actual_content_range}'"
                ));
            }

            let content = response
                .bytes()
                .map_err(|error| format!("Reading {start}-{end} failed: {error}"))?;

            if content.len() as u64 != expected_length {
                return Err(format!(
                    "Incomplete chunk {start}-{end}: expected \
                     {expected_length} bytes, received {}",
                    content.len()
                ));
            }

            let mut file = file
                .lock()
                .map_err(|error| format!("File lock failed: {error}"))?;

            file.seek(SeekFrom::Start(start))
                .map_err(|error| format!("Seek failed: {error}"))?;

            file.write_all(&content)
                .map_err(|error| format!("Write failed: {error}"))?;

            println!("Downloaded bytes {start}-{end}");

            Ok(())
        });

        handles.push(handle);
    }

    // This part was missing.
    for handle in handles {
        match handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(error)) => return Err(error.into()),
            Err(_) => return Err("A download thread panicked".into()),
        }
    }

    file.lock()
        .map_err(|error| format!("File lock failed: {error}"))?
        .sync_all()?;

    println!("Download completed in {:?}", started.elapsed());

    Ok(())
}
