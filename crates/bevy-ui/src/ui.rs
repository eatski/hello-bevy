use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use battle::TeamBattle;
use ui_core::{GameState, GameMode, CurrentRules, FlatTokenInput};
use crate::display_text::format_rule_tokens;

#[derive(Resource)]
pub struct GameFont {
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct GameTeamBattle(pub TeamBattle);

#[derive(Component)]
pub struct BattleUI;


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
pub struct RuleDisplay;

#[derive(Component)]
pub struct BattleRuleDisplay;

#[derive(Component)]
pub struct BattleRuleText;

#[derive(Component)]
pub struct MainContentArea;

#[derive(Component)]
pub struct TokenInventory;

#[derive(Component)]
pub struct InstructionUI;

#[derive(Component)]
pub struct TokenSelectionHeader;


#[derive(Component)]
pub struct TeamBattleUI;

#[derive(Component)]
pub struct TeamDisplay;

#[derive(Component)]
pub struct TeamMemberDisplay;


// Bevy Resource wrappers for ui-core types
#[derive(Resource)]
pub struct BevyCurrentRules(pub CurrentRules);

impl Default for BevyCurrentRules {
    fn default() -> Self {
        Self(CurrentRules::new())
    }
}

#[derive(Resource)]
pub struct BevyGameState(pub GameState);

impl Default for BevyGameState {
    fn default() -> Self {
        Self(GameState::new())
    }
}

// メニュー選択の状態管理
#[derive(Resource, Default)]
pub struct MenuState {
    pub mode: MenuMode,
    pub selected_row: usize,
    pub selected_token: usize,
    pub available_tokens: Vec<FlatTokenInput>,
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
                FlatTokenInput::Check,
                FlatTokenInput::Strike,
                FlatTokenInput::Heal,
                FlatTokenInput::TrueOrFalse,
                FlatTokenInput::GreaterThan,
                FlatTokenInput::Number(50),
                FlatTokenInput::CharacterToHp,
                FlatTokenInput::CharacterHpToCharacter,
                FlatTokenInput::ActingCharacter,
                FlatTokenInput::AllCharacters,
                FlatTokenInput::TeamMembers,
                FlatTokenInput::RandomPick,
                FlatTokenInput::FilterList,
                FlatTokenInput::Map,
                FlatTokenInput::Eq,
                FlatTokenInput::CharacterTeam,
                FlatTokenInput::Element,
                FlatTokenInput::Enemy,
                FlatTokenInput::Hero,
                FlatTokenInput::Max,
                FlatTokenInput::Min,
                FlatTokenInput::NumericMax,
                FlatTokenInput::NumericMin,
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
    commands.insert_resource(BevyGameState::default());
    commands.insert_resource(BevyCurrentRules::default());
    commands.insert_resource(MenuState::new());
    
