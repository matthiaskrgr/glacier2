enum Camera {
    Normal { base_transform: i32 },
    Volume { transform: i32 },
}

fn draw_ui(camera: &mut Camera) {
    || {
        let (Camera::Normal {
            base_transform: _,
        }
        | Camera::Volume {
            transform: _,
        }) = camera;
    };
}
