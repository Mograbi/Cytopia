use sdl2::{
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    ttf::Font,
    surface::Surface,
    render::TextureCreator,
    video::WindowContext,
};

pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub is_hovered: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            text: text.to_string(),
            is_hovered: false,
        }
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }

    pub fn render(&self, canvas: &mut WindowCanvas, font: &Font, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        // Define colors
        let (bg_color, border_color, text_color) = if self.is_hovered {
            (
                Color::RGB(70, 130, 180),  // Steel Blue when hovered
                Color::RGB(135, 206, 235), // Sky Blue border when hovered
                Color::RGB(255, 255, 255)  // White text when hovered
            )
        } else {
            (
                Color::RGB(47, 79, 79),    // Dark Slate Gray
                Color::RGB(112, 128, 144), // Slate Gray border
                Color::RGB(220, 220, 220)  // Light gray text
            )
        };

        // Draw button shadow
        let shadow_rect = Rect::new(
            self.rect.x + 2,
            self.rect.y + 2,
            self.rect.width(),
            self.rect.height()
        );
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.fill_rect(shadow_rect)?;

        // Draw button background
        canvas.set_draw_color(bg_color);
        canvas.fill_rect(self.rect)?;

        // Draw button border
        canvas.set_draw_color(border_color);
        canvas.draw_rect(self.rect)?;

        // Draw inner border for depth effect
        let inner_border = Rect::new(
            self.rect.x + 1,
            self.rect.y + 1,
            self.rect.width() - 2,
            self.rect.height() - 2
        );
        canvas.draw_rect(inner_border)?;

        // Render text with a slight shadow effect
        let text_surface = font
            .render(&self.text)
            .blended(text_color)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .map_err(|e| e.to_string())?;

        // Center text in button
        let text_rect = Rect::new(
            self.rect.x + (self.rect.width() as i32 - text_surface.width() as i32) / 2,
            self.rect.y + (self.rect.height() as i32 - text_surface.height() as i32) / 2,
            text_surface.width(),
            text_surface.height(),
        );

        canvas.copy(&texture, None, text_rect)?;
        Ok(())
    }
}

pub struct MainMenu {
    pub buttons: Vec<Button>,
    pub background_color: Color,
    window_width: u32,
    window_height: u32,
}

impl MainMenu {
    pub fn new(window_width: u32, window_height: u32) -> Result<Self, String> {
        let button_width = 200;
        let button_height = 50;
        let button_spacing = 20;
        let start_y = (window_height as i32 - (4 * button_height + 3 * button_spacing) as i32) / 2;

        let buttons = vec![
            Button::new(
                (window_width as i32 - button_width as i32) / 2,
                start_y,
                button_width,
                button_height,
                "New Game",
            ),
            Button::new(
                (window_width as i32 - button_width as i32) / 2,
                start_y + (button_height + button_spacing) as i32,
                button_width,
                button_height,
                "Load Game",
            ),
            Button::new(
                (window_width as i32 - button_width as i32) / 2,
                start_y + 2 * (button_height + button_spacing) as i32,
                button_width,
                button_height,
                "Settings",
            ),
            Button::new(
                (window_width as i32 - button_width as i32) / 2,
                start_y + 3 * (button_height + button_spacing) as i32,
                button_width,
                button_height,
                "Exit",
            ),
        ];

        Ok(Self {
            buttons,
            background_color: Color::RGB(28, 28, 35), // Darker, slightly bluish background
            window_width,
            window_height,
        })
    }

    pub fn update_layout(&mut self, window_width: u32, window_height: u32) {
        self.window_width = window_width;
        self.window_height = window_height;
    }

    pub fn update(&mut self, mouse_x: i32, mouse_y: i32) {
        for button in &mut self.buttons {
            button.is_hovered = button.contains_point(mouse_x, mouse_y);
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, font: &Font, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        // Check if window size has changed
        let (current_width, current_height) = canvas.output_size()?;
        if current_width != self.window_width || current_height != self.window_height {
            self.update_layout(current_width, current_height);
        }

        // Draw background
        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(Rect::new(0, 0, current_width, current_height))?;

        // Draw title with shadow
        let title = "Cytopia";
        
        // Draw title shadow
        let shadow_surface = font
            .render(title)
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;
        let shadow_texture = texture_creator
            .create_texture_from_surface(&shadow_surface)
            .map_err(|e| e.to_string())?;

        let shadow_rect = Rect::new(
            (current_width as i32 - shadow_surface.width() as i32) / 2 + 2,
            52,
            shadow_surface.width(),
            shadow_surface.height(),
        );
        canvas.copy(&shadow_texture, None, shadow_rect)?;

        // Draw main title
        let title_surface = font
            .render(title)
            .blended(Color::RGB(135, 206, 235)) // Sky Blue title
            .map_err(|e| e.to_string())?;
        let title_texture = texture_creator
            .create_texture_from_surface(&title_surface)
            .map_err(|e| e.to_string())?;

        let title_rect = Rect::new(
            (current_width as i32 - title_surface.width() as i32) / 2,
            50,
            title_surface.width(),
            title_surface.height(),
        );
        canvas.copy(&title_texture, None, title_rect)?;

        // Draw buttons
        for button in &self.buttons {
            button.render(canvas, font, texture_creator)?;
        }

        Ok(())
    }
} 