use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);
        ui.insert(
            "root",
            "HelloWorld",
            Label::new("Hello World", ColorPair::new(YELLOW, BLACK)),
        );
        ui.insert(
            "root",
            "SecondLine",
            Label::new(
                "Multiple lines reflow on their own.",
                ColorPair::new(WHITE, BLACK),
            ),
        );
        Self { ui }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.ui.render_to_batch(ctx);
        render_draw_buffer(ctx).expect("Render batch error");
    }
}

fn main() -> BError {
    let bracket = BTermBuilder::simple80x50().build()?;
    main_loop(bracket, State::new())
}
