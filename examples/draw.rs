use std::{error::Error, io};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    }};
use glam::{dvec2, DMat3, DVec2};
use tactile::{get_tiling_type, IsohedralTiling};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Context, Line},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};

struct App {
    tile_type_num: usize,
    tiling: IsohedralTiling,
    edges_shapes: Vec<Vec<DVec2>>,
    bound: f64,
}

impl App {
    pub fn new() -> Self {
        let tile_type_num = 0;
        let tiling = IsohedralTiling::new(get_tiling_type(tile_type_num));
        // initialise edge shapes with simple lines

        let mut app = App {
            tile_type_num,
            tiling,
            edges_shapes: vec![],
            bound: 3.0,
        };
        app.set_default_edges();

        app
    }

    pub fn prev_tile_type(&mut self) {
        if self.tile_type_num > 0 {
            self.tile_type_num -= 1;
        }
        self.tiling.reset(get_tiling_type(self.tile_type_num));
        self.set_default_edges();
    }

    pub fn next_tile_type(&mut self) {
        if self.tile_type_num < 80 {
            self.tile_type_num += 1;
        }
        self.tiling.reset(get_tiling_type(self.tile_type_num));
        self.set_default_edges();
    }

    pub fn set_default_edges(&mut self) {
        self.edges_shapes.clear();
        for _ in 0..self.tiling.num_edge_shapes() {
            let mut edge = vec![];
            edge.push(dvec2(0.0, 0.0));
            edge.push(dvec2(1.0, 0.0));
            self.edges_shapes.push(edge);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Clear(ClearType::All)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Up => {
                    app.bound += 0.5;
                }
                KeyCode::Down => {
                    if app.bound > 1.0 {
                        app.bound -= 0.5;
                    }
                }
                KeyCode::Right => {
                    app.next_tile_type();
                }
                KeyCode::Left => {
                    app.prev_tile_type();
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let colors = [Color::Blue, Color::Red, Color::Green];
    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .direction(tui::layout::Direction::Vertical)
        .split(f.size());

    // Tiling canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Tiling type: {} ",
            get_tiling_type(app.tile_type_num)
        )))
        .paint(|ctx| {
            for tile in &app
                .tiling
                .fill_region(-app.bound, -app.bound, app.bound, app.bound)
            {
                let c = app.tiling.colour(tile.t1, tile.t2, tile.aspect);
                draw_tile(ctx, app, &tile.transform, colors[c as usize]);
            }
        })
        .x_bounds([-app.bound, app.bound])
        .y_bounds([-app.bound, app.bound]);
    f.render_widget(canvas, chunks[0]);

    // Status bar
    let text = vec![Spans::from(vec![
        Span::styled("←/→", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(
            ": change type, ",
            Style::default().add_modifier(Modifier::ITALIC),
        ),
        Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(
            ": zoom in/out, ",
            Style::default().add_modifier(Modifier::ITALIC),
        ),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(": quit, ", Style::default().add_modifier(Modifier::ITALIC)),
    ])];
    let status_bar = Paragraph::new(text)
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(status_bar, chunks[1]);
}

fn draw_tile(ctx: &mut Context, app: &App, t: &DMat3, c: Color) {
    for e in app.tiling.shapes() {
        let edge = &app.edges_shapes[e.id()];
        let transform = *t * e.transform();
        let p1 = transform.transform_point2(edge[0]);
        let p2 = transform.transform_point2(edge[1]);
        ctx.draw(&Line {
            x1: p1.x,
            y1: p1.y,
            x2: p2.x,
            y2: p2.y,
            color: c,
        });
    }
}
