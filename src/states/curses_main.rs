use crate::*;

pub struct InitState;

impl game_engine_core::State<GameData> for InitState {
    fn on_start(&mut self, data: &mut GameData) {
        println!("Game started!");
        let entity = data.world.get_mut::<Entities>().unwrap().create();
        /*data.world
        .get_mut::<Components<_>>()
        .unwrap()
        .insert(entity, Pos(1, 1));*/

        let mut curses = EasyCurses::initialize_system().expect("Failed to start ncurses.");
        curses.set_input_mode(InputMode::Character);
        curses.set_keypad_enabled(true);
        curses.set_echo(false);
        curses.set_cursor_visibility(CursorVisibility::Invisible);
        curses.set_input_timeout(TimeoutMode::Immediate);
        #[cfg(unix)]
        unsafe {
            // TODO remove uses of escape key and then this dependency
            ncurses::ll::set_escdelay(0)
        };

        curses.refresh();
        *data.world.get_mut::<Option<Curses>>().unwrap() = Some(Curses(curses));
    }

    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        while data.world.get_mut::<Time>().unwrap().step_fixed_update() {
            data.logic_dispatcher
                .run_seq(&mut data.world)
                .expect("Failed to run systems.");
            data.world.maintain();
        }
        data.render_dispatcher
            .run_seq(&mut data.world)
            .expect("Failed to run systems.");
        let mut trans = StateTransition::None;
        for ev in data.world.get::<Vec<InputEvent>>().unwrap().iter() {
            match ev {
                InputEvent::Crafting => trans = StateTransition::Push(Box::new(CraftingState)),
                InputEvent::Inventory => trans = StateTransition::Push(Box::new(InventoryState)),
                _ => {}
            }
        }
        clear_events.system().run(&mut data.world).unwrap();
        data.world.maintain();

        trans
    }
}
