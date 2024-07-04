// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use limine::request::RsdpRequest;

static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

pub fn get_rsdp_address() -> u64 {
    if let Some(response) = RSDP_REQUEST.get_response() {
        return response.address() as u64;
    }

    0
}
