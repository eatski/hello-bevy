// Action implementations with target information

use super::core::Action;

#[derive(Debug)]
pub struct StrikeAction {
    target_id: i32,
}

impl StrikeAction {
    pub fn new(target_id: i32) -> Self {
        Self { target_id }
    }
}

impl Action for StrikeAction {
    fn execute(&self, battle_context: &crate::BattleContext, battle_state: &mut super::BattleState) -> Result<(), String> {
        let acting_character = battle_context.get_acting_character();
        
        // Check if acting character can attack (alive)
        if acting_character.hp <= 0 {
            return Err("Acting character is dead and cannot attack".to_string());
        }
        
        // Get target character by ID (mutable)
        let target_character = battle_state.get_character_by_id_mut(self.target_id)
            .ok_or_else(|| format!("Target character with ID {} not found", self.target_id))?;
        
        // Calculate damage
        let damage = acting_character.attack;
        let target_name = target_character.name.clone();
        let target_id = target_character.id;
        
        // Apply damage to target
        target_character.take_damage(damage);
        
        // Log the action
        let log_message = format!("Strike: {} (ID:{}) attacks {} (ID:{}) for {} damage! (HP: {} -> {})", 
                                 acting_character.name, acting_character.id,
                                 target_name, target_id,
                                 damage, target_character.hp + damage, target_character.hp);
        battle_state.add_log(log_message);
        
        Ok(())
    }
    
    fn get_action_name(&self) -> &'static str {
        "Strike"
    }
}

#[derive(Debug)]
pub struct HealAction {
    target_id: i32,
}

impl HealAction {
    pub fn new(target_id: i32) -> Self {
        Self { target_id }
    }
}

impl Action for HealAction {
    fn execute(&self, battle_context: &crate::BattleContext, battle_state: &mut super::BattleState) -> Result<(), String> {
        let acting_character_id = battle_context.get_acting_character().id;
        
        // Get acting character (mutable) to consume MP
        let acting_character = battle_state.get_character_by_id_mut(acting_character_id)
            .ok_or_else(|| format!("Acting character with ID {} not found", acting_character_id))?;
        
        // Check if acting character can heal (alive and has MP)
        if acting_character.hp <= 0 {
            return Err("Acting character is dead and cannot heal".to_string());
        }
        if acting_character.mp < 10 {
            return Err("Acting character does not have enough MP to heal".to_string());
        }
        
        // Consume MP
        acting_character.consume_mp(10);
        let acting_name = acting_character.name.clone();
        
        // Get target character by ID (mutable)
        let target_character = battle_state.get_character_by_id_mut(self.target_id)
            .ok_or_else(|| format!("Target character with ID {} not found", self.target_id))?;
        
        // Calculate healing
        let heal_amount = 30;
        let target_name = target_character.name.clone();
        let target_id = target_character.id;
        let old_hp = target_character.hp;
        
        // Apply healing to target
        target_character.heal(heal_amount);
        
        // Log the action
        let log_message = format!("Heal: {} (ID:{}) heals {} (ID:{}) for {} HP! (HP: {} -> {})", 
                                 acting_name, acting_character_id,
                                 target_name, target_id,
                                 heal_amount, old_hp, target_character.hp);
        battle_state.add_log(log_message);
        
        Ok(())
    }
    
    fn get_action_name(&self) -> &'static str {
        "Heal"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, BattleState};

    #[test]
    fn test_strike_action() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new(3, "Attacker".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let strike = StrikeAction::new(enemy.id);
        let player_team = crate::Team::new("Player Team".to_string(), vec![player.clone()]);
        let enemy_team = crate::Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let mut battle_state = BattleState::new(player_team, enemy_team);
        
        let result = strike.execute(&battle_context, &mut battle_state);
        
        assert!(result.is_ok());
        assert_eq!(strike.get_action_name(), "Strike");
    }

    #[test]
    fn test_heal_action() {
        let player = Character::new(4, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(5, "Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new(6, "Healer".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let heal = HealAction::new(player.id);
        let player_team = crate::Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = crate::Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let mut battle_state = BattleState::new(player_team, enemy_team);
        
        let result = heal.execute(&battle_context, &mut battle_state);
        
        assert!(result.is_ok());
        assert_eq!(heal.get_action_name(), "Heal");
    }

    #[test]
    fn test_heal_action_insufficient_mp() {
        let player = Character::new(7, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(8, "Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new(9, "Healer".to_string(), 100, 5, 25); // Low MP
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let heal = HealAction::new(player.id);
        let player_team = crate::Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = crate::Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let mut battle_state = BattleState::new(player_team, enemy_team);
        
        let result = heal.execute(&battle_context, &mut battle_state);
        
        assert!(result.is_err(), "Heal action should fail with insufficient MP");
    }
}