// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![no_std]
#![no_main]

mod acpi;
mod framebuffer;
mod memory_map;
mod stack;

use microdragon_interface::ModuleInterface;

microdragon_interface::macros::include_runner!();

/// Entrypoint for the kernel.
/// - Creates the module interface.
/// - Runs the module runner.
/// - Starts the service stack.
fn kernel_main() -> ! {
    let interface = ModuleInterface {
        stack_info: stack::get_stack_info(),
        rsdp_address: acpi::get_rsdp_address(),
        framebuffer_info: framebuffer::get_framebuffer_info(),
        memory_map_info: memory_map::get_memory_map_info(),
        memory_info: memory_map::get_memory_info(),
    };

    run_modules(&interface);

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
