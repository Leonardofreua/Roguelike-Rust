use specs::prelude::*;
use rltk::console;
use super::{CombatStats, WantsToMelee, Name, SufferDamage};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
  type SystemData = (Entities<'a>,
                     ReadStorage<'a, Name>,
                     ReadStorage<'a, CombatStats>,
                     WriteStorage<'a, SufferDamage>,
                     WriteStorage<'a, WantsToMelee>);

  fn run(&mut self, data: Self::SystemData) {
    let (entities, names, combat_stats, mut inflict_damage, mut wants_melee) = data;

    for (_entity, name, stats, wants_melee) in (&entities, &names, &combat_stats, &wants_melee).join() {
      if stats.hp > 0 {
        let target_stats = combat_stats.get(wants_melee.target).unwrap();

        if target_stats.hp > 0 {
          let target_name = names.get(wants_melee.target).unwrap();
          let damage = i32::max(0, stats.power - target_stats.defense);

          if damage == 0 {
            console::log(&format!("{} is unable to hurt {}", &name.name, &target_name.name));
          } else {
            console::log(&format!("{} hits {}, for {} hp", &name.name, &target_name.name, damage));
            SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
          }
        }
      }
    }

    wants_melee.clear();
  }
}