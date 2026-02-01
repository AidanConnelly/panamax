use std::collections::HashSet;
use fastbloom::BloomFilter;
use serde::{Deserialize, Serialize};
use crate::crate_version_set::CrateVersionCheckResult::{Required, Unrequired};

type CrateVersion = String;

#[derive(Debug, Deserialize, Serialize)]
enum CrateVersionSet {
    BloomFilter(CrateVersionBloomFilter),
    Vec(CrateVersionHashSet),
}

impl CheckInsertCrateVersion for CrateVersionSet{
    fn check(&self, crate_version: CrateVersion) -> CrateVersionCheckResult {
        match self {
            CrateVersionSet::BloomFilter(filter) => filter.check(crate_version),
            CrateVersionSet::Vec(vec) => vec.check(crate_version)
        }
    }

    fn insert(&mut self, crate_version: CrateVersion) {
        match self {
            CrateVersionSet::BloomFilter(filter) => filter.insert(crate_version),
            CrateVersionSet::Vec(vec) => vec.insert(crate_version)
        }
    }
}

#[derive(Debug)]
struct CrateVersionBloomFilter(BloomFilter);
#[derive(Debug)]
struct CrateVersionHashSet(HashSet<String>);

// As we want to reduce storage space use, false positives aren't a problem, as it just leads to extra crates
enum CrateVersionCheckResult {
    Required,
    Unrequired,
}

trait CheckInsertCrateVersion {
    fn check(&self, crate_version: CrateVersion) ->CrateVersionCheckResult;
    fn insert(&mut self, crate_version: CrateVersion);
}

impl CheckInsertCrateVersion for CrateVersionBloomFilter {
    fn check(&self, crate_version: CrateVersion) -> CrateVersionCheckResult {
        match self.0.contains(&crate_version) {
            true => Required,
            false => Unrequired,
        }
    }

    fn insert(&mut self, crate_version: CrateVersion) {
        self.0.insert(&crate_version);
    }
}

impl CheckInsertCrateVersion for CrateVersionHashSet {
    fn check(&self, crate_version: CrateVersion) -> CrateVersionCheckResult {
        match self.0.contains(&crate_version) {
            true => Required,
            false => Unrequired,
        }
    }

    fn insert(&mut self, crate_version: CrateVersion) {
        self.0.insert(crate_version);
    }
}