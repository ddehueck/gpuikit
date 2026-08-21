//! Input components for text editing in GPUI applications.
//!
//! This module provides the state management and keybinding infrastructure for
//! building text input components. It includes:
//!
//! - [`InputState`]: The core state model for text input, handling content,
//!   selection, cursor management, and edit history.
//! - [`CursorBlink`]: Manages cursor blinking state for input components.
//! - [`InputBindings`]: Configurable keybindings for input actions.
//! - [`TextDirection`]: Bidirectional text support for RTL/LTR detection.
//!
//! # Example
//!
//! ```ignore
//! use gpui::Context;
//! use gpuikit::input::{InputState, bind_input_keys};
//!
//! // Initialize keybindings (typically in app initialization)
//! bind_input_keys(cx, None);
//!
//! // Create an input state
//! let input = cx.new(|cx| InputState::new_singleline(cx));
//! ```

use std::ops::Range;

use gpui::HighlightStyle;

/// A GPUI highlight applied to a UTF-8 byte range in the input content.
///
/// Ranges must be sorted, non-overlapping, within the current content, and
/// begin and end on UTF-8 character boundaries. Empty ranges are ignored.
///
/// ```ignore
/// use gpui::{rgb, HighlightStyle};
/// use gpuikit::elements::input::{input, InputHighlight};
/// use gpuikit::elements::textarea::textarea;
///
/// let highlights: Vec<InputHighlight> = vec![(
///     0..5,
///     HighlightStyle {
///         color: Some(rgb(0x4ade80).into()),
///         ..Default::default()
///     },
/// )];
///
/// input(&state, cx).highlights(highlights.clone());
/// textarea(&state, cx).highlights(highlights);
/// ```
pub type InputHighlight = (Range<usize>, HighlightStyle);

mod bidi;
/// Input keybinding configuration & actions that can be bound (`Backspace`, `Copy`, etc.).
///
/// Explicitly not exported using `pub use bindings::*` to avoid namespace pollution.
pub mod bindings;
mod blink;
mod handler;
mod state;

pub use bidi::{detect_base_direction, TextDirection};
pub use bindings::{bind_input_keys, InputBindings, INPUT_CONTEXT};
pub use blink::CursorBlink;
pub use handler::*;
pub use state::{InputLineLayout, InputState, InputStateEvent, SubmitOn};
