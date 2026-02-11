use bevy::prelude::*;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, _app: &mut App) {
        // Intentionally empty for now.
        // (Your previous .ogg loading was failing due to loader/feature mismatch.)
    }
}
