/* Copyright 2023 shadow3aaa@gitbub.com
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License. */
mod policy;

use std::{cell::Cell, collections::HashSet, ffi::OsStr, fs};

use anyhow::Result;
use fas_rs_fw::prelude::*;
use log::error;

use crate::error::Error;
use policy::Policy;

pub type Freq = u32; // 单位: khz

pub struct CpuCommon {
    step: Freq,
    policies: Vec<Policy>,
    enable: Cell<bool>,
}

impl CpuCommon {
    pub fn new(config: &Config) -> Result<Self> {
        let mut policies: Vec<_> = fs::read_dir("/sys/devices/system/cpu/cpufreq")?
            .filter_map(|d| Some(d.ok()?.path()))
            .filter(|p| p.is_dir())
            .filter(|p| {
                p.file_name()
                    .and_then(OsStr::to_str)
                    .unwrap()
                    .contains("policy")
            })
            .map(Policy::new)
            .map(Result::unwrap)
            .collect();

        policies.sort_unstable();

        let ignore = config
            .get_conf("ignore_little")?
            .as_bool()
            .ok_or(Error::ParseConfig)?;

        // 设置了忽略小核则去掉第一个
        if policies.len() > 2 {
            if ignore {
                policies.remove(0);
            } else {
                policies.first_mut().unwrap().is_little = true.into();
            }
        }

        let mut freqs: Vec<_> = policies
            .iter()
            .flat_map(|p| p.freqs.iter().copied())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        freqs.sort_unstable();

        let step = freqs.windows(2).map(|arr| arr[1] - arr[0]).min().unwrap();

        let max_freq_all = freqs.last().copied().unwrap();
        policies.iter_mut().for_each(|p| p.max_freq = max_freq_all);

        Ok(Self {
            step,
            policies,
            enable: Cell::new(false),
        })
    }
}

impl PerformanceController for CpuCommon {
    fn limit(&self, _c: &Config) -> fas_rs_fw::Result<()> {
        self.policies
            .iter()
            .for_each(|p| p.limit(self.step).unwrap_or_else(|e| error!("{e:?}")));
        Ok(())
    }

    fn release(&self, _c: &Config) -> fas_rs_fw::Result<()> {
        self.policies
            .iter()
            .for_each(|p| p.release(self.step).unwrap_or_else(|e| error!("{e:?}")));
        Ok(())
    }

    fn release_max(&self, _c: &Config) -> fas_rs_fw::Result<()> {
        self.policies
            .iter()
            .for_each(|p| p.release_max().unwrap_or_else(|e| error!("{e:?}")));
        Ok(())
    }

    fn init_game(&self, _c: &Config) -> Result<(), fas_rs_fw::Error> {
        self.enable.set(true);
        self.policies
            .iter()
            .for_each(|p| p.init_game().unwrap_or_else(|e| error!("{e:?}")));
        Ok(())
    }

    fn init_default(&self, _c: &Config) -> Result<(), fas_rs_fw::Error> {
        self.enable.set(false);
        self.policies
            .iter()
            .for_each(|p| p.init_default().unwrap_or_else(|e| error!("{e:?}")));
        Ok(())
    }
}
