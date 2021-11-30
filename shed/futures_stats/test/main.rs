/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use futures::{future, stream, FutureExt, TryStreamExt};

use futures_stats::{TimedFutureExt, TimedStreamExt};

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let fut = future::lazy(|_| {
        println!("future polled");
        Ok::<(), ()>(())
    })
    .timed()
    .map(|(stats, res)| {
        println!("{:#?}", stats);
        res
    });
    runtime.block_on(fut).unwrap();

    let stream = stream::iter([1, 2, 3].map(Ok::<u32, ()>)).timed(|stats| {
        println!("{:#?}", stats);
        future::ready(())
    });
    runtime
        .block_on(stream.try_for_each(|_| future::ok(())))
        .unwrap();

    let empty: Vec<Result<u32, ()>> = vec![];
    let stream = stream::iter(empty.into_iter()).timed(|stats| {
        assert!(stats.first_item_time.is_none());
        future::ready(())
    });
    runtime
        .block_on(stream.try_for_each(|_| future::ok(())))
        .unwrap();
}
