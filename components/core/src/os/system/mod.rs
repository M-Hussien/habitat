// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::uname;

#[cfg(not(windows))]
pub mod linux;
#[cfg(not(windows))]
pub use self::linux::uname;

#[derive(Debug)]
pub struct Uname {
    pub sys_name:  String,
    pub node_name: String,
    pub release:   String,
    pub version:   String,
    pub machine:   String,
}