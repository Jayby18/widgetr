use ratatui::{
    widgets::*,
    style::{Style, Color},
};
use sysinfo::{
    System,
    SystemExt,
    CpuExt,
};

pub fn system_info(sys: &System) -> Paragraph {
    paragraph("System name", format!("{} {}", sys.name().unwrap(), sys.host_name().unwrap()))
}

pub fn cpu_usage(sys: &System) -> Gauge {
    gauge("CPU usage", (sys.global_cpu_info().cpu_usage()) as u16)
}

pub fn ram_usage(sys: &System) -> Gauge {
    gauge("RAM usage", (sys.used_memory() / sys.total_memory()) as u16)
}

fn paragraph(title: &str, value: String) -> Paragraph {
    Paragraph::new(value)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
        )
}

fn gauge(title: &str, value: u16) -> Gauge {
    Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
        )
        .gauge_style(
            Style::default()
                .fg(Color::White)
        )
        .percent(value)
}