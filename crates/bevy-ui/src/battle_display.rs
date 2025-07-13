// Battle display formatting logic
use battle::TeamBattle;

pub fn format_battle_display(battle: &TeamBattle) -> String {
    let mut display_text = String::new();
    
    display_text.push_str(&format!("=== チーム戦闘 (ターン {}) ===\n", battle.current_turn + 1));
    
    // プレイヤーチーム情報
    display_text.push_str(&format!("\n【{}】\n", battle.player_team.name));
    for member in &battle.player_team.members {
        let status = if member.is_alive() { "生存" } else { "戦闘不能" };
        display_text.push_str(&format!("  {} - HP:{}/{} MP:{}/{} ({})\n", 
            member.name, member.hp, member.max_hp, member.mp, member.max_mp, status));
    }
    
    // 敵チーム情報
    display_text.push_str(&format!("\n【{}】\n", battle.enemy_team.name));
    for member in &battle.enemy_team.members {
        let status = if member.is_alive() { "生存" } else { "戦闘不能" };
        display_text.push_str(&format!("  {} - HP:{}/{} MP:{}/{} ({})\n", 
            member.name, member.hp, member.max_hp, member.mp, member.max_mp, status));
    }
    
    // 現在のターン情報
    if !battle.battle_over {
        if let Some(current_character) = battle.get_current_acting_character() {
            display_text.push_str(&format!("\n現在の行動キャラクター: {} ({})\n", 
                current_character.name, battle.get_current_team_name()));
            display_text.push_str("スペースキーでターン実行\n");
        }
    } else {
        if let Some(winner) = &battle.winner {
            display_text.push_str(&format!("\n🎉 {} の勝利！\n", winner));
            display_text.push_str("Shiftキーでリセット\n");
        }
    }
    
    display_text
}

pub fn format_latest_log(battle: &TeamBattle) -> String {
    if let Some(latest_log) = battle.battle_log.last() {
        format!(">>> {}", latest_log)
    } else {
        "チーム戦闘開始！スペースキーでターン実行".to_string()
    }
}