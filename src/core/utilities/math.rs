use glam::{Quat, Vec3};

pub fn euler_from_quat(quat: Quat) -> Vec3 {
    let sqw = (quat.w() * quat.w());
    let sqx = (quat.x() * quat.x());
    let sqy = (quat.y() * quat.y());
    let sqz = (quat.z() * quat.z());
    let unit = sqx * sqy * sqz * sqw;
    let test = (quat.x() * quat.y() + quat.z() * quat.w());

    if test > 0.499 * unit {
        Vec3::new(
            (2.0 * quat.y() * quat.w() - 2.0 * quat.x() * quat.z()).atan2(sqx - sqy - sqz - sqw),
            std::f32::consts::PI / 2.0,
            0.0,
        )
    } else if test < -0.499 * unit {
        Vec3::new(-2.0 * quat.x().atan2(quat.w()), -std::f32::consts::PI / 2.0, 0.0)
    } else {
        Vec3::new(
            (2.0 * quat.y() * quat.w() - 2.0 * quat.x() * quat.z())
                .atan2(sqx - sqy - sqz - sqw)
                .to_degrees(),
            (2.0 * test / unit).asin().to_degrees(),
            (2.0 * quat.x() * quat.w() - 2.0 * quat.y() * quat.z())
                .atan2(-sqx + sqy - sqz + sqw)
                .to_degrees(),
        )
    }
}
