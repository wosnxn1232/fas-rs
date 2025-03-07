// Copyright 2024-2025, dependabot[bot], reigadegr, shadow3aaa
//
// This file is part of fas-rs.
//
// fas-rs is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// fas-rs is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along
// with fas-rs. If not, see <https://www.gnu.org/licenses/>.

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use log::warn;

use super::IGNORE_MAP;
use crate::file_handler::FileHandler;

#[derive(Debug)]
pub struct Info {
    pub policy: i32,
    path: PathBuf,
    pub cur_fas_freq: isize,
    pub freqs: Vec<isize>,
    verify_freq: Option<isize>,
    verify_timer: Instant,
}

impl Info {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("Invalid file name")?;
        let policy_str = file_name.get(6..).context("Invalid policy format")?;
        let policy = policy_str
            .parse::<i32>()
            .context("Failed to parse policy")?;

        let freqs_content = fs::read_to_string(path.join("scaling_available_frequencies"))
            .context("Failed to read frequencies")?;
        let mut freqs: Vec<isize> = freqs_content
            .split_whitespace()
            .map(|f| f.parse::<isize>().context("Failed to parse frequency"))
            .collect::<Result<_>>()?;
        freqs.sort_unstable();

        Ok(Self {
            policy,
            path,
            cur_fas_freq: *freqs.last().context("No frequencies available")?,
            freqs,
            verify_freq: None,
            verify_timer: Instant::now(),
        })
    }

    pub fn write_freq(&mut self, freq: isize, file_handler: &mut FileHandler) -> Result<()> {
        if self.verify_timer.elapsed() >= Duration::from_secs(3) {
            self.verify_timer = Instant::now();

            if let Some(verify_freq) = self.verify_freq {
                let current_freq = self.read_freq();
                let min_acceptable_freq = self
                    .freqs
                    .iter()
                    .take_while(|freq| **freq < current_freq)
                    .last()
                    .copied()
                    .unwrap_or(current_freq);
                let max_acceptable_freq = self
                    .freqs
                    .iter()
                    .find(|freq| **freq > current_freq)
                    .copied()
                    .unwrap_or(current_freq);
                if !(min_acceptable_freq..=max_acceptable_freq).contains(&verify_freq) {
                    warn!(
                        "CPU Policy{}: Frequency control does not meet expectations! Expected: {}-{}, Actual: {}",
                        self.policy, min_acceptable_freq, max_acceptable_freq, verify_freq
                    );
                }
            }
        }

        let min_freq = *self.freqs.first().context("No frequencies available")?;
        let max_freq = *self.freqs.last().context("No frequencies available")?;

        let adjusted_freq = freq.clamp(min_freq, max_freq);
        self.verify_freq = Some(adjusted_freq);
        self.cur_fas_freq = adjusted_freq;
        let adjusted_freq = adjusted_freq.to_string();

        if !IGNORE_MAP
            .get()
            .context("IGNORE_MAP not initialized")?
            .get(&self.policy)
            .context("Policy ignore flag not found")?
            .load(Ordering::Acquire)
        {
            file_handler.write_with_workround(self.max_freq_path(), &adjusted_freq)?;
            file_handler.write_with_workround(self.min_freq_path(), &adjusted_freq)?;
        }
        Ok(())
    }

    pub fn reset_freq(&mut self, file_handler: &mut FileHandler) -> Result<()> {
        let min_freq = self
            .freqs
            .first()
            .context("No frequencies available")?
            .to_string();
        let max_freq = self
            .freqs
            .last()
            .context("No frequencies available")?
            .to_string();
        self.verify_freq = None;

        file_handler.write_with_workround(self.max_freq_path(), &max_freq)?;
        file_handler.write_with_workround(self.min_freq_path(), &min_freq)?;
        Ok(())
    }

    pub fn read_freq(&self) -> isize {
        fs::read_to_string(self.path.join("scaling_cur_freq"))
            .context("Failed to read scaling_cur_freq")
            .unwrap()
            .trim()
            .parse::<isize>()
            .context("Failed to parse scaling_cur_freq")
            .unwrap()
    }

    fn max_freq_path(&self) -> PathBuf {
        self.path.join("scaling_max_freq")
    }

    fn min_freq_path(&self) -> PathBuf {
        self.path.join("scaling_min_freq")
    }
}
