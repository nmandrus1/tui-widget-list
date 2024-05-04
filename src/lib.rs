//! # A versatile widget list for Ratatui
//!
//!<div align="center">
//!     
//! [![Continuous Integration](https://github.com/preiter93/tui-widget-list/actions/workflows/ci.yml/badge.svg)](https://github.com/preiter93/tui-widget-list/actions/workflows/ci.yml)
//!
//! </div>
//!
//! This crate provides a stateful widget [`List`] implementation for `Ratatui`, enabling listing
//! widgets that implement the [`ListWidget`] trait. The associated [`ListState`], offers functionalities
//! such as navigating to the next and previous items.
//! Additionally, the lists support both horizontal and vertical scrolling.
//!
//! ## Configuration
//! The [`List`] can be customized with the following options:
//! - `scroll_direction`: Specifies whether the list is vertically or horizontally scrollable.
//! - `style`: Defines the base style of the list.
//! - `block`: Optional outer block surrounding the list.
//!
//! You can adjust the behavior of [`ListState`] with the following options:
//! - `circular`: Determines if the selection is circular. When enabled, selecting the last item loops back to the first. Enabled by default.
//!
//! ## Example
//! ```
//! use ratatui::prelude::*;
//! use tui_widget_list::{List, ListState, ListWidget, PreRenderContext};
//!
//! #[derive(Debug, Clone)]
//! pub struct ListItem {
//!     text: String,
//!     style: Style,
//! }
//!
//! impl ListItem {
//!     pub fn new<T: Into<String>>(text: T) -> Self {
//!         Self {
//!             text: text.into(),
//!             style: Style::default(),
//!         }
//!     }
//! }
//!
//! impl ListWidget for ListItem {
//!    fn pre_render(mut self, context: &PreRenderContext) -> (Self, u16) {
//!        // Set alternating styles
//!        if context.index % 2 == 0 {
//!            self.style = Style::default().bg(Color::Rgb(28, 28, 32));
//!        } else {
//!            self.style = Style::default().bg(Color::Rgb(0, 0, 0));
//!        }
//!
//!        // Highlight the selected widget
//!        if context.is_selected {
//!            self.style = Style::default()
//!                .bg(Color::Rgb(255, 153, 0))
//!                .fg(Color::Rgb(28, 28, 32));
//!        };
//!
//!        // Example: set main axis size to 1
//!        let main_axis_size = 1;
//!
//!        (self, main_axis_size)
//!    }
//! }
//!
//! impl Widget for ListItem {
//!     fn render(self, area: Rect, buf: &mut Buffer) {
//!         Line::from(self.text).style(self.style).render(area, buf);
//!     }
//! }
//!
//! pub fn render(f: &mut Frame) {
//!     let list = List::new(vec![
//!         ListItem::new("Item 1"),
//!         ListItem::new("Item 2"),
//!     ]);
//!     let mut state = ListState::default();
//!     f.render_stateful_widget(list, f.size(), &mut state);
//! }
//! ```
//!
//! For more examples see [tui-widget-list](https://github.com/preiter93/tui-widget-list/tree/main/examples).
//!
//! ## Demos
//!
//! ### Simple list with alternating colors
//!
//!![](examples/simple.gif?v=1)
//!
//! ### Vertically and horizontally scrollable
//!
//!![](examples/demo.gif?v=1)
pub mod state;
pub mod traits;
pub mod traits_deprecated;
pub mod widget;

pub use state::ListState;
pub use traits::{ListWidget, PreRenderContext};
pub use widget::{List, ScrollAxis};

#[allow(deprecated)]
pub use traits_deprecated::ListableWidget;
