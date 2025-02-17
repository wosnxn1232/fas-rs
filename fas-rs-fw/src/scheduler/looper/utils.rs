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
use std::collections::hash_map::Entry;

use log::info;

use super::{super::FasData, Buffer, Looper};
use crate::{config::TargetFps, error::Result, PerformanceController};

impl<P: PerformanceController> Looper<P> {
    /* 检查是否为顶层应用，并且删除不是顶层应用的buffer **/
    pub fn retain_topapp(&mut self) {
        self.buffers
            .retain(|(_, p), _| self.topapp_checker.is_topapp(*p).unwrap_or(false));
    }

    pub fn buffer_update(&mut self, d: &FasData) -> Result<()> {
        if !self.topapp_checker.is_topapp(d.pid)? || d.frametime.is_zero() {
            return Ok(());
        } else if d.target_fps == TargetFps::Value(0) {
            panic!("Target fps must be bigger than zero");
        }

        let process = (d.pkg.clone(), d.pid);
        let target_fps = d.target_fps;

        match self.buffers.entry(process) {
            Entry::Occupied(mut o) => o.get_mut().push_frametime(d.frametime),
            Entry::Vacant(v) => {
                info!("Loaded fas on {:?}", v.key());
                let mut buffer = Buffer::new(target_fps);
                buffer.push_frametime(d.frametime);
                v.insert(buffer);
            }
        }

        Ok(())
    }
}
