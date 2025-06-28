use bevy::prelude::*;
use battle::{Battle, RuleNode};
use crate::ui_converter::{UITokenType, convert_ui_rules_to_nodes};

#[derive(Resource)]
pub struct GameFont {
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct GameBattle(pub Battle);

#[derive(Component)]
pub struct BattleUI;

#[derive(Component)]
pub struct LogUI;

#[derive(Component)]
pub struct LatestLogUI;

#[derive(Component)]
pub struct HPBar;

#[derive(Component)]
pub struct MPBar;

// ルール編集UI用のコンポーネント
#[derive(Component)]
pub struct RuleEditor;

#[derive(Component)]
pub struct TokenInventory;

#[derive(Component)]
pub struct InstructionUI;

#[derive(Component)]
pub struct TokenSelectionHeader;

#[derive(Component)]
pub struct BattleInfo;


// 現在設定されているルール（キー操作用）
#[derive(Resource, Default)]
pub struct CurrentRules {
    pub rules: Vec<Vec<UITokenType>>,
    pub selected_row: usize,
}

impl CurrentRules {
    pub fn new() -> Self {
        Self {
            rules: vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            selected_row: 0,
        }
    }

    // UIのUITokenTypeからrule-systemを経由してaction-systemのRuleNodeに変換
    pub fn convert_to_rule_nodes(&self) -> Vec<RuleNode> {
        convert_ui_rules_to_nodes(&self.rules)
    }
}

// ルールトークンを整形した文字列に変換
fn format_rule_tokens(rule_row: &[UITokenType]) -> String {
    if rule_row.is_empty() {
        "(空)".to_string()
    } else {
        rule_row.iter()
            .map(|token| token.display_text())
            .collect::<Vec<_>>()
            .join(" → ")
    }
}

// ゲーム全体の状態管理
#[derive(Resource, Default)]
pub struct GameState {
    pub mode: GameMode,
}

#[derive(Default, PartialEq)]
pub enum GameMode {
    #[default]
    RuleCreation, // ルール作成モード
    Battle,       // 戦闘モード
}

// メニュー選択の状態管理
#[derive(Resource, Default)]
pub struct MenuState {
    pub mode: MenuMode,
    pub selected_row: usize,
    pub selected_token: usize,
    pub available_tokens: Vec<UITokenType>,
}

#[derive(Default, PartialEq)]
pub enum MenuMode {
    #[default]
    RowSelection,  // 行選択モード
    TokenSelection, // トークン選択モード
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            mode: MenuMode::RowSelection,
            selected_row: 0,
            selected_token: 0,
            available_tokens: vec![
                UITokenType::Check,
                UITokenType::Strike,
                UITokenType::Heal,
                UITokenType::TrueOrFalse,
                UITokenType::GreaterThan,
                UITokenType::Number(50),
                UITokenType::HP,
            ],
        }
    }
}

pub fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSansCJK-Regular.ttc");
    commands.insert_resource(GameFont { font });
}

pub fn setup_ui(
    mut commands: Commands, 
    game_font: Res<GameFont>,
) {
    commands.spawn(Camera2d);
    commands.insert_resource(GameState::default());
    commands.insert_resource(CurrentRules::new());
    commands.insert_resource(MenuState::new());
    
    // ルール編集エリア（左側）
    setup_rule_editor(&mut commands, &game_font);
    
    // インベントリエリア（右側）
    setup_inventory(&mut commands, &game_font);
    
    // 操作説明表示（上部）
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(20.0),
            ..default()
        },
        InstructionUI,
    ));
    
    // バトル情報表示（下部）
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        BattleUI,
    ));
    
    // ログ表示（下部右側）
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            width: Val::Px(300.0),
            ..default()
        },
        LogUI,
    ));
    
    // 最新ログ表示（中央上部）
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(50.0),
            right: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        LatestLogUI,
    ));
}

