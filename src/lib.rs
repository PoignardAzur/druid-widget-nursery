// Copyright 2018 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A collection of widgets for the druid GUI framework

pub mod animation;
mod canvas;
mod seperator;
pub mod theme_loader;
mod tree;

#[cfg(feature = "async")]
mod future_widget;

#[cfg(feature = "hot-reload")]
pub mod hot_reload;

pub use canvas::{Canvas, CanvasLayout, CanvasWrap};
pub use seperator::{Orientation, Seperator};
pub use tree::{Tree, TreeNode};

#[cfg(feature = "async")]
pub use future_widget::{Delegate as AsyncDelegate, FutureWidget};
