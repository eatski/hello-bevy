// Character nodes - nodes that evaluate to character references for calculations

// Team battle context struct for team-based battles
#[derive(Debug)]
pub struct BattleContext<'a> {
    pub acting_character: &'a crate::Character,
    pub acting_team: crate::TeamSide,
    pub player_team: &'a crate::Team,
    pub enemy_team: &'a crate::Team,
}

impl<'a> BattleContext<'a> {
    // Constructor for team battles
    pub fn new(
        acting_character: &'a crate::Character,
        acting_team: crate::TeamSide,
        player_team: &'a crate::Team,
        enemy_team: &'a crate::Team,
    ) -> Self {
        Self {
            acting_character,
            acting_team,
            player_team,
            enemy_team,
        }
    }
    
    pub fn all_characters(&self) -> Vec<&'a crate::Character> {
        let mut characters = Vec::new();
        characters.extend(self.player_team.members.iter());
        characters.extend(self.enemy_team.members.iter());
        characters
    }

    pub fn get_team_members(&self, team: crate::TeamSide) -> Vec<&'a crate::Character> {
        match team {
            crate::TeamSide::Player => self.player_team.members.iter().collect(),
            crate::TeamSide::Enemy => self.enemy_team.members.iter().collect(),
        }
    }

    pub fn get_alive_team_members(&self, team: crate::TeamSide) -> Vec<&'a crate::Character> {
        match team {
            crate::TeamSide::Player => self.player_team.alive_members(),
            crate::TeamSide::Enemy => self.enemy_team.alive_members(),
        }
    }

    pub fn get_opposing_team(&self, team: crate::TeamSide) -> crate::TeamSide {
        match team {
            crate::TeamSide::Player => crate::TeamSide::Enemy,
            crate::TeamSide::Enemy => crate::TeamSide::Player,
        }
    }
    
    pub fn get_acting_character(&self) -> &'a crate::Character {
        self.acting_character
    }

    pub fn get_acting_team(&self) -> crate::TeamSide {
        self.acting_team
    }


    // ID-based character lookup
    pub fn get_character_by_id(&self, id: i32) -> Option<&'a crate::Character> {
        self.player_team.get_member_by_id(id)
            .or_else(|| self.enemy_team.get_member_by_id(id))
    }
}

