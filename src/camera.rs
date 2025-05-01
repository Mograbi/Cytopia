pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub drag_start_x: i32,
    pub drag_start_y: i32,
    pub is_dragging: bool,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            x: 0.0,
            y: 0.0,
            drag_start_x: 0,
            drag_start_y: 0,
            is_dragging: false,
            zoom: 1.0,
        }
    }

    pub fn start_drag(&mut self, x: i32, y: i32) {
        self.drag_start_x = x;
        self.drag_start_y = y;
        self.is_dragging = true;
    }

    pub fn update_drag(&mut self, x: i32, y: i32) {
        if self.is_dragging {
            let dx = (x - self.drag_start_x) as f32;
            let dy = (y - self.drag_start_y) as f32;
            self.x += dx;
            self.y += dy;
            self.drag_start_x = x;
            self.drag_start_y = y;
        }
    }

    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
    }

    pub fn zoom_at_point(&mut self, screen_x: i32, screen_y: i32, zoom_delta: f32) {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom * zoom_delta).clamp(0.5, 3.0);

        // Convert screen coordinates to world coordinates before zoom
        let world_x = (screen_x as f32 - self.x) / old_zoom;
        let world_y = (screen_y as f32 - self.y) / old_zoom;

        // Calculate how the world point moves after zoom
        let dx = world_x * (self.zoom - old_zoom);
        let dy = world_y * (self.zoom - old_zoom);

        // Adjust camera to keep the mouse point fixed on screen
        self.x -= dx;
        self.y -= dy;
    }
} 