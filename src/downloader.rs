use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

use actix_http::http::header::{self, ContentDisposition};
use anyhow::{anyhow, Result};
use percent_encoding::percent_decode_str;
use reqwest::{blocking::Response, Url};

use crate::indicator::{create_progress_bar, FILE_DOWNLOAD_SUCCESS, FILE_NAME_TEMPLATE};

pub fn download_file(url: Url, file_name: Option<String>) -> Result<()> {
    let mut response = reqwest::blocking::get(url)?;
    let file_name = file_name
        .or_else(|| find_filename(&response))
        .ok_or_else(|| {
            anyhow!("Unable to detect filename automatically, provide filename manually")
        })?;

    let pb = create_progress_bar(response.content_length());
    pb.println(FILE_NAME_TEMPLATE.replace("{}", &file_name));

    let mut file = BufWriter::new(File::options().append(true).create(true).open(&file_name)?);
    let mut buffer = vec![0u8; 128 * 1024];
    loop {
        let n = response.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer)?;
        pb.inc(n as u64);
    }
    pb.println(FILE_DOWNLOAD_SUCCESS);

    Ok(())
}

fn find_filename(response: &Response) -> Option<String> {
    if let Some(content_deposition) = response
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .and_then(|hv| ContentDisposition::from_raw(hv).ok())
    {
        if let Some(ext_filename) = content_deposition
            .get_filename_ext()
            .and_then(|ext_val| String::from_utf8(ext_val.value.clone()).ok())
        {
            return Some(ext_filename);
        } else if let Some(file_name) = content_deposition.get_filename() {
            return Some(file_name.to_owned());
        }
    }

    response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|encoded_str| {
            percent_decode_str(encoded_str)
                .decode_utf8()
                .map(|cow| cow.into_owned())
                .ok()
        })
}
