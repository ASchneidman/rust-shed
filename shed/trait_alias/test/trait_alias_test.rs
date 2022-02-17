/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

trait One {
    fn one(&self) -> u32;
}

trait Two {
    fn two(&self) -> u32;
}

#[trait_alias::trait_alias]
trait Both = One + Two;

struct Impl;

impl One for Impl {
    fn one(&self) -> u32 {
        1
    }
}

impl Two for Impl {
    fn two(&self) -> u32 {
        2
    }
}

fn test(both: &impl Both) {
    assert_eq!(both.one(), 1);
    assert_eq!(both.two(), 2);
}

#[test]
fn main() {
    test(&Impl);
}
