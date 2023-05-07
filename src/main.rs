use std::{env, io, error::Error, time::Duration};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, poll, self, KeyCode}};
//use crossterm::
use tokio;
use tui::{backend::CrosstermBackend, Terminal, layout::{Layout, Direction, Constraint}, widgets::{Block, Borders, BorderType, ListItem, List, ListState}, style::{Style, Modifier}};
use players::{Players, Position};

mod players;

const SPORTS_DATA_IO_NHL_API_KEY: &str = "SPORTS_DATA_IO_NHL_KEY";

#[tokio::main]
async fn main() {
    run_tui().await.unwrap();
}

async fn run_tui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new(get_players_by_team("DAL".to_owned()).await.unwrap());

    draw_and_control_ui(&mut terminal, &mut state).unwrap();

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw_and_control_ui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &mut AppState) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|frame| draw_ui(frame, state))?;

        if poll(Duration::from_millis(100)).unwrap() {
            if let crossterm::event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Up => {
                        state.move_up();
                    }
                    KeyCode::Down => {
                        state.move_down();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw_ui(frame: &mut tui::Frame<CrosstermBackend<io::Stdout>>, state: &mut AppState) {
    let parent_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60)
            ].as_ref()
        )
        .split(frame.size());

    let player_list_block = Block::default()
        .title("Players")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);
    frame.render_widget(player_list_block, parent_chunk[0]);
    player_list_render(frame, state, parent_chunk[0]);
        
}

fn player_list_render(frame: &mut tui::Frame<CrosstermBackend<io::Stdout>>, state: &mut AppState, area: tui::layout::Rect) {
    let players = state.players.clone();

    let list_items: Vec<ListItem> = players.into_iter()
        .map(|player| {
            ListItem::new(format!("{}, {}, - {}", player.last_name, player.first_name, player.position))
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default())
        .highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
    frame.render_stateful_widget(list, area, &mut state.list_state)
}

async fn get_players_by_team(team_key: String) -> Result<Players, reqwest::Error> {
    let api_endpoint = format!("https://api.sportsdata.io/v3/nhl/scores/json/Players/{}?key={}", team_key, env::var(SPORTS_DATA_IO_NHL_API_KEY).unwrap());

    let response = reqwest::get(api_endpoint).await?;
        
    let players: Players = response.json().await?;

    Ok(players)
}

struct AppState {
    players: Players,
    active_player_id: Option<usize>,
    list_state: ListState

}
impl AppState {
    fn new(players: Players) -> Self {
        Self {
            players,
            active_player_id: None,
            list_state: ListState::default(),
        }
    }

    fn move_up(&mut self) {
        let selected = match self.list_state.selected() {
            Some(v) => {
                if v == 0 {
                    Some(v)
                } else {
                    Some(v - 1)
                }
            }
            None => {
                Some(0)
            }
        };
        self.list_state.select(selected);
    }

    fn move_down(&mut self) {
        let selected = match self.list_state.selected() {
            Some(v) => {
                if v == self.players.len() - 1 {
                    Some(v)
                } else {
                    Some(v + 1)
                }
            }
            None => {
                Some(0)
            }
        };
        self.list_state.select(selected);
    }
}
