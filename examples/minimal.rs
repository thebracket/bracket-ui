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
            ui.root(),
            Label::new("Hello World", ColorPair::new(YELLOW, BLACK), true),
        );
        ui.insert(
            "root",
            ui.root(),
            Label::new("(Same Line Demo)", ColorPair::new(CYAN, BLACK), false),
        );
        ui.insert(
            "root",
            ui.root(),
            Label::new("Hello World on a 2nd line", ColorPair::new(GREEN, BLACK), false),
        );

        Self { ui }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.ui.render_to_batch(ctx).expect("Error batching UI");
        render_draw_buffer(ctx).expect("Render batch error");
    }
}

fn main() -> BError {
    let bracket = BTermBuilder::simple80x50().build()?;
    main_loop(bracket, State::new())
}
