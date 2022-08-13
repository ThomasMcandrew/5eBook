mod modifier;
mod skill;


struct Player {
    name: String,
    race: Race,
    alignment: Alignment,
    background: Background,
    class: Class,
    max_health: u16,
    level: u8,
    modifiers: PlayerModifiers,
    saving_throw: SavingThrows,
    skills: Skills,
    proficiency_bonus: u8,
    speed: u8,
    armor: Armor,
    death_save_sucess: u8,
    death_save_fail: u8,

}
struct PlayerModifiers {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}















#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
