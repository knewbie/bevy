use crate::{Asset, Assets};
use bevy_app::prelude::*;
use bevy_diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy_ecs::system::{IntoSystem, Res, ResMut};

/// Adds "asset count" diagnostic to an App
pub struct AssetCountDiagnosticsPlugin<T: Asset> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Asset> Default for AssetCountDiagnosticsPlugin<T> {
    fn default() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Asset> Plugin for AssetCountDiagnosticsPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup_system.system())
            .add_system(Self::diagnostic_system.system());
    }
}

impl<T: Asset> AssetCountDiagnosticsPlugin<T> {
    pub fn diagnostic_id() -> DiagnosticId {
        DiagnosticId(T::TYPE_UUID)
    }

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(
            Self::diagnostic_id(),
            format!("asset_count {}", std::any::type_name::<T>()),
            20,
        ));
    }

    pub fn diagnostic_system(mut diagnostics: ResMut<Diagnostics>, assets: Res<Assets<T>>) {
        diagnostics.add_measurement(Self::diagnostic_id(), assets.len() as f64);
    }
}
