use indicatif::{ProgressBar, ProgressStyle};

pub const FILE_NAME_TEMPLATE: &str = "\
📂 File       = {}";

pub const FILE_DOWNLOAD_SUCCESS: &str = "\
Download success 👍";

const PROGRESS_BAR_TEMPLATE: &str = "\
🕒 Elapsed    = {elapsed_precise}
📝 Downloaded = {bytes} / {total_bytes}
💨 Speed      = {bytes_per_sec}
⏳ ETA        = {eta:.bold}
{percent:>3}% {bar:40}
";

const PROGRESS_SPINNER_TEMPLATE: &str = "\
🕒 Elapsed    = {elapsed_precise}
📝 Downloaded = {bytes} / ?
💨 Speed      = {bytes_per_sec}
";

pub fn create_progress_bar(content_size: Option<u64>) -> ProgressBar {
    if let Some(content_size) = content_size {
        ProgressBar::new(content_size)
            .with_style(ProgressStyle::default_bar().template(PROGRESS_BAR_TEMPLATE))
    } else {
        ProgressBar::new_spinner()
            .with_style(ProgressStyle::default_spinner().template(PROGRESS_SPINNER_TEMPLATE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, prelude::Distribution};
    use std::{cmp::min, thread, time::Duration};

    #[test]
    fn test_progress_bar() {
        let mut rng = rand::thread_rng();
        let distribution = Uniform::new(0, 500);

        let file_size = 200 * 1024;
        for content_size in [Some(file_size), None] {
            let pb = create_progress_bar(content_size);
            pb.println(FILE_NAME_TEMPLATE.replace("{}", "temp-file"));
            distribution
                .sample_iter(&mut rng)
                .scan(0, |accu, x: u64| {
                    if *accu >= file_size {
                        None
                    } else {
                        let x = min(x, file_size - *accu);
                        *accu += x;
                        Some(x)
                    }
                })
                .for_each(|x| {
                    thread::sleep(Duration::from_millis(10));
                    pb.inc(x);
                });
            pb.println(FILE_DOWNLOAD_SUCCESS);
            pb.finish_and_clear();
        }
    }
}
