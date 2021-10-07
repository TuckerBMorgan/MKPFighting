#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub struct CurveConfig {
    start: f32,
    stop: f32,
    duration: f32,

}

struct CurvesSystem {
    curves: HashMap<String, CurvesSystem>
}

pub fn curve_update_system(
    mut curves: ResMut<CurvesSystem>
) { 

}