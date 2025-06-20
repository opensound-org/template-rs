#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![allow(rustdoc::broken_intra_doc_links)]

//! 🚧 Template for Rust repositories used by `opensound-org` (WIP, nothing usable for now).
//!
//! This is a template repository for Rust projects in the opensound-org organization.
//! Replace this documentation with your actual project description.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
