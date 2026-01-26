use piston_window::graphics::{Context, Graphics, rectangle};

pub struct UIButton {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    pub label: String,
    pub color: [f32; 4],
}

impl UIButton {
    pub fn new(x: f64, y: f64, width: f64, height: f64, label: &str, color: [f32; 4]) -> Self {
        UIButton {
            x,
            y,
            width,
            height,
            label: label.to_string(),
            color,
        }
    }

    pub fn is_clicked(&self, mouse_x: f64, mouse_y: f64) -> bool {
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }

    pub fn is_hovered(&self, mouse_x: f64, mouse_y: f64) -> bool {
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }

    pub fn draw<G: Graphics>(
        &self,
        context: &Context,
        graphics: &mut G,
        is_hovered: bool,
        is_selected: bool,
    ) {
        let bg_color = if is_selected {
            [
                self.color[0] * 1.0,
                self.color[1] * 1.0,
                self.color[2] * 1.0,
                1.0,
            ]
        } else if is_hovered {
            self.color
        } else {
            [
                self.color[0] * 0.1,
                self.color[1] * 0.1,
                self.color[2] * 0.1,
                1.0,
            ]
        };

        rectangle(
            bg_color,
            [self.x, self.y, self.width, self.height],
            context.transform,
            graphics,
        );

        let border_width = if is_selected { 3.0 } else { 2.0 };
        rectangle::Rectangle::new_border([0.0, 0.0, 0.0, 1.0], border_width).draw(
            [self.x, self.y, self.width, self.height],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
