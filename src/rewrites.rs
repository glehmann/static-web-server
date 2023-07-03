// SPDX-License-Identifier: MIT OR Apache-2.0
// This file is part of Static Web Server.
// See https://static-web-server.net/ for more information
// Copyright (C) 2019-present Jose Quintana <joseluisq.net>

//! Module that allows to rewrite request URLs with pattern matching support.
//!

use wax::{CandidatePath, MatchedText, Pattern};

use crate::settings::Rewrites;

/// It returns a rewrite's destination path if the current request uri
/// matches against the provided rewrites array.
pub fn rewrite_uri_path<'a>(
    uri_path: &'a str,
    rewrites_opts_vec: &'a Option<Vec<Rewrites>>,
) -> Option<(&'a Rewrites, MatchedText<'a>)> {
    if let Some(rewrites_vec) = rewrites_opts_vec {
        for rewrites_entry in rewrites_vec.iter() {
            // Match source glob pattern against request uri path
            // if rewrites_entry.source.is_match(uri_path) {
            if let Some(matched) = rewrites_entry
                .source
                .matched(&CandidatePath::from(uri_path))
            {
                return Some((rewrites_entry, matched.into_owned()));
            }
        }
    }

    None
}
