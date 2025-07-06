use action_system::{ActionCalculationSystem, Action, BattleState, RuleNode, Character, Team, TeamSide, BattleContext};
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};

pub struct TeamBattle {
    pub player_team: Team,
    pub enemy_team: Team,
    pub current_turn: usize,
    pub current_character_index: usize,
    pub current_team: TeamSide,
    pub battle_over: bool,
    pub winner: Option<String>,
    pub battle_log: Vec<String>,
    pub player_action_systems: Vec<ActionCalculationSystem>,
    pub enemy_action_systems: Vec<ActionCalculationSystem>,
    pub rng: StdRng,
}

impl TeamBattle {
    pub fn new(
        player_team: Team,
        enemy_team: Team,
        player_rules: Vec<Vec<RuleNode>>,
        enemy_rules: Vec<Vec<RuleNode>>,
        mut rng: StdRng,
    ) -> Self {
        let player_action_systems: Vec<ActionCalculationSystem> = player_rules
            .into_iter()
            .map(|rules| {
                let system_rng = StdRng::from_seed(rng.gen());
                ActionCalculationSystem::new(rules, system_rng)
            })
            .collect();

        let enemy_action_systems: Vec<ActionCalculationSystem> = enemy_rules
            .into_iter()
            .map(|rules| {
                let system_rng = StdRng::from_seed(rng.gen());
                ActionCalculationSystem::new(rules, system_rng)
            })
            .collect();

        Self {
            player_team,
            enemy_team,
            current_turn: 0,
            current_character_index: 0,
            current_team: TeamSide::Player,
            battle_over: false,
            winner: None,
            battle_log: Vec::new(),
            player_action_systems,
            enemy_action_systems,
            rng,
        }
    }

    pub fn is_player_turn(&self) -> bool {
        !self.battle_over && self.current_team == TeamSide::Player
    }

    pub fn get_current_acting_character(&self) -> Option<&Character> {
        match self.current_team {
            TeamSide::Player => self.player_team.alive_members().get(self.current_character_index).copied(),
            TeamSide::Enemy => self.enemy_team.alive_members().get(self.current_character_index).copied(),
        }
    }

    pub fn get_current_acting_character_index(&self) -> Option<usize> {
        let alive_members = match self.current_team {
            TeamSide::Player => self.player_team.alive_members(),
            TeamSide::Enemy => self.enemy_team.alive_members(),
        };
        
        if self.current_character_index < alive_members.len() {
            // Find the index in the original members array
            let alive_member = alive_members[self.current_character_index];
            match self.current_team {
                TeamSide::Player => {
                    self.player_team.members.iter().position(|c| std::ptr::eq(c, alive_member))
                }
                TeamSide::Enemy => {
                    self.enemy_team.members.iter().position(|c| std::ptr::eq(c, alive_member))
                }
            }
        } else {
            None
        }
    }

    pub fn execute_turn(&mut self) {
        if self.battle_over {
            return;
        }

        // Check if character exists and get name early
        let character_name = match self.get_current_acting_character() {
            Some(character) => character.name.clone(),
            None => {
                self.advance_turn();
                return;
            }
        };

        // Validate action system exists
        let action_system_exists = match self.current_team {
            TeamSide::Player => self.current_character_index < self.player_action_systems.len(),
            TeamSide::Enemy => self.current_character_index < self.enemy_action_systems.len(),
        };

        if !action_system_exists {
            self.advance_turn();
            return;
        }

        // Get character reference manually to avoid double borrowing
        let acting_character = match self.current_team {
            TeamSide::Player => {
                let alive_members = self.player_team.alive_members();
                alive_members.get(self.current_character_index).copied().unwrap()
            }
            TeamSide::Enemy => {
                let alive_members = self.enemy_team.alive_members();
                alive_members.get(self.current_character_index).copied().unwrap()
            }
        };
        
        // Create battle context
        let battle_context = BattleContext::new(
            acting_character,
            self.current_team,
            &self.player_team,
            &self.enemy_team,
        );
        
        // Calculate action using the action system
        let action = match self.current_team {
            TeamSide::Player => {
                self.player_action_systems[self.current_character_index].calculate_action(&battle_context)
            }
            TeamSide::Enemy => {
                self.enemy_action_systems[self.current_character_index].calculate_action(&battle_context)
            }
        };

        // Execute action or log no action
        if let Some(action) = action {
            self.execute_action(action, character_name);
        } else {
            self.battle_log.push(format!(
                "ターン{}: {}は何もしなかった", 
                self.current_turn + 1, 
                character_name
            ));
        }

        self.check_battle_end();
        self.advance_turn();
    }

    fn advance_turn(&mut self) {
        let alive_count = match self.current_team {
            TeamSide::Player => self.player_team.alive_count(),
            TeamSide::Enemy => self.enemy_team.alive_count(),
        };

        self.current_character_index += 1;

        // If we've gone through all characters in current team, switch teams
        if self.current_character_index >= alive_count {
            self.current_character_index = 0;
            self.current_team = match self.current_team {
                TeamSide::Player => TeamSide::Enemy,
                TeamSide::Enemy => TeamSide::Player,
            };

            // If we're back to player team, increment turn counter
            if self.current_team == TeamSide::Player {
                self.current_turn += 1;
            }
        }
    }