fn setup_rule_editor(commands: &mut Commands, game_font: &GameFont) {
    // Rule ヘッダー
    commands.spawn((
        Text::new("Rule"),
        TextFont {
            font: game_font.font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
    
    // ルール表示エリア
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(110.0),
            left: Val::Px(20.0),
            width: Val::Px(600.0),
            height: Val::Px(400.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        RuleEditor,
    ));
}

fn setup_inventory(commands: &mut Commands, game_font: &GameFont) {
    // Token Selection ヘッダー
    commands.spawn((
        Text::new("Token Selection"),
        TextFont {
            font: game_font.font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            right: Val::Px(20.0),
            ..default()
        },
        TokenSelectionHeader,
    ));
    
    // トークン選択メニューエリア
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(110.0),
            right: Val::Px(20.0),
            width: Val::Px(300.0),
            height: Val::Px(400.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        TokenInventory,
    ));
    
    // 戦闘情報表示エリア（戦闘モード時のみ表示）
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(110.0),
            right: Val::Px(20.0),
            width: Val::Px(300.0),
            height: Val::Px(400.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.3, 0.4)),
        BattleInfo,
    ));
}

pub fn handle_battle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_battle: ResMut<GameBattle>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Battle || game_battle.0.battle_over {
        return;
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_battle.0.is_player_turn() {
            game_battle.0.execute_player_action();
        } else {
            game_battle.0.execute_enemy_action();
        }
    }
}





pub fn update_battle_ui(
    game_state: Res<GameState>,
    game_battle: Res<GameBattle>,
    mut ui_query: Query<&mut Text, With<BattleUI>>,
) {
    for mut text in ui_query.iter_mut() {
        match game_state.mode {
            GameMode::RuleCreation => {
                text.0 = "ルール作成中...\nスペースキーで戦闘開始".to_string();
            }
            GameMode::Battle => {
                let battle = &game_battle.0;
                let mut display_text = String::new();
                
                display_text.push_str(&format!("{}:\n", battle.player.name));
                display_text.push_str(&format!("HP {}/{}\n", battle.player.hp, battle.player.max_hp));
                display_text.push_str(&format!("MP {}/{}\n", battle.player.mp, battle.player.max_mp));
                
                display_text.push_str("\n");
                
                display_text.push_str(&format!("{}:\n", battle.enemy.name));
                display_text.push_str(&format!("HP {}/{}\n", battle.enemy.hp, battle.enemy.max_hp));
                display_text.push_str(&format!("MP {}/{}\n", battle.enemy.mp, battle.enemy.max_mp));
                
                display_text.push_str("\n");
                
                if battle.battle_over {
                    if let Some(winner) = &battle.winner {
                        display_text.push_str(&format!("バトル終了！{}の勝利！\n", winner));
                        display_text.push_str("Shiftキーで戦闘リセット！\n");
                    }
                } else {
                    display_text.push_str(&format!("{}のターン\n", battle.get_current_player_name()));
                    if battle.is_player_turn() {
                        display_text.push_str("スペースキーで行動！\n");
                    }
                }
                
                text.0 = display_text;
            }
        }
    }
}

pub fn update_log_ui(
    game_state: Res<GameState>,
    game_battle: Res<GameBattle>,
    mut log_query: Query<&mut Text, (With<LogUI>, Without<BattleUI>, Without<LatestLogUI>)>
) {
    for mut text in log_query.iter_mut() {
        match game_state.mode {
            GameMode::RuleCreation => {
                text.0 = "ルール作成中...\n戦闘開始後にログが表示されます".to_string();
            }
            GameMode::Battle => {
                let battle = &game_battle.0;
                let mut log_text = String::from("戦闘ログ:\n");
                
                for log in battle.get_recent_logs(8) {
                    log_text.push_str(&format!("{}\n", log));
                }
                
                text.0 = log_text;
            }
        }
    }
}

pub fn update_latest_log_ui(
    game_state: Res<GameState>,
    game_battle: Res<GameBattle>,
    mut latest_log_query: Query<&mut Text, (With<LatestLogUI>, Without<BattleUI>, Without<LogUI>)>
) {
    for mut text in latest_log_query.iter_mut() {
        match game_state.mode {
            GameMode::RuleCreation => {
                text.0 = "ルール作成モード：トークンを組み合わせて行動ルールを作成してください".to_string();
            }
            GameMode::Battle => {
                let battle = &game_battle.0;
                
                if let Some(latest_log) = battle.battle_log.last() {
                    text.0 = format!(">>> {}", latest_log);
                } else {
                    text.0 = "戦闘開始！".to_string();
                }
            }
        }
    }
}