    // メインコンテナ
    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
    )).with_children(|parent| {
        // ヘッダー（操作説明表示）
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BorderRadius::all(Val::Px(4.0)),
        )).with_children(|header| {
            header.spawn((
                Text::new(""),
                TextFont {
                    font: game_font.font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                InstructionUI,
            ));
        });
        
        // 最新ログ表示
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.0)),
            BorderColor(Color::srgb(0.8, 0.8, 0.0)),
            BorderRadius::all(Val::Px(4.0)),
        )).with_children(|log| {
            log.spawn((
                Text::new(""),
                TextFont {
                    font: game_font.font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                LatestLogUI,
            ));
        });
        
        // メインコンテンツエリア
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(200.0), // 最小高さを設定
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            MainContentArea, // 戦闘モード時の表示制御用コンポーネント
        )).with_children(|main| {
            // 左側：ルールエディタ（戦闘モード時は右側に移動）
            main.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(0.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                BorderRadius::all(Val::Px(8.0)),
                RuleEditor, // コンテナ全体にRuleEditorコンポーネントを付与
            )).with_children(|parent| {
                // Rule ヘッダー
                parent.spawn((
                    Text::new("ルール"),
                    TextFont {
                        font: game_font.font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                ));
                
                // ルール表示エリア
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: game_font.font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        padding: UiRect::all(Val::Px(15.0)),
                        flex_grow: 1.0,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                    RuleDisplay, // 実際のルール表示エリア用の新しいコンポーネント
                ));
            });
            
            // 右側：トークンインベントリ（選択中のみ表示）
            main.spawn((
                Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(0.0)),
                    flex_direction: FlexDirection::Column,
                    display: Display::None, // 初期状態では非表示
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                BorderRadius::all(Val::Px(8.0)),
                TokenSelectionHeader, // コンテナ全体にヘッダーコンポーネントを付与
            )).with_children(|parent| {
                // Token Selection ヘッダー
                parent.spawn((
                    Text::new("トークン選択"),
                    TextFont {
                        font: game_font.font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                ));
                
                // トークン選択メニューエリア
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: game_font.font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        padding: UiRect::all(Val::Px(15.0)),
                        flex_grow: 1.0,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                    TokenInventory,
                ));
            });
        });
        
        // フッター（バトル情報表示 + 戦闘モード時のルール表示）
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BorderRadius::all(Val::Px(4.0)),
        )).with_children(|footer| {
            // バトル情報表示
            footer.spawn((
                Text::new(""),
                TextFont {
                    font: game_font.font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                BattleUI,
            ));
            
            // 戦闘モード時のルール表示
            footer.spawn((
                Node {
                    width: Val::Percent(40.0),
                    padding: UiRect::all(Val::Px(0.0)),
                    flex_direction: FlexDirection::Column,
                    display: Display::None, // 初期状態では非表示
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                BorderRadius::all(Val::Px(8.0)),
                BattleRuleDisplay, // 戦闘モード用のルール表示コンポーネント
            )).with_children(|rule_parent| {
                // ルールヘッダー
                rule_parent.spawn((
                    Text::new("設定済みルール"),
                    TextFont {
                        font: game_font.font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                ));
                
                // ルール内容
                rule_parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: game_font.font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_grow: 1.0,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                    BattleRuleText, // 戦闘モード用のルールテキスト
                ));
            });
        });
    });
}









// 戦闘リセット機能（ルール作成モードに戻る）
pub fn handle_battle_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<BevyGameState>,
    mut menu_state: ResMut<MenuState>,
) {
    if game_state.0.mode == GameMode::Battle &&
       (keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight)) {
        // ルール作成モードに戻る
        game_state.0.mode = GameMode::RuleCreation;
        menu_state.mode = MenuMode::RowSelection;
        menu_state.selected_row = 0;
        menu_state.selected_token = 0;
    }
}

// スクリーンショット機能
pub fn handle_screenshot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        let path = format!("screenshots/screenshot_{}.png", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());
        
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}


