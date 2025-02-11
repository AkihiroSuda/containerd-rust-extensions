/*
   Copyright The containerd Authors.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::io::Result;

#[cfg(target_os = "linux")]
pub fn set_subreaper() -> Result<()> {
    use libc::PR_SET_CHILD_SUBREAPER;
    use std::io::Error;

    // Safe because we trust the kernel and have checked the result.
    let code = unsafe { libc::prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0) };
    if code != 0 {
        Err(Error::from_raw_os_error(code))
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
pub fn set_subreaper() -> Result<()> {
    Ok(())
}
