//! Code from https://github.com/DioxusLabs/sdk/blob/main/packages/time/src/interval.rs

use dioxus::{
    core::Task,
    dioxus_core::SpawnIfAsync,
    prelude::{Callback, use_hook},
    signals::{Signal, WritableExt as _},
};
use std::time::Duration;

/// The interface to a debounce.
///
/// You can cancel an interval with [`UseInterval::cancel`].
/// See [`use_interval`] for more information.
#[derive(Clone, PartialEq, Copy)]
pub struct UseInterval {
    inner: Signal<InnerUseInterval>,
}

struct InnerUseInterval {
    pub(crate) interval: Option<Task>,
}

impl Drop for InnerUseInterval {
    fn drop(&mut self) {
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }
    }
}

impl UseInterval {
    /// Cancel the interval.
    pub fn cancel(&mut self) {
        if let Some(interval) = self.inner.write().interval.take() {
            interval.cancel();
        }
    }
}

/// Repeatedly call a function at a specific interval.
///
/// Intervals are cancelable with the [`UseInterval::cancel`] method.
pub fn use_interval<MaybeAsync: SpawnIfAsync<Marker>, Marker>(
    period: Duration,
    callback: impl FnMut(()) -> MaybeAsync + 'static,
) -> UseInterval {
    let inner = use_hook(|| {
        let callback = Callback::new(callback);
        #[cfg(target_family = "wasm")]
        let task = Some(dioxus::core::spawn(async move {
            loop {
                gloo_timers::future::sleep(period).await;
                callback.call(());
            }
        }));

        #[cfg(not(target_family = "wasm"))]
        let task = None;

        Signal::new(InnerUseInterval { interval: task })
    });

    UseInterval { inner }
}