// 戦闘リセット機能（ルール作成モードに戻る）
pub fn handle_battle_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut menu_state: ResMut<MenuState>,
) {
    if game_state.mode == GameMode::Battle &&
       (keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight)) {
        // ルール作成モードに戻る
        game_state.mode = GameMode::RuleCreation;
        menu_state.mode = MenuMode::RowSelection;
        menu_state.selected_row = 0;
        menu_state.selected_token = 0;
    }
}

// 矢印キーとEnterでのメニュー操作システム
pub fn handle_rule_editing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut menu_state: ResMut<MenuState>,
    mut current_rules: ResMut<CurrentRules>,
) {
    // ルール作成モードの時のみ処理
    if game_state.mode != GameMode::RuleCreation {
        return;
    }

    match menu_state.mode {
        MenuMode::RowSelection => {
            // 行選択モード
            if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                if menu_state.selected_row > 0 {
                    menu_state.selected_row -= 1;
                }
            } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                if menu_state.selected_row < 4 {
                    menu_state.selected_row += 1;
                }
            } else if keyboard_input.just_pressed(KeyCode::Enter) {
                // トークン選択モードに切り替え
                menu_state.mode = MenuMode::TokenSelection;
                menu_state.selected_token = 0;
                current_rules.selected_row = menu_state.selected_row;
            } else if keyboard_input.just_pressed(KeyCode::Backspace) {
                // 選択中の行の最後のトークンを削除
                current_rules.rules[menu_state.selected_row].pop();
            } else if keyboard_input.just_pressed(KeyCode::Space) {
                // ルール作成完了 → 戦闘モードに移行
                game_state.mode = GameMode::Battle;
                // メニュー状態をリセット
                menu_state.mode = MenuMode::RowSelection;
                menu_state.selected_row = 0;
                menu_state.selected_token = 0;
            }
        }
        MenuMode::TokenSelection => {
            // トークン選択モード
            if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                if menu_state.selected_token > 0 {
                    menu_state.selected_token -= 1;
                }
            } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                if menu_state.selected_token < menu_state.available_tokens.len() - 1 {
                    menu_state.selected_token += 1;
                }
            } else if keyboard_input.just_pressed(KeyCode::Enter) {
                // 選択されたトークンを追加
                if let Some(token) = menu_state.available_tokens.get(menu_state.selected_token) {
                    let selected_row = current_rules.selected_row;
                    current_rules.rules[selected_row].push(token.clone());
                    // 行選択モードに戻る
                    menu_state.mode = MenuMode::RowSelection;
                }
            } else if keyboard_input.just_pressed(KeyCode::Backspace) {
                // 行選択モードに戻る（キャンセル）
                menu_state.mode = MenuMode::RowSelection;
            }
        }
    }
}

pub fn update_rule_display(
    game_state: Res<GameState>,
    menu_state: Res<MenuState>,
    current_rules: Res<CurrentRules>,
    mut rule_query: Query<&mut Text, With<RuleEditor>>,
) {
    for mut text in rule_query.iter_mut() {
        let mut display_text = String::new();
        
        match game_state.mode {
            GameMode::RuleCreation => {
                // ルール作成モード表示
                match menu_state.mode {
                    MenuMode::RowSelection => {
                        display_text.push_str("【ルール作成モード - 行選択】\n");
                        display_text.push_str("↑↓: 行選択  Enter: トークン追加  Backspace: 削除  スペース: 戦闘開始\n\n");
                    }
                    MenuMode::TokenSelection => {
                        display_text.push_str("【ルール作成モード - トークン選択】\n");
                        display_text.push_str("↑↓: トークン選択  Enter: 追加  Backspace: キャンセル\n\n");
                    }
                }
                
                // ルール表示
                for (i, rule_row) in current_rules.rules.iter().enumerate() {
                    let prefix = if menu_state.mode == MenuMode::RowSelection && i == menu_state.selected_row {
                        format!("▶ 行{}: ", i + 1)
                    } else {
                        format!("  行{}: ", i + 1)
                    };
                    
                    display_text.push_str(&prefix);
                    display_text.push_str(&format_rule_tokens(rule_row));
                    
                    display_text.push('\n');
                }
            }
            GameMode::Battle => {
                // 戦闘モード表示
                display_text.push_str("【戦闘モード】\n");
                display_text.push_str("設定されたルール:\n\n");
                
                for (i, rule_row) in current_rules.rules.iter().enumerate() {
                    display_text.push_str(&format!("行{}: {}\n", i + 1, format_rule_tokens(rule_row)));
                }
            }
        }
        
        text.0 = display_text;
    }
}

