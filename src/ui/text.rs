use piston_window::graphics::{Context, Graphics, Transformed, character, text};

pub struct Label {
    x: f64,
    y: f64,
    label_text: String,
}

impl Label {
    pub fn new(x: f64, y: f64, label_text: String) -> Self {
        Self {
            x: x,
            y: y,
            label_text: label_text,
        }
    }

    pub fn draw<G, C>(&self, context: &Context, graphics: &mut G, glyphs: &mut C)
    where
        G: Graphics<Texture = C::Texture>,
        C: character::CharacterCache,
    {
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
            .draw(
                &self.label_text,
                glyphs,
                &context.draw_state,
                context.transform.trans(self.x as f64, self.y as f64),
                graphics,
            )
            .unwrap();
    }
}
