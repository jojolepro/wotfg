use crate::*;

pub fn curses_render_close_items_system(
    controlled: &Components<Player>,
    inventories: &Components<Inventory<Items, (), ()>>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    items: &Components<ItemInstance<Items, ()>>,
    positions: &Components<Position>,
    render: &RenderInfo,
    auth: &Auth,
    selected_item: &QuickItemSelect,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(40, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("=== Close Items ===");
    let mut y = 41;
    for (player, player_position) in join!(&controlled && &positions) {
        if player.unwrap().id == auth.id {
            let mut i = 0;
            for (item, item_position) in join!(&items && &positions) {
                // TODO move this check in a reusable function
                if player_position.unwrap().distance(item_position.unwrap()) <= PICKUP_DISTANCE
                    && player_position.unwrap().z() == item_position.unwrap().z()
                {
                    // TODO check that item is right next to player
                    let def = item_defs
                        .defs
                        .get(&item.as_ref().unwrap().key)
                        .unwrap_or_else(|| {
                            panic!(
                                "Failed to find item def for item key {:?}",
                                item.as_ref().unwrap().key
                            )
                        });
                    curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                    curses.set_color_pair(def.user_data.rarity.into());
                    if let Some(select) = selected_item.selected {
                        if select == i {
                            curses.print(format!(
                                ">{} x{}",
                                def.name,
                                item.as_ref().unwrap().quantity
                            ));
                        } else {
                            curses.print(format!(
                                " {} x{}",
                                def.name,
                                item.as_ref().unwrap().quantity
                            ));
                        }
                    } else {
                        curses.print(format!(
                            " {} x{}",
                            def.name,
                            item.as_ref().unwrap().quantity
                        ));
                    }
                    y += 1;
                    i += 1;
                }
            }
        }
    }
    Ok(())
}
