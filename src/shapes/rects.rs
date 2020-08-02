use sdl2::pixels::Color;
use sdl2::rect::Point;
use super::clickable::Clickable;
use super::into_sdl_rect::IntoSdlRect;

pub struct Rects {
    tables_ctrlpts: Vec<Point>,
    selected_table_idx: Option<usize>,
    selected_table_ctrlpt: Option<usize>
}

impl Rects {
    pub fn new() -> Rects {
        Rects {
            tables_ctrlpts: Vec::new(),
            selected_table_idx: None,
            selected_table_ctrlpt: None
        }
    }

    pub fn add_table(&mut self, sdl_rect: sdl2::rect::Rect) {
        // adding the rect's control points
        let mut control_points: Vec<Point> = vec![sdl_rect.top_left(), sdl_rect.top_right(), sdl_rect.bottom_right(), sdl_rect.bottom_left()];

        self.tables_ctrlpts.append(&mut control_points);
    }

    pub fn select_rect(&mut self, clicked_point: sdl2::rect::Point) {
        self.selected_table_ctrlpt = self.tables_ctrlpts
        .iter()
        .enumerate()
        .find(|(_, point)| point.is_hovered(&clicked_point))
        .map(|(idx, _)| idx);

        if let Some(_) = self.selected_table_ctrlpt  {
            return;
        }

        // when we select table as a whole, the `id` corresponds to an id of a chunk of 4
        self.selected_table_idx = self.tables_ctrlpts
        .chunks(4)
        .enumerate()
        .find(|(_, table_ctrlpts)| table_ctrlpts.into_sdl_rect().is_hovered(&clicked_point))
        .map(|(idx, _)| idx);
    }

    pub fn unselect_any_rect(&mut self) {
        self.selected_table_idx = None;
        self.selected_table_ctrlpt = None;
    }

    pub fn move_selected_rect(&mut self, x_diff: i32, y_diff: i32) {
        if let Some(selected_table_idx) = self.selected_table_idx {            
            // moving entire table's control points
            self.tables_ctrlpts[selected_table_idx * 4..=selected_table_idx*4+3]
            .iter_mut()
            .for_each(|selected_ctrlpt| {
                selected_ctrlpt.move_by(x_diff, y_diff)
            });

            return;
        }

        // user grabbed one contol point, but we need to change 4
        if let Some(selected_ctrlpt_idx) = self.selected_table_ctrlpt {
            let first_ctrlpt_in_slice_idx = selected_ctrlpt_idx / 4;
            let manipulated_ctrlpt_num = selected_ctrlpt_idx % 4;
            
            let selected_table_ctrlpts_slice = &mut self.tables_ctrlpts[first_ctrlpt_in_slice_idx* 4..=first_ctrlpt_in_slice_idx*4+3];

            // moving three control points, here the one updated explicitly
            selected_table_ctrlpts_slice[manipulated_ctrlpt_num].move_by(x_diff, y_diff);

            // moving the two control points updated implicitly
            match manipulated_ctrlpt_num {
                0 => {
                    selected_table_ctrlpts_slice[1].move_by(0, y_diff);
                    selected_table_ctrlpts_slice[3].move_by(x_diff, 0);
                },
                1 => {
                    selected_table_ctrlpts_slice[0].move_by(0, y_diff);
                    selected_table_ctrlpts_slice[2].move_by(x_diff, 0);
                },
                2 => {
                    selected_table_ctrlpts_slice[1].move_by(x_diff, 0);
                    selected_table_ctrlpts_slice[3].move_by(0, y_diff);
                },
                3 => {
                    selected_table_ctrlpts_slice[2].move_by(0, y_diff);
                    selected_table_ctrlpts_slice[0].move_by(x_diff, 0);
                }
                _ => {}
            }
        }

    }

    pub fn put_on_window_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        // rendering all rects
        let sdl_tables: Vec<sdl2::rect::Rect> = 
            self.tables_ctrlpts
            .chunks(4)
            .map(|table_ctrlpts| 
                table_ctrlpts.into_sdl_rect()
            ).collect();

        let sdl_ctrlpts: Vec<sdl2::rect::Rect> = 
            self.tables_ctrlpts
            .iter()
            .map(|point| 
                point.into_sdl_rect()
            ).collect();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .draw_rects(sdl_tables.as_slice())?;

        canvas
            .draw_rects(sdl_ctrlpts.as_slice())?;
            
        // rendering selected rect
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        if let Some(selected_idx) = self.selected_table_idx {
            canvas.fill_rect(self.tables_ctrlpts[selected_idx*4..=selected_idx*4+3].into_sdl_rect())?;
            return Ok(())
        }

        if let Some(selected_idx) = self.selected_table_ctrlpt {
            canvas.fill_rect(self.tables_ctrlpts[selected_idx].into_sdl_rect())?;
        }

        Ok(())
    }
}
