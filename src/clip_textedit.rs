use eframe::{egui::*};
use eframe::egui;
use epaint::text::cursor::Cursor;
use epaint::{Color32, text::{LayoutJob, TextFormat}, FontFamily, FontId};

pub struct ClipTextEdit {
    pub text: Vec<String>,
    row_index: Option<usize>,
    cursor: Option<Cursor>,
}

impl ClipTextEdit {
    pub fn new(text: String) -> Self{
        Self {
            text: text.split('\n').map(|s| s.to_string()).collect(),
            row_index: None,
            cursor: None,
        }
    }
}

impl ClipTextEdit {
    pub fn get_now_row(self)->Option<usize>{
        return self.row_index;
    }
    pub fn get_now_cursor(self)->Option<Cursor>{
        return self.cursor;
    }
    pub fn get_now_cursor_pos(self)->Option<usize>{
        match self.cursor{
            None=> {return None},
            Some(c)=>{
                return Some(c.ccursor.index)
            }
        }
    }
}

impl ClipTextEdit {

    pub fn load_text(&mut self, text: String){
        self.text = text.split('\n').map(|s| s.to_string()).collect();
    }

    fn update_text(&mut self, input_text: Vec<&String>){
        input_text.iter().for_each(|t| {
            //if let Some(c) = self.cursor {
            if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                let mut c = c;
                self.text[ri].insert_str(c.ccursor.index, t);
                c.ccursor.index += 1;
                self.cursor = Some(c); //Some(galley.from_ccursor(c.ccursor));
            }
        });
    }

    fn event_key(&mut self, key: &Key){
        if key == &Key::Backspace {
            //if let Some(c) = self.cursor {
            if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                let mut c = c;
                if c.ccursor.index > 0 {
                    let cindex =  c.ccursor.index - 1;
                    self.text[ri].remove(cindex);
                    c.ccursor.index -= 1;
                    self.cursor =  Some(c); //Some(galley.from_ccursor(c.ccursor));
                }else if c.ccursor.index == 0 && ri > 0{
                    let row_text = self.text[ri].clone();
                    let add_idx = self.text[ri-1].len();
                    self.text[ri-1].insert_str(add_idx, &row_text);
                    self.row_index = Some(ri - 1);
                    c.ccursor.index = add_idx; //+ row_text.len();
                    self.cursor = Some(c);
                    self.text.remove(ri);
                }
            }
        }else if key == &Key::Enter {
            if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                let mut c = c;
                let mut row_text = self.text[ri].clone();
                let new_row_text = row_text.split_off(c.ccursor.index);
                self.text[ri] = row_text;
                self.text.insert(ri+1, new_row_text);
                c.ccursor.index = 0;
                self.cursor = Some(c);
                self.row_index = Some(ri + 1);
            }

        }
    }


    fn paint_cursor(&mut self, ui: &mut Ui, row_height: f32, painter:&Painter, pos: Pos2, galley: &Galley, row_index:usize ,cursor: &Cursor){
        let stroke = ui.visuals().selection.stroke;

        let mut cursor_pos = galley.pos_from_cursor(cursor).translate(pos.to_vec2());
        cursor_pos.max.y = cursor_pos.max.y.at_least(cursor_pos.min.y + row_height); // Handle completely empty galleys
        cursor_pos = cursor_pos.expand(1.5); // slightly above/below row
        let top = cursor_pos.center_top();
        let bottom = cursor_pos.center_bottom();
        painter.line_segment(
            [top, bottom],
            (ui.visuals().text_cursor_width, stroke.color),
        );
    }
}

impl ClipTextEdit {
    fn events(&mut self, ui: &mut Ui){
        let events = ui.input(|i| i.events.clone());

        let mut input_text: Vec<&String> = vec![];
        for event in events.iter() {
            match event {
                Event::Text(t) => input_text.push(t),
                Event::Key {
                    key: Key::Backspace,  
                    pressed: true, 
                    modifiers:_, 
                    ..
                } => {
                    if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                        let mut c = c;
                        if c.ccursor.index > 0 {
                            let cindex =  c.ccursor.index - 1;
                            self.text[ri].remove(cindex);
                            c.ccursor.index -= 1;
                            self.cursor =  Some(c); //Some(galley.from_ccursor(c.ccursor));
                        }else if c.ccursor.index == 0 && ri > 0{
                            let row_text = self.text[ri].clone();
                            let add_idx = self.text[ri-1].len();
                            self.text[ri-1].insert_str(add_idx, &row_text);
                            self.row_index = Some(ri - 1);
                            c.ccursor.index = add_idx; //+ row_text.len();
                            self.cursor = Some(c);
                            self.text.remove(ri);
                        }
                    }
                },                
                Event::Key {
                    key: Key::Enter,  
                    pressed: true, 
                    modifiers:_, 
                    ..
                } => {
                    if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                        let mut c = c;
                        let mut row_text = self.text[ri].clone();
                        let new_row_text = row_text.split_off(c.ccursor.index);
                        self.text[ri] = row_text;
                        self.text.insert(ri+1, new_row_text);
                        c.ccursor.index = 0;
                        self.cursor = Some(c);
                        self.row_index = Some(ri + 1);
                    }
                },
                _ => {},
            }
        }

        if input_text.len() >= 1 {
            println!("Input text len: {}", input_text.len());
            self.update_text(input_text);
        }

    }
}

