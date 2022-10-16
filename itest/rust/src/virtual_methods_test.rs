/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![allow(dead_code)]

use godot_core::api::RefCounted;
use godot_core::builtin::GodotString;
use godot_core::obj::{Base, Gd};
use godot_core::traits::GodotExt;
use godot_macros::{godot_api, itest, GodotClass};

/// Simple class, that deliberately has no constructor accessible from GDScript
#[derive(GodotClass, Debug)]
#[godot(base=RefCounted)]
struct WithoutInit {
    #[base]
    some_base: Base<RefCounted>,
}

#[derive(GodotClass, Debug)]
#[godot(init, base=RefCounted)]
struct VirtualMethodTest {
    #[base]
    some_base: Base<RefCounted>,

    integer: i32,
}

#[godot_api]
impl VirtualMethodTest {}

#[godot_api]
impl GodotExt for VirtualMethodTest {
    fn to_string(&self) -> GodotString {
        format!("VirtualMethodTest[integer={}]", self.integer).into()
    }
}

pub(crate) fn run() -> bool {
    let mut ok = true;
    ok &= test_to_string();
    ok
}

// pub(crate) fn register() {
//     godot_core::register_class::<VirtualMethodTest>();
// }

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[itest]
fn test_to_string() {
    let _obj = Gd::<VirtualMethodTest>::new_default();
    dbg!(_obj);
}
