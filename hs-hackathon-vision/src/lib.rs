pub(crate) mod raw;

use crate::raw::distance::centroid_distance;
use crate::raw::led_detector::get_leds;
use image::DynamicImage;
pub use raw::bounding_box::BoundingBox;
pub use raw::colors::Color;
pub use raw::led_detector::{Led, LedDetectionConfig};

/// Detect all LEDs that are visible in a given frame
pub fn detect(frame: &DynamicImage, configuration: &LedDetectionConfig) -> eyre::Result<Vec<Led>> {
    get_leds(frame, configuration)
}

/// Get distance between two LEDs
pub fn distance(led_1: &Led, led_2: &Led) -> u32 {
    centroid_distance(led_1.bbox, led_2.bbox)
}
