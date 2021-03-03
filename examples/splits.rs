use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);

        let mut main_panel = Panel::new(PanelLayout::Fill);
        let (top, mut bottom) = main_panel.split_vertical(50);
        let (bl, br) = bottom.split_horizontal(25);

        ui.insert("root", ui.root(), main_panel);
        ui.insert("top_panel", ui.by_name("root").unwrap(), top);
        ui.insert("bottom_panel", ui.by_name("root").unwrap(), bottom);
        ui.insert("bl", ui.by_name("bottom_panel").unwrap(), bl);
        ui.insert("br", ui.by_name("bottom_panel").unwrap(), br);

        // Add fillers to show the split locations
        ui.insert(
            "tf",
            ui.by_name("top_panel").unwrap(),
            Filler::new(to_cp437('#'), ColorPair::new(RED, BLACK)),
        );
        ui.insert(
            "blf",
            ui.by_name("bl").unwrap(),
            Filler::new(to_cp437('!'), ColorPair::new(GREEN, BLACK)),
        );
        ui.insert(
            "blf",
            ui.by_name("br").unwrap(),
            Filler::new(to_cp437('?'), ColorPair::new(GREY, BLACK)),
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
