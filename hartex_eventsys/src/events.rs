//! # The `events` Module
//!
//! This module defines some custom events that can be sent and received within HarTex Discord bot.

use std::{
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use futures_channel::mpsc::UnboundedReceiver;

use futures_util::{
    Stream,
    StreamExt
};

use hartex_model::payload::CommandExecuted;

/// # Struct `Events`
///
/// This is basically a wrapper around `UnboundedReceiver<HarTexEvent>`; it receives event from
/// the stream.
pub struct Events<'a> {
    receiver: UnboundedReceiver<HarTexEvent<'a>>
}

impl<'a> Events<'a> {
    pub fn new(receiver: UnboundedReceiver<HarTexEvent<'a>>) -> Self {
        Self {
            receiver
        }
    }
}

impl<'a> Stream for Events<'a> {
    type Item = HarTexEvent<'a>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(context)
    }
}

/// # Enum `HarTexEvent`
///
/// An enumeration represents the various custom-defined events that is used within HarTex.
pub enum HarTexEvent<'a> {
    /// # Enum Variant `HarTexEvent::CommandExecuted`
    ///
    /// A command is executed.
    ///
    /// ## Tuple Struct Parameters
    /// - `0`, type `Box<CommandExecuted<'a>>`: the payload of the event.
    CommandExecuted(Box<CommandExecuted<'a>>)
}