// SPDX-FileCopyrightText: Timothée Ravier <tim@siosm.fr>
// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const MOK_EVENTS_PCR14: [&str; 3] = ["MokList", "MokListX", "MokListTrusted"];

fn mok_event_to_file_name(event_name: &str) -> String {
    format!("{event_name}RT")
}

//fn mok_event_hash(events_dir_path: &Path, event_name: &str) -> Vec<u8> {
//    let data = fs::read(events_dir_path.join(mok_event_to_file_name(event_name))).unwrap();
//    Sha256::digest(data).to_vec()
//}

fn mok_event_hash(events_dir_path: &Path, event_name: &str) -> Option<Vec<u8>> {
    match fs::read(events_dir_path.join(mok_event_to_file_name(event_name))) {
        Ok(data) => Some(Sha256::digest(data).to_vec()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => panic!("{e:?}"),
    }
}

#[derive(Debug, Clone)]
pub struct MokEventHashes {
    /// Path to the directory containing MokList{}RT files
    path: PathBuf,
    index: usize,
}

impl MokEventHashes {
    pub fn new(path: &str) -> MokEventHashes {
        MokEventHashes {
            path: path.into(),
            index: 0,
        }
    }
}

impl Iterator for MokEventHashes {
    type Item = Vec<u8>;

    //fn next(&mut self) -> Option<Self::Item> {
    //    let hash = mok_event_hash(&self.path, MOK_EVENTS_PCR14.get(self.index)?);
    //    self.index += 1;
    //    Some(hash)
    //}
    fn next(&mut self) -> Option<Self::Item> {
    while let Some(&event_name) = MOK_EVENTS_PCR14.get(self.index) {
        self.index += 1;
        if let Some(hash) = mok_event_hash(&self.path, event_name) {
            return Some(hash);
        }
    }
    None
    }
}
