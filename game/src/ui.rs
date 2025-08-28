//! UI system - HUD, menus, level-up screen, and shop interface
use fyrox::{
    core::{
        algebra::Vector2,
        reflect::prelude::*,
        visitor::prelude::*,
    },
    gui::{
        button::ButtonBuilder,
        grid::{GridBuilder, Column, Row},
        text::TextBuilder,
        widget::WidgetBuilder,
        UiNode, BuildContext,
    },
};

use crate::{
    player::Player,
    wave::WaveManager,
    upgrade::{Upgrade, UpgradeManager},
};

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct UIManager {
    // HUD elements
    pub health_bar: Option<UiNode>,
    pub shield_bar: Option<UiNode>,
    pub exp_bar: Option<UiNode>,
    pub wave_info: Option<UiNode>,
    pub currency_display: Option<UiNode>,
    
    // Overlays
    pub levelup_panel: Option<UiNode>,
    pub shop_panel: Option<UiNode>,
    pub pause_panel: Option<UiNode>,
    
    // State
    pub is_levelup_open: bool,
    pub is_shop_open: bool,
    pub is_paused: bool,
    pub current_currency: u32,
}

impl UIManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn build_hud(&mut self, ctx: &mut BuildContext) {
        // Health Bar
        self.health_bar = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("HealthBar")
                    .with_width(200.0)
                    .with_height(20.0)
            )
            .add_row(Row::stretch())
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                ).with_text("Health: 100/100")
                .build(ctx)
            )
            .build(ctx)
        );
        
        // Shield Bar
        self.shield_bar = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("ShieldBar")
                    .with_width(200.0)
                    .with_height(20.0)
            )
            .add_row(Row::stretch())
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                ).with_text("Shields: 50/50")
                .build(ctx)
            )
            .build(ctx)
        );
        
        // Experience Bar
        self.exp_bar = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("ExpBar")
                    .with_width(300.0)
                    .with_height(20.0)
            )
            .add_row(Row::stretch())
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                ).with_text("Level 1 - EXP: 0/100")
                .build(ctx)
            )
            .build(ctx)
        );
        
        // Wave Info
        self.wave_info = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("WaveInfo")
                    .with_width(200.0)
                    .with_height(20.0)
            )
            .add_row(Row::stretch())
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                ).with_text("Wave 1")
                .build(ctx)
            )
            .build(ctx)
        );
        
        // Currency Display
        self.currency_display = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("Currency")
                    .with_width(150.0)
                    .with_height(20.0)
            )
            .add_row(Row::stretch())
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                ).with_text("Credits: 0")
                .build(ctx)
            )
            .build(ctx)
        );
    }
    
    pub fn build_levelup_screen(&mut self, ctx: &mut BuildContext, upgrades: &[Upgrade]) {
        // Create level-up panel with upgrade choices
        self.levelup_panel = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("LevelUpPanel")
                    .with_width(600.0)
                    .with_height(400.0)
            )
            .add_row(Row::strict(40.0)) // Title
            .add_row(Row::stretch())    // Upgrade choices
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                        .on_row(0)
                        .on_column(0)
                ).with_text("LEVEL UP! Choose an upgrade:")
                .build(ctx)
            )
            .with_child(
                self.build_upgrade_grid(ctx, upgrades)
            )
            .build(ctx)
        );
        
        self.is_levelup_open = true;
    }
    
    pub fn build_shop_screen(&mut self, ctx: &mut BuildContext, shop_items: &[Upgrade]) {
        // Create shop panel with purchasable items
        self.shop_panel = Some(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_name("ShopPanel")
                    .with_width(800.0)
                    .with_height(600.0)
            )
            .add_row(Row::strict(40.0)) // Title
            .add_row(Row::stretch())    // Shop items
            .add_row(Row::strict(60.0)) // Continue button
            .add_column(Column::stretch())
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new()
                        .on_row(0)
                        .on_column(0)
                ).with_text("SHOP - Upgrade your ship between waves")
                .build(ctx)
            )
            .with_child(
                self.build_shop_grid(ctx, shop_items)
            )
            .with_child(
                ButtonBuilder::new(
                    WidgetBuilder::new()
                        .on_row(2)
                        .on_column(0)
                        .with_width(200.0)
                        .with_height(40.0)
                ).with_text("Continue to Next Wave")
                .build(ctx)
            )
            .build(ctx)
        );
        
        self.is_shop_open = true;
    }
    
    pub fn update_hud(&mut self, ctx: &mut BuildContext, player: &Player, wave_manager: &WaveManager) {
        // Update health bar
        if let Some(_health_bar) = self.health_bar {
            // Update health bar visual
            // In a real implementation, you'd update the bar's fill percentage
        }
        
        // Update shield bar
        if let Some(_shield_bar) = self.shield_bar {
            // Update shield bar visual
        }
        
        // Update experience bar
        if let Some(_exp_bar) = self.exp_bar {
            // Update exp bar visual and text
        }
        
        // Update wave info
        if let Some(_wave_info) = self.wave_info {
            // Update wave number and timer
        }
        
        // Update currency
        if let Some(_currency_display) = self.currency_display {
            // Update currency amount
        }
    }
    
    pub fn close_levelup_screen(&mut self) {
        self.is_levelup_open = false;
        if let Some(_panel) = self.levelup_panel.take() {
            // Remove panel from UI
        }
    }
    
    pub fn close_shop_screen(&mut self) {
        self.is_shop_open = false;
        if let Some(_panel) = self.shop_panel.take() {
            // Remove panel from UI
        }
    }
    
    pub fn handle_levelup_choice(&mut self, choice_index: usize, upgrade_manager: &mut UpgradeManager, player: &mut Player) {
        // Handle the player's upgrade choice
        let choices = upgrade_manager.get_levelup_choices(3);
        if let Some(chosen_upgrade) = choices.get(choice_index) {
            upgrade_manager.apply_upgrade(chosen_upgrade.clone(), player);
            self.close_levelup_screen();
        }
    }
    
    pub fn handle_shop_purchase(&mut self, item_index: usize, upgrade_manager: &mut UpgradeManager, player: &mut Player) -> bool {
        // Handle shop item purchase
        let shop_items = upgrade_manager.get_shop_items();
        if let Some(item) = shop_items.get(item_index) {
            let cost = item.get_current_cost();
            if self.current_currency >= cost {
                self.current_currency -= cost;
                upgrade_manager.apply_upgrade(item.clone(), player);
                return true;
            }
        }
        false
    }
    
    pub fn add_currency(&mut self, amount: u32) {
        self.current_currency += amount;
    }
    
    fn build_upgrade_grid(&self, ctx: &mut BuildContext, upgrades: &[Upgrade]) -> UiNode {
        let mut grid = GridBuilder::new(
            WidgetBuilder::new()
                .on_row(1)
                .on_column(0)
        );
        
        // Add rows and columns based on upgrade count
        for i in 0..upgrades.len() {
            grid = grid.add_row(Row::strict(100.0));
        }
        grid = grid.add_column(Column::stretch());
        
        // Add upgrade buttons
        for (i, upgrade) in upgrades.iter().enumerate() {
            let button = ButtonBuilder::new(
                WidgetBuilder::new()
                    .on_row(i)
                    .on_column(0)
                    .with_width(500.0)
                    .with_height(80.0)
            )
            .with_text(&format!("{}\n{}", upgrade.name, upgrade.description))
            .build(ctx);
            
            grid = grid.with_child(button);
        }
        
        grid.build(ctx)
    }
    
    fn build_shop_grid(&self, ctx: &mut BuildContext, items: &[Upgrade]) -> UiNode {
        let mut grid = GridBuilder::new(
            WidgetBuilder::new()
                .on_row(1)
                .on_column(0)
        );
        
        // Add rows and columns based on item count
        for i in 0..items.len() {
            grid = grid.add_row(Row::strict(120.0));
        }
        grid = grid.add_column(Column::stretch());
        
        // Add shop item buttons
        for (i, item) in items.iter().enumerate() {
            let cost = item.get_current_cost();
            let can_afford = self.current_currency >= cost;
            
            let button_text = format!(
                "{}\n{}\nCost: {} Credits{}",
                item.name,
                item.description,
                cost,
                if can_afford { "" } else { " (Cannot Afford)" }
            );
            
            let button = ButtonBuilder::new(
                WidgetBuilder::new()
                    .on_row(i)
                    .on_column(0)
                    .with_width(700.0)
                    .with_height(100.0)
                    .with_enabled(can_afford)
            )
            .with_text(&button_text)
            .build(ctx);
            
            grid = grid.with_child(button);
        }
        
        grid.build(ctx)
    }
}

#[derive(Clone, Debug)]
pub enum UIEvent {
    LevelUpChoice(usize),
    ShopPurchase(usize),
    ContinueToNextWave,
    PauseWave,
    ResumeWave,
}
