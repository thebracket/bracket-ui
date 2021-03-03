use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);
        ui.track_mouse();
        ui.insert(
            "btn1",
            ui.root(),
            Button::new(
                "You can click me. It's ok.",
                ColorPair::new(YELLOW, BLUE),
                ButtonDecor::Plain,
                false,
            ),
        );
        ui.insert(
            "btn2",
            ui.root(),
            Button::new(
                "I'm clickable, too.",
                ColorPair::new(YELLOW, BLUE),
                ButtonDecor::Border,
                false,
            ),
        );

        Self { ui }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.ui.render_to_batch(ctx).expect("Error batching UI");
        render_draw_buffer(ctx).expect("Render batch error");

        loop {
            if let Some(msg) = self.ui.pop_message() {
                println!("{:#?}", msg);
            } else {
                break;
            }
        }
    }
}

fn main() -> BError {
    let bracket = BTermBuilder::simple80x50().build()?;
    main_loop(bracket, State::new())
}
