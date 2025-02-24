enum Camera {
    Normal { base_transform: i32 },
    Volume { transform: i32 },
}

fn draw_ui(camera: &mut Camera) {
    || {
        let (Camera::Normal {
            base_transform: _transform,
        }
        | Camera::Volume {
            transform: _transform,
        }) = camera;
    };
}
