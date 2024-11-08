use crate::entities::npc::NPC;
use crate::entities::player::Player;
use crate::maps::maps::Maps;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self};
use std::collections::VecDeque;

pub fn run_ui(player: &mut Player, map: &mut Maps, npcs: &mut Vec<NPC>) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;

    let initial_size = terminal.size()?;
    let max_log_lines = (initial_size.height * 20 / 100) as usize;
    let mut ui_state = UIState::new(max_log_lines);

    loop {
        let size = terminal.size()?;
        let map_width = (size.width * 70 / 100) as usize;
        let map_height = (size.height * 80 / 100) as usize;
        map.resize(map_width, map_height);
        map.update_player_position(player.position.0, player.position.1);

        ui_state.max_log_lines = (size.height * 20 / 100) as usize;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let left_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(chunks[0]);

            let map_area = left_chunks[0];
            map.resize_to_frame(map_area.width as usize, map_area.height as usize);

            let map_display = map.draw(player.position, npcs);
            let map_widget = Paragraph::new(map_display)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Map"));
            f.render_widget(map_widget, map_area);

            let stats = format!(
                "Name: {}\nHealth: {}\nStrength: {}\nAgility: {}\nExperience: {}",
                player.name, player.health, player.strength, player.agility, player.experience
            );
            let stats_block = Paragraph::new(stats)
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("Player Stats"));
            f.render_widget(stats_block, left_chunks[1]);

            let command_output = format!("> {}", ui_state.get_log());
            let command_input_display = Paragraph::new(command_output)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Commands"));
            f.render_widget(command_input_display, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('w') => {
                    player.move_up(map);
                    ui_state.add_log("Player moved up".to_string());
                }
                KeyCode::Char('s') => {
                    player.move_down(map);
                    ui_state.add_log("Player moved down".to_string());
                }
                KeyCode::Char('a') => {
                    player.move_left(map);
                    ui_state.add_log("Player moved left".to_string());
                }
                KeyCode::Char('d') => {
                    player.move_right(map);
                    ui_state.add_log("Player moved right".to_string());
                }

                KeyCode::Char('q') => {
                    break;
                }

                KeyCode::Enter => {
                    let command = ui_state.command_input.trim().to_lowercase();
                    ui_state.command_input.clear();

                    let log_message = match command.as_str() {
                        "engage" => {
                            if let Some(npc_index) = map.find_nearby_npc(player.position, npcs) {
                                let npc = &mut npcs[npc_index];
                                let combat_log = player.engage_in_combat(npc);

                                if npc.health <= 0 {
                                    npcs.remove(npc_index);
                                }
                                combat_log
                            } else {
                                "No NPC nearby to engage in combat!".to_string()
                            }
                        }
                        "fight" => "Initiating fight...".to_string(),
                        _ => "Invalid command! Type 'engage' or 'fight' for combat.".to_string(),
                    };

                    ui_state.add_log(log_message);
                }

                KeyCode::Char(c) => {
                    ui_state.command_input.push(c);
                }
                KeyCode::Backspace => {
                    ui_state.command_input.pop();
                }
                _ => (),
            }

            map.update_player_position(player.position.0, player.position.1);
        }
    }

    Ok(())
}



pub struct UIState {
    pub command_input: String,
    pub log_buffer: VecDeque<String>,
    pub max_log_lines: usize
}

impl UIState {
    pub fn new(max_log_lines: usize) -> Self {
        Self {
            command_input: String::new(),
            log_buffer: VecDeque::with_capacity(max_log_lines),
            max_log_lines
        }
    }

    pub fn add_log(&mut self, message: String) {
        if self.log_buffer.len() == self.max_log_lines {
            self.log_buffer.pop_front();
        }
        self.log_buffer.push_back(message)
    }

    pub fn get_log(&self) -> String {
        self.log_buffer.iter().cloned().collect::<Vec<_>>().join(" | ")
    }
}

