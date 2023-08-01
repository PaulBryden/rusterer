#[derive(Default, Debug, Clone)]
pub struct Texture
{
  pub pixels: Vec<u32>,
  pub width: u32,
  pub height: u32
}

impl Texture
{
  pub fn get_color_at_normalized_coord(&self, x_coord: &f32, y_coord: &f32) -> &u32
  {
      let texture_coord_x = (x_coord*((self.width-1)as f32)).round() as u32;
      let texture_coord_y = (self.height-1)-(y_coord*((self.height-1)as f32)).round() as u32;
      &self.pixels[((texture_coord_x+(texture_coord_y*(self.width))) as usize)]
  }
}