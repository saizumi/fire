use raylib::prelude::*;
use rand;

#[warn(dead_code)]
const FIRE_WIDTH: i32 = 60;
const FIRE_HEIGHT: i32 = 60;
const SIZE: i32 = 10;

pub const FIRE_COLORS_PALETTE: [(u8, u8, u8); 37] = [
    (7, 7, 7), (31, 7, 7), (47, 15, 7), (71, 15, 7), (87, 23, 7), (103, 31, 7), (119, 31, 7), (143, 39, 7),
    (159, 47, 7), (175, 63, 7), (191, 71, 7), (199, 71, 7), (223, 79, 7), (223, 87, 7), (223, 87, 7), (215, 95, 7),
    (215, 95, 7), (215, 103, 15), (207, 111, 15), (207, 119, 15), (207, 127, 15), (207, 135, 23), (199, 135, 23), (199, 143, 23),
    (199, 151, 31), (191, 159, 31), (191, 159, 31), (191, 167, 39), (191, 167, 39), (191, 175, 47), (183, 175, 47), (183, 183, 47),
    (183, 183, 55), (207, 207, 111), (223, 223, 159), (239, 239, 199), (255, 255, 255)
];

fn main() {
  let (mut rl, thread) = raylib::init()
    .size(FIRE_WIDTH * SIZE, FIRE_HEIGHT * SIZE)  
    .title("DOOM FIRE")
    .build();
  
  rl.set_target_fps(20);
  let mut fire_array = create_array();
  create_fire_data_structure(&mut fire_array);
  create_fire_source(&mut fire_array);
  
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    render_fire(&mut fire_array, &mut d);
    calculate_fire_propagation(&mut fire_array, &mut d);
    d.clear_background(Color::WHITE);
  }
}

fn create_array()  -> Vec<i32> {
  let array: Vec<i32> = vec![0; 64];
  return array;
}

fn create_fire_data_structure(fire_array: &mut Vec<i32>) {
  let number_of_pixels: i32 = FIRE_WIDTH * FIRE_HEIGHT;
  for _i in 0..number_of_pixels {
    fire_array.push(0);
  }
}

fn calculate_fire_propagation(fire_array: &mut Vec<i32>, d: &mut RaylibDrawHandle) {
  for column in 0..FIRE_WIDTH {
    for row in 0..FIRE_HEIGHT {
      let pixel_index: i32 = column + (FIRE_HEIGHT * row);
      update_fire_intensity_per_pixel(pixel_index, fire_array);
    }
  }
  render_fire(fire_array, d);
}

fn update_fire_intensity_per_pixel(current_pixel_index: i32, fire_array: &mut Vec<i32>) {
  let below_pixel_index: i32 = current_pixel_index + FIRE_WIDTH;

  // Verificação de índice fora dos limites
  if below_pixel_index >= FIRE_WIDTH * FIRE_HEIGHT {
      return;
  }

  // Gerando decaimento aleatório
  let decay: i32 = rand::Rng::gen_range(&mut rand::thread_rng(), 0..3);

  // Obtendo intensidade do pixel abaixo
  let below_pixel_fire_intensity = fire_array.get(below_pixel_index as usize).copied().unwrap_or(0);

  // Cálculo da nova intensidade
  let new_intensity = (below_pixel_fire_intensity - decay).max(0); // Ensure decay doesn't result in negative intensity

  // Atualização da intensidade no array
  fire_array[current_pixel_index as usize] = new_intensity;
}


fn render_fire(fire_array: &mut Vec<i32>, d: &mut RaylibDrawHandle) {
  for row in 0..FIRE_WIDTH {
    for column in 0..FIRE_HEIGHT {
      let pixel_index = column + (row * FIRE_WIDTH);
      let fire_intensity = fire_array[pixel_index as usize];
      let position_pixel_index_x = SIZE * column;
      let position_pixel_index_y: i32 = SIZE * row;
      let color = get_color_from_palette(&FIRE_COLORS_PALETTE, fire_intensity as usize);
      d.draw_rectangle(
        position_pixel_index_x,
        position_pixel_index_y,
        SIZE,
        SIZE,
        color);
    }
  }
  
}
fn create_fire_source(fire_array: &mut Vec<i32>) {
  for column in 0..FIRE_WIDTH {
    let overflow_pixel_index = FIRE_WIDTH * FIRE_HEIGHT;
    let pixel_index = (overflow_pixel_index - FIRE_WIDTH) + column;
    fire_array[pixel_index as usize] = 36;

  }
}

fn get_color_from_palette(palette: &[(u8, u8, u8)], id: usize) -> Color {
  let (red, green, blue) = palette[id];
  Color { r: red, g: green, b: blue, a: 255 }
}

