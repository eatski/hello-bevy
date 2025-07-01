// Core traits and types for the action system

// Common error type for all resolvers and nodes
#[derive(Clone, Debug, PartialEq)]
pub enum NodeError {
    // General evaluation error
    EvaluationError(String),
    // Action resolution should break/skip this rule
    Break,
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            NodeError::Break => write!(f, "Action resolution break"),
        }
    }
}

impl std::error::Error for NodeError {}

// Type alias for Result with NodeError
pub type NodeResult<T> = Result<T, NodeError>;

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
pub trait Action: Send + Sync + std::fmt::Debug {
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

// Trait for nodes that can resolve to actions or break
pub trait ActionResolver: Send + Sync + std::fmt::Debug {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<Box<dyn Action>>;
}

impl ActionResolver for Box<dyn ActionResolver> {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<Box<dyn Action>> {
        (**self).resolve(battle_context, rng)
    }
}

// Simplified rule system - all nodes are ActionResolvers
pub type RuleNode = Box<dyn ActionResolver>;