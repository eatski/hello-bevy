mod core;

fn main() {
    use rand::seq::SliceRandom;
    
    // Example battle simulation
    let player = core::Character::new("Hero", 100, 20, 10, 15);
    let enemy = core::Character::new("Goblin", 50, 15, 5, 10);
    let mut battle = core::Battle::new(player, enemy);
    let mut rng = rand::thread_rng();

    // Get available skills once before the battle
    let available_skills = battle.player.get_available_skills().to_vec();
    
    println!("Battle Start!");
    println!("Hero's available skills: {:?}", available_skills);
    
    loop {
        // Randomly choose a skill for demonstration
        let skill = available_skills.choose(&mut rng).unwrap();
        
        // Execute turns until someone wins
        let battle_over = battle.execute_turn(skill);

        // Print latest battle log and status
        if let Some(log) = battle.logs.last() {
            let status = match (log.damage, log.healing) {
                (Some(damage), _) => format!(" for {} damage!", damage),
                (_, Some(healing)) => format!(" for {} HP!", healing),
                _ => "!".to_string(),
            };
            println!("{} {} {}{}", log.actor, log.action, log.target, status);
            println!("Hero HP: {:.1}%, Goblin HP: {:.1}%",
                battle.player.hp_percentage(),
                battle.enemy.hp_percentage()
            );
        }

        if battle_over {
            match battle.is_player_winner() {
                Some(true) => println!("Player wins!"),
                Some(false) => println!("Enemy wins!"),
                None => unreachable!(),
            }
            break;
        }
    }
}
