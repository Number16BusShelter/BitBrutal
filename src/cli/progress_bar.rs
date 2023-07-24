
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::{cmp::min, fmt::Write};

pub fn get_progress_bar(num_outcomes: usize) -> ProgressBar {
    let pb: ProgressBar = ProgressBar::new(num_outcomes as u64);
    pb.set_style(ProgressStyle::with_template("\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})\n").unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-"));

    return pb;
}