impl ClipTextEdit {

    pub fn show_editor(&mut self, ui: &mut Ui, allocate_max_rect: Rect){

        ui.allocate_ui_at_rect(
            allocate_max_rect,
            |ui|{
                //エディタの外周
                let outer_rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(
                    outer_rect,
                    2.0,                              //  curve
                    Color32::from_rgb(48, 0, 0)     //  color
                );

                //フォントの設定
                let fontid = FontId::new(16.0, FontFamily::Monospace);

                //フォントの高さ
                let row_height = ui.fonts(|f| f.row_height(&fontid));

                //行数を取得
                let total_rows = self.text.len();

                //layouterの設定
                const MIN_WIDTH:f32 = 24.0;
                let available_width = ui.available_width().at_least(MIN_WIDTH);

                let layouter_font_id = fontid.clone();
                let mut default_layouter = move |ui: &Ui, text: &str, wrap_width: f32| {
                    let layout_job = LayoutJob::simple_singleline(text.to_string(), layouter_font_id.clone(), Color32::from_rgba_premultiplied(100, 100, 100, 100));
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                let layouter = &mut default_layouter;

                let output = egui::ScrollArea::vertical()
                .max_width(f32::INFINITY)
                //.stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, total_rows, |ui, row_range| {

                    //EVENT
                    let ctx = ui.ctx();
                    let evts = ctx.input(|i| i.events.clone());

                    let mut input_text: Vec<&String> = vec![];
                    for ev in evts.iter() {
                        match ev {
                            Event::Text(t) => input_text.push(t),
                            Event::Key {key, pressed, modifiers:_, ..} => {
                                if pressed == &true { self.event_key(key);}
                            },                

                            _ => {},
                        }
                    }

                    if input_text.len() >= 1 {
                        println!("Input text len: {}", input_text.len());
                        self.update_text(input_text);
                    }


                    let mut row_number_width = 0.0; //row_number_galley.mesh_bounds.max.x - row_number_galley.mesh_bounds.min.x;
                    let row_number_enable = true;
                    
                    if row_number_enable {
                        let _row_num_galley = layouter(ui, &self.text.len().to_string(), available_width);
                        row_number_width = _row_num_galley.mesh_bounds.max.x - _row_num_galley.mesh_bounds.min.x;
                        //println!("galley: min:{:?}, max:{:?}", row_number_galley.mesh_bounds.min, row_number_galley.mesh_bounds.max);
                    } 

                    for row in row_range {
                        if row < self.text.len() {
                            ui.horizontal(|ui|{

                            let mut galley = layouter(ui, &self.text[row], available_width);
                            if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                                if ri == row{
                                    self.cursor = Some(galley.from_ccursor(c.ccursor));
                                }
                            }

                            if row_number_enable {
                                let mut row_num_galley = layouter(ui, &(row+1).to_string(), available_width);

                                let desired_width = row_number_width;
                                let desired_height = row_height;
                                let desired_size = vec2(desired_width, galley.size().y.max(desired_height));//.at_least(min_size - margin * 2.0);
                                let (id, rect) = ui.allocate_space(desired_size);

                                let painter = ui.painter_at(rect);
                                painter.galley(rect.min, row_num_galley.clone());
                            }

                            let desired_width = available_width;
                            let desired_height = row_height;
                            let desired_size = vec2(desired_width, galley.size().y.max(desired_height));//.at_least(min_size - margin * 2.0);
                            let (id, rect) = ui.allocate_space(desired_size);

                            let painter = ui.painter_at(rect);
                            let sense = Sense::click();

                            let mut response = ui.interact(rect, id, sense);

                            if response.clicked() {
                                if let Some(click_pos) = response.interact_pointer_pos() {
                                    let pos = click_pos - rect.min;
                                    let c = galley.cursor_from_pos(pos);
                                    println!("Click index: {:?}, Row: {:?}", c.ccursor.index, row);
                                    self.row_index = Some(row);
                                    self.cursor = Some(c);
                                }
                            }

                            painter.galley(rect.min, galley.clone());
                            if let (Some(ri) , Some(c)) =  (self.row_index, self.cursor) {
                                if ri == row {
                                    self.paint_cursor(ui, row_height, &painter, rect.min, &galley, ri, &c);
                                }
                            }
                            //let mut layout_job = LayoutJob::simple(self.input_text[row].clone(), FontId::new(16.0, FontFamily::Monospace) , Color32::GREEN, 100.0);
                            //layout_job.halign = Align::LEFT;
                            ////ui.label(self.input_text[row].clone());
                            //ui.label(layout_job);

                            });
                        }
                    }
                });
            }
        );
    }
}