    fn execute_action(&mut self, action: Box<dyn Action>, attacker_name: String) {
        // Create a battle context for action execution
        let acting_character = match self.get_current_acting_character() {
            Some(c) => c.clone(),
            None => return,
        };
        
        let battle_context = BattleContext::new(
            &acting_character,
            self.current_team,
            &self.player_team,
            &self.enemy_team
        );
        
        // Create battle state for action execution
        let mut battle_state = BattleState::new(self.player_team.clone(), self.enemy_team.clone());
        
        // Execute the action using the action trait
        match action.execute(&battle_context, &mut battle_state) {
            Ok(()) => {
                // Update teams with battle state changes
                self.player_team = battle_state.player_team;
                self.enemy_team = battle_state.enemy_team;
                
                // Add action-specific logs to battle log
                self.battle_log.extend(battle_state.battle_log);
            }
            Err(error_msg) => {
                self.battle_log.push(format!(
                    "ターン{}: {}の{}が失敗！ (理由: {})",
                    self.current_turn + 1,
                    attacker_name,
                    action.get_action_name(),
                    error_msg
                ));
            }
        }
    }

    fn check_battle_end(&mut self) {
        if self.player_team.alive_count() == 0 {
            self.battle_over = true;
            self.winner = Some(self.enemy_team.name.clone());
            self.battle_log.push(format!("{}の勝利！", self.enemy_team.name));
        } else if self.enemy_team.alive_count() == 0 {
            self.battle_over = true;
            self.winner = Some(self.player_team.name.clone());
            self.battle_log.push(format!("{}の勝利！", self.player_team.name));
        }
    }

    pub fn get_current_team_name(&self) -> &str {
        match self.current_team {
            TeamSide::Player => &self.player_team.name,
            TeamSide::Enemy => &self.enemy_team.name,
        }
    }

    pub fn get_recent_logs(&self, count: usize) -> Vec<&String> {
        self.battle_log.iter().rev().take(count).collect()
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use rand::{SeedableRng};
    use rand::rngs::StdRng;
    
    fn create_test_rng() -> StdRng {
        StdRng::seed_from_u64(42)
    }

    #[test]
    fn test_team_battle_creation() {
        let player_team = Team::new("Players".to_string(), vec![
            Character::new(1, "Hero".to_string(), 100, 50, 25),
            Character::new(2, "Warrior".to_string(), 120, 40, 30),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            Character::new(3, "Orc".to_string(), 150, 30, 20),
            Character::new(4, "Goblin".to_string(), 80, 40, 15),
        ]);
        
        let player_rules: Vec<Vec<RuleNode>> = vec![
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
        ];
        let enemy_rules: Vec<Vec<RuleNode>> = vec![
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
        ];
        
        let rng = create_test_rng();
        let team_battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        assert_eq!(team_battle.player_team.name, "Players");
        assert_eq!(team_battle.enemy_team.name, "Enemies");
        assert_eq!(team_battle.player_team.member_count(), 2);
        assert_eq!(team_battle.enemy_team.member_count(), 2);
        assert_eq!(team_battle.current_turn, 0);
        assert_eq!(team_battle.battle_over, false);
        assert_eq!(team_battle.is_player_turn(), true);
    }

    #[test]
    fn test_team_battle_turn_execution() {
        let player_team = Team::new("Players".to_string(), vec![
            Character::new(5, "Hero".to_string(), 100, 50, 25),
            Character::new(6, "Warrior".to_string(), 120, 40, 30),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            Character::new(7, "Orc".to_string(), 150, 30, 20),
            Character::new(8, "Goblin".to_string(), 80, 40, 15),
        ]);
        
        let player_rules: Vec<Vec<RuleNode>> = vec![
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
        ];
        let enemy_rules: Vec<Vec<RuleNode>> = vec![
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
        ];
        
        let rng = create_test_rng();
        let mut team_battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        // Execute first player turn
        assert_eq!(team_battle.current_team, TeamSide::Player);
        assert_eq!(team_battle.current_character_index, 0);
        
        team_battle.execute_turn();
        
        // Should advance to second player character
        assert_eq!(team_battle.current_team, TeamSide::Player);
        assert_eq!(team_battle.current_character_index, 1);
        
        team_battle.execute_turn();
        
        // Should switch to enemy team
        assert_eq!(team_battle.current_team, TeamSide::Enemy);
        assert_eq!(team_battle.current_character_index, 0);
    }

    #[test]
    fn test_team_battle_complete_round() {
        let player_team = Team::new("Players".to_string(), vec![
            Character::new(9, "Hero".to_string(), 100, 50, 25),
            Character::new(10, "Warrior".to_string(), 120, 40, 30),
        ]);
        let enemy_team = Team::new("Enemies".to_string(), vec![
            Character::new(11, "Orc".to_string(), 150, 30, 20),
            Character::new(12, "Goblin".to_string(), 80, 40, 15),
        ]);
        
        let player_rules: Vec<Vec<RuleNode>> = vec![
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
        ];
        let enemy_rules: Vec<Vec<RuleNode>> = vec![
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
            vec![Box::new(action_system::StrikeActionNode::new(Box::new(action_system::ActingCharacterNode)))],
        ];
        
        let rng = create_test_rng();
        let mut team_battle = TeamBattle::new(player_team, enemy_team, player_rules, enemy_rules, rng);
        
        let initial_turn = team_battle.current_turn;
        
        // Execute a full round (2 player + 2 enemy actions)
        for _ in 0..4 {
            team_battle.execute_turn();
        }
        
        // Should be back to player team with incremented turn
        assert_eq!(team_battle.current_team, TeamSide::Player, "Should switch back to player team");
        assert_eq!(team_battle.current_turn, initial_turn + 1, "Turn counter should increment");
    }

}