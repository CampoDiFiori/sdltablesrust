use sdl2::pixels::Color;
use super::point::Point;
use super::rect::Rect;
use super::clickable::Clickable;

pub struct Rects {
    tables: Vec<Rect>,
    control_points: Vec<Rect>,
    selected_table: Option<usize>,
    selected_control_point: Option<usize>
}

impl Rects {
    pub fn new() -> Rects {
        Rects {
            tables: Vec::new(),
            control_points: Vec::new(),
            selected_table: None,
            selected_control_point: None
        }
    }

    pub fn add_table(&mut self, rect: Rect) {
        // adding the rect's control points
        let mut control_points: Vec<Rect> = rect.get_all_points()
        .iter()
        .map(|point| point.to_rect())
        .collect();

        self.control_points.append(&mut control_points);

        // adding a table
        self.tables.push(rect);
    }

    pub fn select_rect(&mut self, clicked_point: Point) {
        let selected_ctrl_pt = self.control_points.iter()
        .enumerate()
        .find(|(_, rect)| rect.is_hovered(&clicked_point));

        self.selected_control_point = match selected_ctrl_pt {
            Some((idx, _)) => Some(idx),
            None => None,
        };

        if let Some(_) = self.selected_control_point  {
            return;
        }

        let selected_table = self.tables.iter()
        .enumerate()
        .find(|(_, rect)| rect.is_hovered(&clicked_point));

        self.selected_table = match selected_table {
            Some((idx, _)) => Some(idx),
            None => None,
        };

    }

    pub fn unselect_any_rect(&mut self) {
        self.selected_table = None;
        self.selected_control_point = None;
    }

    pub fn move_selected_rect(&mut self, x_diff: i32, y_diff: i32) {
        if let Some(selected_table_idx) = self.selected_table {
            // moving the table 
            self.tables[selected_table_idx].move_by(x_diff, y_diff);
            
            // moving table's control points
            (selected_table_idx * 4..=selected_table_idx*4+3)
            .into_iter()
            .for_each(|selected_ctrlpt_idx| {
                self.control_points[selected_ctrlpt_idx].move_by(x_diff, y_diff)
            });

            return;
        }

        if let Some(selected_ctrl_pt_idx) = self.selected_control_point {
            let manipulated_table_idx = selected_ctrl_pt_idx / 4;
            let manipulated_ctrl_pt_num = selected_ctrl_pt_idx % 4;
            
            let mut all_ctrl_pts = self.tables[manipulated_table_idx].get_all_points();

            // moving three control points, here the one updated explicitly
            all_ctrl_pts[manipulated_ctrl_pt_num].move_by(x_diff, y_diff);

            // moving the two control points updated implicitly
            match manipulated_ctrl_pt_num {
                0 => {
                    all_ctrl_pts[1].move_by(0, y_diff);
                    all_ctrl_pts[3].move_by(x_diff, 0);
                },
                1 => {
                    all_ctrl_pts[0].move_by(0, y_diff);
                    all_ctrl_pts[2].move_by(x_diff, 0);
                },
                2 => {
                    all_ctrl_pts[1].move_by(x_diff, 0);
                    all_ctrl_pts[3].move_by(0, y_diff);
                },
                3 => {
                    all_ctrl_pts[2].move_by(0, y_diff);
                    all_ctrl_pts[0].move_by(x_diff, 0);
                }
                _ => {}
            }

            self.tables[manipulated_table_idx] = Rect::new(
                all_ctrl_pts[0].x,
                all_ctrl_pts[0].y,
                all_ctrl_pts[2].x,
                all_ctrl_pts[2].y
            );

            for i in 0..=3 {
                self.control_points[manipulated_table_idx * 4 + i] = all_ctrl_pts[i].to_rect();
            }
        }

    }

    pub fn put_on_window_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        // rendering all rects
        let sdl_tables: Vec<sdl2::rect::Rect> =
            self.tables.iter().map(|rect| rect.to_sdl_rect()).collect();

        let sdl_control_points: Vec<sdl2::rect::Rect> = 
            self.control_points.iter().map(|rect| rect.to_sdl_rect()).collect();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .draw_rects(sdl_tables.as_slice())?;

        canvas
            .draw_rects(sdl_control_points.as_slice())?;
            
        // rendering selected rect
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        if let Some(selected_idx) = self.selected_table {
            canvas.fill_rect(self.tables[selected_idx].to_sdl_rect())?;
            return Ok(())
        }

        if let Some(selected_idx) = self.selected_control_point {
            canvas.fill_rect(self.control_points[selected_idx].to_sdl_rect())?;
        }

        Ok(())
    }
}
