// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use limine::request::FramebufferRequest;
use microdragon_interface::framebuffer::FramebufferInfo;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn get_framebuffer_info() -> FramebufferInfo {
    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        let fb = response
            .framebuffers()
            .filter(|x| {
                x.bpp() == 32
                    && x.red_mask_size() == 8
                    && x.green_mask_size() == 8
                    && x.blue_mask_size() == 8
            })
            .max_by_key(|x| (x.width(), x.height()));

        if let Some(fb) = fb {
            if fb.addr().is_null() {
                return FramebufferInfo::default();
            } else {
                return FramebufferInfo {
                    address: fb.addr() as u64,
                    size: fb.width() as usize * fb.height() as usize * fb.bpp() as usize,
                    width: fb.width(),
                    height: fb.height(),
                    pitch: fb.pitch(),
                    red_mask_shift: fb.red_mask_shift(),
                    green_mask_shift: fb.green_mask_shift(),
                    blue_mask_shift: fb.blue_mask_shift(),
                };
            }
        }
    }

    FramebufferInfo::default()
}
