use macroquad::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const HISTORY_SIZE: usize = 256;

#[derive(Default)]
struct Metric {
    avg: f32,
    max: f32,
    last_start: Option<Instant>,
    history: Vec<f32>,
}

#[derive(Default)]
pub struct Profiler {
    metrics: HashMap<String, Metric>,
    alpha: f32, // smoothing factor
}

impl Profiler {
    pub fn new(alpha: f32) -> Self {
        Self {
            metrics: HashMap::new(),
            alpha,
        }
    }

    pub fn register(&mut self, name: &str) {
        self.metrics
            .entry(name.to_string())
            .or_insert_with(Metric::default);
    }

    pub fn start(&mut self, name: &str) {
        if let Some(metric) = self.metrics.get_mut(name) {
            metric.last_start = Some(Instant::now());
        }
    }

    pub fn end(&mut self, name: &str) {
        if let Some(metric) = self.metrics.get_mut(name) {
            if let Some(start) = metric.last_start.take() {
                let ms = start.elapsed().as_secs_f32() * 1000.0;

                metric.avg = self.alpha * ms + (1.0 - self.alpha) * metric.avg;
                metric.max = metric.max.max(ms);

                if metric.history.len() == HISTORY_SIZE {
                    metric.history.remove(0);
                }
                metric.history.push(ms);
            }
        }
    }

    fn compute_p99(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let mut sorted = samples.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = (sorted.len() as f32 * 0.99).ceil() as usize - 1;
        sorted[idx.min(sorted.len() - 1)]
    }

    pub fn update(&self, ctx: &egui::Context) {
        egui::Window::new("Profiler")
            .default_pos(egui::pos2(10.0, 10.0))
            .resizable(false)
            .show(ctx, |ui| {
                for (name, metric) in &self.metrics {
                    let p99 = Self::compute_p99(&metric.history);
                    let text = format!(
                        "{name}: avg {:.2} ms, p99 {:.2} ms, max {:.2} ms",
                        metric.avg, p99, metric.max,
                    );
                    ui.label(text);
                }
            });
    }
}