pub fn update_token_inventory_display(
    game_state: Res<GameState>,
    menu_state: Res<MenuState>,
    mut inventory_query: Query<&mut Text, With<TokenInventory>>,
) {
    for mut text in inventory_query.iter_mut() {
        let mut display_text = String::new();
        
        match game_state.mode {
            GameMode::RuleCreation => {
                match menu_state.mode {
                    MenuMode::RowSelection => {
                        display_text.push_str("行を選択してEnterを押すと\nトークン選択モードになります\n\n");
                        display_text.push_str("スペースキーを押すと\n戦闘開始できます");
                    }
                    MenuMode::TokenSelection => {
                        display_text.push_str("トークンを選択してください:\n\n");
                        
                        for (i, token) in menu_state.available_tokens.iter().enumerate() {
                            let prefix = if i == menu_state.selected_token {
                                "▶ "
                            } else {
                                "  "
                            };
                            
                            display_text.push_str(&format!("{}{}\n", prefix, token.display_text()));
                        }
                    }
                }
            }
            GameMode::Battle => {
                // 戦闘モードでは非表示（空文字）
                display_text = String::new();
            }
        }
        
        text.0 = display_text;
    }
}

// Token Selection ヘッダーとBattle Infoの表示を切り替える
pub fn update_right_panel_visibility(
    game_state: Res<GameState>,
    mut header_query: Query<&mut Node, (With<TokenSelectionHeader>, Without<TokenInventory>, Without<BattleInfo>)>,
    mut inventory_query: Query<&mut Node, (With<TokenInventory>, Without<TokenSelectionHeader>, Without<BattleInfo>)>,
    mut battle_info_query: Query<&mut Node, (With<BattleInfo>, Without<TokenSelectionHeader>, Without<TokenInventory>)>,
) {
    match game_state.mode {
        GameMode::RuleCreation => {
            // ルール作成モード：Token Selectionを表示、Battle Infoを非表示
            for mut node in header_query.iter_mut() {
                node.display = Display::Flex;
            }
            for mut node in inventory_query.iter_mut() {
                node.display = Display::Flex;
            }
            for mut node in battle_info_query.iter_mut() {
                node.display = Display::None;
            }
        }
        GameMode::Battle => {
            // 戦闘モード：Token Selectionを非表示、Battle Infoを表示
            for mut node in header_query.iter_mut() {
                node.display = Display::None;
            }
            for mut node in inventory_query.iter_mut() {
                node.display = Display::None;
            }
            for mut node in battle_info_query.iter_mut() {
                node.display = Display::Flex;
            }
        }
    }
}

// 戦闘情報表示の更新
pub fn update_battle_info_display(
    game_state: Res<GameState>,
    game_battle: Res<GameBattle>,
    current_rules: Res<CurrentRules>,
    mut battle_info_query: Query<&mut Text, With<BattleInfo>>,
) {
    if game_state.mode != GameMode::Battle {
        return;
    }
    
    for mut text in battle_info_query.iter_mut() {
        let battle = &game_battle.0;
        let mut display_text = String::new();
        
        display_text.push_str("=== 戦闘情報 ===\n\n");
        
        // キャラクター情報
        display_text.push_str(&format!("{}:\n", battle.player.name));
        display_text.push_str(&format!("  HP: {}/{}\n", battle.player.hp, battle.player.max_hp));
        display_text.push_str(&format!("  MP: {}/{}\n\n", battle.player.mp, battle.player.max_mp));
        
        display_text.push_str(&format!("{}:\n", battle.enemy.name));
        display_text.push_str(&format!("  HP: {}/{}\n", battle.enemy.hp, battle.enemy.max_hp));
        display_text.push_str(&format!("  MP: {}/{}\n\n", battle.enemy.mp, battle.enemy.max_mp));
        
        // 設定ルール
        display_text.push_str("設定ルール:\n");
        for (i, rule_row) in current_rules.rules.iter().enumerate() {
            if !rule_row.is_empty() {
                display_text.push_str(&format!("{}. {}\n", i + 1, format_rule_tokens(rule_row)));
            }
        }
        
        text.0 = display_text;
    }
}

