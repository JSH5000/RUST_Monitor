use sysinfo::{
    System, 
    SystemExt
};
use std::{io, thread, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    style,
    widgets::{Block, Borders, Paragraph},
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
    loop {
        // update terminal if we are polling

        // if we are polling update the terminal
        if pol {
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
        }

        // if s is pressed stop polling, if q is pressed quit, if r is pressed start polling again
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    event::KeyCode::Char('s') => {
                        pol = false;
                    }
                    event::KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        std::process::exit(0);
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