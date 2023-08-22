use sysinfo::{
    System, 
    SystemExt
};
use std::{
    error::Error,
    io,
    time::Duration, thread,};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Constraint, Direction, Layout,},
    style,
    widgets::{Block, Borders, Paragraph, BarChart},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
        .title("System Information")
        .borders(Borders::ALL);

        let para = Paragraph::new(get_sys_info_to_formatted_string())
            .style(style::Style::default().fg(style::Color::Red))
            .block(block);

        f.render_widget(para, size);
    })?;

    thread::spawn(|| loop {
        let _ = event::read();
    });

    // loop and update terminal every 5 seconds, 
    let mut pol = true;
    let mut sysinfo = System::new_all();
    let mut bar_chart_data = vec![("B0".to_string(), 0)];
    let value = |x: String, y: u64| -> String { format!("{}{}", x, y) };
    
    loop {
        // if we are polling update the terminal
        if pol {
            sysinfo.refresh_all();
            terminal.draw(|f| {
                // How to split area into 2 parts
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                    .split(f.size());

                let block = Block::default()
                .title("System Information")
                .borders(Borders::ALL);

                let block2 = Block::default()
                    .title("Memory Usage Chart")
                    .style(style::Style::default().bg(style::Color::White))
                    .borders(Borders::ALL);

                let data = convert_kb_to_gb(sysinfo.used_memory());
                let v = value("B".to_string(), bar_chart_data.len() as u64).to_string();
                // add new data to the chart and increment label for &str
                bar_chart_data.push((v, data as u64));
                // Some hokey pokey to get the data into the right format
                let borrowed_data: Vec<(&str, u64)> = bar_chart_data
                    .iter()
                    .rev()
                    .take(10)
                    .rev()
                    .map(|(s, num)| (s.as_str(), *num))
                    .collect();

                let barchart = BarChart::default()
                    .block(block2)
                    .data(&borrowed_data)
                    .max(convert_kb_to_gb(sysinfo.available_memory()) as u64)
                    .bar_width(9)
                    .bar_style(style::Style::default().fg(style::Color::Yellow))
                    .value_style(style::Style::default().fg(style::Color::Black).bg(style::Color::Yellow));

                let para = Paragraph::new(get_sys_info_to_formatted_string())
                    .style(style::Style::default().bg(style::Color::White))
                    .block(block);

                f.render_widget(para, chunks[0]);
                f.render_widget(barchart, chunks[1]);
            })?;
        }

        // if s is pressed stop polling, 
        // if q is pressed quit, 
        // if r is pressed start polling again
        // ignore mouse movements
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    event::KeyCode::Char('s') => {
                        pol = false;
                    }
                    event::KeyCode::Char('q') => {
                        // Exit the program and clear the terminal
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        return Ok(());
                    }
                    event::KeyCode::Char('r') => {
                        pol = true;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn get_sys_info_to_formatted_string() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut s = String::new();
    s.push_str(&format!("total memory: {} GB\n", convert_kb_to_gb(sys.total_memory())));
    s.push_str(&format!("used memory : {} GB\n", convert_kb_to_gb(sys.used_memory())));
    s.push_str(&format!("total swap  : {} GB\n", convert_kb_to_gb(sys.total_swap())));
    s.push_str(&format!("used swap   : {} GB\n", convert_kb_to_gb(sys.used_swap())));
    s.push_str(&format!("process count: {}\n", sys.processes().len()));
    s.push_str(&format!("boot time: {:?}\n", sys.boot_time()));
    s.push_str(&format!("networks: \n"));
    for (interface_name, data) in sys.networks() {
        s.push_str(&format!("{}: {:?}\n", interface_name, data));
    }
    s
}

fn convert_kb_to_gb(kb: u64) -> f64 {
    kb as f64 / 1024.0 / 1024.0 / 1024.0
}