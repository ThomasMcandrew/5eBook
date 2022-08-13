use crate::skill::Skill;
use chrono::prelude::*;

pub struct PlayerSkill {
    skill: Skill,
    proficiency: bool,
    player: Player,
    level_added: u8,
    date_added: DateTime,
}
