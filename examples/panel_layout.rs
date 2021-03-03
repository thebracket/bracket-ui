use bracket_lib::prelude::*;
use bracket_ui::prelude::*;

struct State {
    ui: UserInterface,
}

impl State {
    fn new() -> Self {
        let mut ui = UserInterface::new(0, 0);
        ui.insert("root", ui.root(), Panel::new(PanelLayout::Fill));
        ui.insert(
            "root_filler",
            ui.by_name("root").unwrap(),
            Filler::new(to_cp437('░'), ColorPair::new(NAVY, BLUE)),
        );

        ui.insert(
            "leftist",
            ui.by_name("root").unwrap(),
            Panel::new(PanelLayout::Left {
                percent: Some(50),
                min: None,
                max: None,
            }),
        );
        ui.insert(
            "leftist_filler",
            ui.by_name("leftist").unwrap(),
            Filler::new(to_cp437('░'), ColorPair::new(DARK_GREEN, GREEN)),
        );

        ui.insert(
            "lt",
            ui.by_name("leftist").unwrap(),
            Panel::new(PanelLayout::Top {
                percent: Some(50),
                min: None,
                max: None,
            }),
        );
        ui.insert(
            "lt_filler",
            ui.by_name("lt").unwrap(),
            Filler::new(to_cp437('░'), ColorPair::new(BLACK, GOLD)),
        );

        // Add some labels
        ui.insert(
            "lefty_border",
            ui.by_name("lt").unwrap(),
            Border::new(
                true,
                ColorPair::new(WHITE, BLACK),
                Some(BorderTitle {
                    title: "Panel Title".to_string(),
                    color: ColorPair::new(CYAN, BLACK),
                }),
            ),
        );
        ui.insert(
            "lefty_label",
            ui.by_name("leftist").unwrap(),
            Label::new("Lefty", ColorPair::new(BLACK, WHITE), false),
        );
        ui.insert(
            "left-top-label",
            ui.by_name("lefty_border").unwrap(),
            Label::new("Left-Top", ColorPair::new(GRAY, BLACK), false),
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
    let bracket = BTermBuilder::simple80x50()
        .with_automatic_console_resize(true)
        .build()?;
    main_loop(bracket, State::new())
}
