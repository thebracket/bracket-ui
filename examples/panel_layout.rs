use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);
        let main = ui.insert(ui.root(), Panel::new(PanelLayout::Fill));
        ui.insert(main, Filler::new(to_cp437('░'), ColorPair::new(NAVY, BLUE)));

        let leftist = ui.insert(
            main,
            Panel::new(PanelLayout::Left {
                percent: Some(50),
                min: None,
                max: None,
            }),
        );
        ui.insert(
            leftist,
            Filler::new(to_cp437('░'), ColorPair::new(DARK_GREEN, GREEN)),
        );

        let lt = ui.insert(
            leftist,
            Panel::new(PanelLayout::Top {
                percent: Some(50),
                min: None,
                max: None,
            }),
        );
        ui.insert(lt, Filler::new(to_cp437('░'), ColorPair::new(BLACK, GOLD)));

        // Add some labels
        ui.insert(leftist, Label::new("Lefty", ColorPair::new(BLACK, WHITE)));
        ui.insert(lt, Label::new("Left-Top", ColorPair::new(BLACK, WHITE)));

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
    let bracket = BTermBuilder::simple80x50()
        .with_automatic_console_resize(true)
        .build()?;
    main_loop(bracket, State::new())
}
