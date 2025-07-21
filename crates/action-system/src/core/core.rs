// Core traits and types for the action system

// Re-export node core types
pub use node_core::{NodeError, NodeResult};

// Battle state for mutable operations during action execution
#[derive(Debug)]
pub struct BattleState {
    pub player_team: crate::Team,
    pub enemy_team: crate::Team,
    pub battle_log: Vec<String>,
}

impl BattleState {
    pub fn new(player_team: crate::Team, enemy_team: crate::Team) -> Self {
        Self {
            player_team,
            enemy_team,
            battle_log: Vec::new(),
        }
    }
    
    pub fn get_character_by_id_mut(&mut self, id: i32) -> Option<&mut crate::Character> {
        self.player_team.get_member_by_id_mut(id)
            .or_else(|| self.enemy_team.get_member_by_id_mut(id))
    }
    
    pub fn get_character_by_id(&self, id: i32) -> Option<&crate::Character> {
        self.player_team.get_member_by_id(id)
            .or_else(|| self.enemy_team.get_member_by_id(id))
    }
    
    pub fn add_log(&mut self, message: String) {
        self.battle_log.push(message);
    }
}

// Trait for executable actions with target information
pub trait Action {
    fn execute(&self, battle_context: &crate::BattleContext, battle_state: &mut BattleState) -> Result<(), String>;
    fn get_action_name(&self) -> &'static str;
}

impl Action for Box<dyn Action> {
    fn execute(&self, battle_context: &crate::BattleContext, battle_state: &mut BattleState) -> Result<(), String> {
        (**self).execute(battle_context, battle_state)
    }
    
    fn get_action_name(&self) -> &'static str {
        (**self).get_action_name()
    }
}

// Simplified rule system - all nodes are unified Node<Box<dyn Action>>
pub type RuleNode = Box<dyn for<'a> node_core::Node<Box<dyn Action>, crate::nodes::evaluation_context::EvaluationContext<'a>> + Send + Sync>;