// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate crypto;
extern crate gpgme;
#[macro_use]
extern crate lazy_static;
extern crate libarchive;
#[macro_use]
extern crate log;
extern crate regex;
extern crate rustc_serialize;

pub use self::error::{Error, Result};

pub mod error;
pub mod fs;
pub mod gpg;
pub mod package;
pub mod util;
