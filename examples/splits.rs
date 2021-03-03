use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);

        let mut main_panel = Panel::new(PanelLayout::Fill);
        let (top,bottom) = main_panel.split_horizontal(50);

        ui.insert("root", ui.root(), main_panel);
        ui.insert("tf", top, Filler::new(to_cp437('#'), ColorPair::new(RED, BLACK)));
        ui.insert("bf", bottom, Filler::new(to_cp437('!'), ColorPair::new(GREEN, BLACK)));

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