// UI表示を更新（モードに応じて）
pub fn update_instruction_display(
    game_state: Res<GameState>,
    mut instruction_query: Query<&mut Text, With<InstructionUI>>,
) {
    for mut text in instruction_query.iter_mut() {
        match game_state.mode {
            GameMode::RuleCreation => {
                text.0 = "【ルール作成モード】 ↑↓: 選択  Enter: 決定  Backspace: 削除  スペース: 戦闘開始".to_string();
            }
            GameMode::Battle => {
                text.0 = "【戦闘モード】 スペース: 行動実行  Shift: 戦闘リセット（ルール作成に戻る）".to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_greater_than_number_hp_pattern() {
        // Test: Check → GreaterThan → Number(50) → HP → Strike
        let mut current_rules = CurrentRules::new();
        current_rules.rules[0] = vec![
            UITokenType::Check,
            UITokenType::GreaterThan,
            UITokenType::Number(50),
            UITokenType::HP,
            UITokenType::Strike,
        ];
        
        let rule_nodes = current_rules.convert_to_rule_nodes();
        assert_eq!(rule_nodes.len(), 1, "Check → GreaterThan → Number → HP → Strike should be valid");
    }
    
    #[test]
    fn test_all_valid_hp_patterns() {
        // Test: Check → GreaterThan → HP → Number → Strike (HP > Number)
        let mut current_rules1 = CurrentRules::new();
        current_rules1.rules[0] = vec![
            UITokenType::Check,
            UITokenType::GreaterThan,
            UITokenType::HP,
            UITokenType::Number(50),
            UITokenType::Strike,
        ];
        
        let rule_nodes1 = current_rules1.convert_to_rule_nodes();
        assert_eq!(rule_nodes1.len(), 1, "Check → GreaterThan → HP → Number → Strike should be valid (HP > Number)");
        
        // Test: Check → GreaterThan → Number → HP → Strike (Number > HP)
        let mut current_rules2 = CurrentRules::new();
        current_rules2.rules[0] = vec![
            UITokenType::Check,
            UITokenType::GreaterThan,
            UITokenType::Number(50),
            UITokenType::HP,
            UITokenType::Strike,
        ];
        
        let rule_nodes2 = current_rules2.convert_to_rule_nodes();
        assert_eq!(rule_nodes2.len(), 1, "Check → GreaterThan → Number → HP → Strike should be valid (Number > HP)");
    }
    
    
    
    
    #[test]
    fn test_all_valid_greater_than_patterns() {
        // Pattern 1: Number > HP
        let mut current_rules1 = CurrentRules::new();
        current_rules1.rules[0] = vec![
            UITokenType::Check,
            UITokenType::GreaterThan,
            UITokenType::Number(30),
            UITokenType::HP,
            UITokenType::Heal,
        ];
        
        // Pattern 2: HP > Number
        let mut current_rules2 = CurrentRules::new();
        current_rules2.rules[0] = vec![
            UITokenType::Check,
            UITokenType::GreaterThan,
            UITokenType::HP,
            UITokenType::Number(30),
            UITokenType::Heal,
        ];
        
        // Both patterns should be valid
        let rule_nodes1 = current_rules1.convert_to_rule_nodes();
        let rule_nodes2 = current_rules2.convert_to_rule_nodes();
        assert_eq!(rule_nodes1.len(), 1, "Number > HP pattern should convert successfully");
        assert_eq!(rule_nodes2.len(), 1, "HP > Number pattern should convert successfully");
    }
    
}