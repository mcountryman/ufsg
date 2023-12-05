mod console;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::tracing::span::{Attributes, Record};
use bevy::utils::tracing::{subscriber, Id, Subscriber};
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};
use puffin_egui::puffin::{self, ThreadProfiler};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::Duration;
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::fmt::{FormatFields, FormattedFields};
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::{LookupSpan, Registry};
use tracing_subscriber::{EnvFilter, Layer};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    puffin::set_scopes_on(true);
    subscriber::set_global_default(Registry::default().with(PuffinLayer::new())).unwrap();

    app
      .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
      .add_systems(First, new_frame)
      .add_systems(Update, draw_debug);
  }
}

fn new_frame() {
  puffin::GlobalProfiler::lock().new_frame();
}

fn draw_debug(mut egui: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
  let fps = diagnostics
    .get(FrameTimeDiagnosticsPlugin::FPS)
    .and_then(|fps| fps.smoothed())
    .unwrap_or_default();

  let frame_time = diagnostics
    .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
    .and_then(|frame_time| frame_time.smoothed())
    .map(|frame_time| Duration::from_millis(frame_time as _))
    .unwrap_or_default();

  egui::Window::new("debug")
    .title_bar(false)
    .movable(false)
    .resizable(false)
    .anchor(Align2::LEFT_TOP, (10.0, 10.0))
    .show(egui.ctx_mut(), |ui| {
      egui::Grid::new("info")
        .num_columns(2)
        .striped(true)
        .spacing([-4.0, 0.0])
        .show(ui, |ui| {
          ui.label("fps");
          ui.label(format!("{fps:.0}"));
          ui.end_row();

          ui.label("time");
          ui.label(format!("{frame_time:.0?}"));
          ui.end_row();
        })
    });

  // puffin_egui::profiler_window(egui.ctx_mut());
}

/// A tracing layer that collects data for puffin.
pub struct PuffinLayer<F = DefaultFields> {
  fmt: F,
}

impl Default for PuffinLayer<DefaultFields> {
  fn default() -> Self {
    Self::new()
  }
}

thread_local! {
  static PUFFIN_SPAN_STACK: RefCell<VecDeque<(Id, usize)>> = RefCell::new(VecDeque::with_capacity(16));
}

impl PuffinLayer<DefaultFields> {
  /// Create a new `PuffinLayer`.
  pub fn new() -> Self {
    Self {
      fmt: DefaultFields::default(),
    }
  }

  /// Use a custom field formatting implementation.
  pub fn with_formatter<F>(self, fmt: F) -> PuffinLayer<F> {
    let _ = self;
    PuffinLayer { fmt }
  }
}

impl<S: Subscriber, F> Layer<S> for PuffinLayer<F>
where
  S: Subscriber + for<'a> LookupSpan<'a>,
  F: for<'writer> FormatFields<'writer> + 'static,
{
  fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
    if !puffin::are_scopes_on() {
      return;
    }

    if let Some(span) = ctx.span(id) {
      let mut extensions = span.extensions_mut();
      if extensions.get_mut::<FormattedFields<F>>().is_none() {
        let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
        if self.fmt.format_fields(fields.as_writer(), attrs).is_ok() {
          extensions.insert(fields);
        }
      }
    }
  }

  fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
    if let Some(span) = ctx.span(id) {
      let mut extensions = span.extensions_mut();
      if let Some(fields) = extensions.get_mut::<FormattedFields<F>>() {
        let _ = self.fmt.add_fields(fields, values);
      } else {
        let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
        if self.fmt.format_fields(fields.as_writer(), values).is_ok() {
          extensions.insert(fields);
        }
      }
    }
  }

  fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
    if !puffin::are_scopes_on() {
      return;
    }

    if let Some(span_data) = ctx.span(id) {
      let metadata = span_data.metadata();
      let name = metadata.name();
      let target = metadata.target();
      let extensions = span_data.extensions();
      let data = extensions
        .get::<FormattedFields<F>>()
        .map(|fields| fields.fields.as_str())
        .unwrap_or_default();

      ThreadProfiler::call(|tp| {
        let start_stream_offset = tp.begin_scope(name, target, data);
        PUFFIN_SPAN_STACK.with(|s| {
          s.borrow_mut().push_back((id.clone(), start_stream_offset));
        });
      });
    }
  }

  fn on_exit(&self, id: &Id, _ctx: Context<'_, S>) {
    PUFFIN_SPAN_STACK.with(|s| {
      let value = s.borrow_mut().pop_back();
      if let Some((last_id, start_stream_offset)) = value {
        if *id == last_id {
          ThreadProfiler::call(|tp| tp.end_scope(start_stream_offset));
        } else {
          s.borrow_mut().push_back((last_id, start_stream_offset));
        }
      }
    });
  }

  fn on_close(&self, id: Id, ctx: Context<'_, S>) {
    if let Some(span) = ctx.span(&id) {
      span.extensions_mut().remove::<FormattedFields<F>>();
    }
  }
}