// 矢印キーとEnterでのメニュー操作システム
pub fn handle_rule_editing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<BevyGameState>,
    mut menu_state: ResMut<MenuState>,
    mut current_rules: ResMut<BevyCurrentRules>,
) {
    // ルール作成モードの時のみ処理
    if game_state.0.mode != GameMode::RuleCreation {
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
                current_rules.0.selected_row = menu_state.selected_row;
            } else if keyboard_input.just_pressed(KeyCode::Backspace) {
                // 選択中の行の最後のトークンを削除
                current_rules.0.rules[menu_state.selected_row].pop();
            } else if keyboard_input.just_pressed(KeyCode::Space) {
                // ルール作成完了 → 戦闘モードに移行
                game_state.0.mode = GameMode::Battle;
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
                    let selected_row = current_rules.0.selected_row;
                    current_rules.0.rules[selected_row].push(token.clone());
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
    game_state: Res<BevyGameState>,
    menu_state: Res<MenuState>,
    current_rules: Res<BevyCurrentRules>,
    mut rule_query: Query<&mut Text, With<RuleDisplay>>,
) {
    for mut text in rule_query.iter_mut() {
        let mut display_text = String::new();
        
        match game_state.0.mode {
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
                for (i, rule_row) in current_rules.0.rules.iter().enumerate() {
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
                // 戦闘モードでは表示しない（BattleRuleTextで表示）
                display_text = String::new();
            }
        }
        
        text.0 = display_text;
    }
}

// 戦闘モードでのルール表示
pub fn update_battle_rule_display(
    current_rules: Res<BevyCurrentRules>,
    mut battle_rule_query: Query<&mut Text, With<BattleRuleText>>,
) {
    for mut text in battle_rule_query.iter_mut() {
        let mut display_text = String::new();
        
        for (i, rule_row) in current_rules.0.rules.iter().enumerate() {
            display_text.push_str(&format!("行{}: {}\n", i + 1, format_rule_tokens(rule_row)));
        }
        
        text.0 = display_text;
    }
}

pub fn update_token_inventory_display(
    game_state: Res<BevyGameState>,
    menu_state: Res<MenuState>,
    mut inventory_query: Query<&mut Text, With<TokenInventory>>,
) {
    for mut text in inventory_query.iter_mut() {
        let mut display_text = String::new();
        
        match game_state.0.mode {
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

// Token Selection パネルの表示を切り替える（選択中のみ表示）
pub fn update_right_panel_visibility(
    game_state: Res<BevyGameState>,
    menu_state: Res<MenuState>,
    mut header_query: Query<&mut Node, With<TokenSelectionHeader>>,
) {
    match game_state.0.mode {
        GameMode::RuleCreation => {
            // ルール作成モード：トークン選択モードの時のみ表示
            for mut node in header_query.iter_mut() {
                node.display = match menu_state.mode {
                    MenuMode::TokenSelection => Display::Flex,
                    MenuMode::RowSelection => Display::None,
                };
            }
        }
        GameMode::Battle => {
            // 戦闘モード：Token Selectionを非表示
            for mut node in header_query.iter_mut() {
                node.display = Display::None;
            }
        }
    }
}

// ルールエディタの位置を戦闘モードに応じて調整
pub fn update_rule_editor_position(
    game_state: Res<BevyGameState>,
    mut rule_editor_query: Query<&mut Node, With<RuleEditor>>,
    mut battle_rule_query: Query<&mut Node, (With<BattleRuleDisplay>, Without<RuleEditor>, Without<MainContentArea>)>,
    mut main_content_query: Query<&mut Node, (With<MainContentArea>, Without<RuleEditor>, Without<BattleRuleDisplay>)>,
) {
    match game_state.0.mode {
        GameMode::RuleCreation => {
            // ルール作成モード：メインエリアにルールエディタを表示
            for mut node in rule_editor_query.iter_mut() {
                node.display = Display::Flex;
            }
            // メインコンテンツエリアを表示
            for mut node in main_content_query.iter_mut() {
                node.display = Display::Flex;
                node.min_height = Val::Px(200.0);
            }
            // バトルルール表示を非表示
            for mut node in battle_rule_query.iter_mut() {
                node.display = Display::None;
            }
        }
        GameMode::Battle => {
            // 戦闘モード：メインエリアのルールエディタを非表示
            for mut node in rule_editor_query.iter_mut() {
                node.display = Display::None;
            }
            // メインコンテンツエリアを小さく
            for mut node in main_content_query.iter_mut() {
                node.display = Display::None; // 戦闘モードでは完全に非表示
            }
            // フッターにバトルルール表示を表示
            for mut node in battle_rule_query.iter_mut() {
                node.display = Display::Flex;
            }
        }
    }
}


// UI表示を更新（モードに応じて）
pub fn update_instruction_display(
    game_state: Res<BevyGameState>,
    mut instruction_query: Query<&mut Text, With<InstructionUI>>,
) {
    for mut text in instruction_query.iter_mut() {
        match game_state.0.mode {
            GameMode::RuleCreation => {
                text.0 = "【ルール作成モード】 ↑↓: 選択  Enter: 決定  Backspace: 削除  スペース: 戦闘開始  S: スクリーンショット".to_string();
            }
            GameMode::Battle => {
                text.0 = "【戦闘モード】 スペース: 行動実行  Shift: 戦闘リセット（ルール作成に戻る）  S: スクリーンショット".to_string();
            }
        }
    }
}

