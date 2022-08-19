mod modifier;
mod skill;


pub struct Player {
    player_id: i32,
    player_name: String,
    player_race: Race,
    player_alignment: Alignment,
    background: Background,
    class: Class,



   // max_health: u16,
   // level: u8,
   // modifiers: PlayerModifiers,
   // saving_throw: SavingThrows,
   // skills: Skills,
   // proficiency_bonus: u8,
   // speed: u8,
   // armor: Armor,
   // death_save_sucess: u8,
   // death_save_fail: u8,

}
struct Race {
    race_id: i32,
    race_name: String,
}

struct Class {
    class_id: i32,
    class_name: String,
}

struct Alignment {
    alignment_id: i32,
    alignment_name: String,
}

struct Background {
    background_id: i32,
    background_name: String,
}














#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
