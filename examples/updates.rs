use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
    counter: usize,
    time_accumulator: f32,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);
        ui.insert(
            "label",
            ui.root(),
            Label::new("FPS: ", ColorPair::new(YELLOW, BLUE), true),
        );
        ui.insert(
            "fps",
            ui.root(),
            Label::new("NaN", ColorPair::new(YELLOW, BLUE), false),
        );

        ui.insert(
            "test_panel",
            ui.root(),
            Panel::new(PanelLayout::Absolute(Rect::with_size(5, 5, 20, 5))),
        );
        ui.insert(
            "test_panel_title",
            ui.by_name("test_panel").unwrap(),
            Border::new(
                false,
                ColorPair::new(WHITE, BLACK),
                Some(BorderTitle {
                    title: "Blah".to_string(),
                    color: ColorPair::new(CYAN, BLACK),
                }),
            ),
        );

        Self {
            ui,
            counter: 0,
            time_accumulator: 0.0,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.counter += 1;
        self.time_accumulator += ctx.frame_time_ms;
        ctx.cls();

        // Peform some updates
        self.ui
            .set_text(self.ui.by_name("fps").unwrap(), format!("{}", ctx.fps));
        self.ui.set_text(
            self.ui.by_name("test_panel_title").unwrap(),
            format!("Frame #{}", self.counter),
        );
        if self.time_accumulator > 1000.0 {
            self.time_accumulator = 0.0;
            let target = self.ui.by_name("test_panel").unwrap();
            let v = self.ui.visible(target);
            if v {
                self.ui.hide(target);
            } else {
                self.ui.show(target);
            }
        }

        // Do the drawing
        self.ui.render_to_batch(ctx).expect("Error batching UI");
        render_draw_buffer(ctx).expect("Render batch error");
    }
}

fn main() -> BError {
    let bracket = BTermBuilder::simple80x50().build()?;
    main_loop(bracket, State::new())
}
