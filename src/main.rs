mod app;
mod data;
mod error;
mod ui;

use app::{
    events::{Action, Event},
    App,
};
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use data::{SkillsClient, ViewKind};
use ratatui::prelude::*;
use std::{io::stdout, time::Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let result = run().await;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    result
}

async fn run() -> anyhow::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    let (tx, mut rx) = mpsc::channel::<Event>(32);
    let client = SkillsClient::new()?;

    {
        let view = app.state.current_view;
        if let Some(view_state) = app.state.views.get_mut(&view) {
            view_state.loading = true;
        }
    }
    spawn_fetch_task(tx.clone(), client.clone(), app.state.current_view);

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // Process any pending events first
        while let Ok(event) = rx.try_recv() {
            let actions = app.update(event);
            for action in actions {
                match action {
                    Action::FetchView(view) => {
                        spawn_fetch_task(tx.clone(), client.clone(), view);
                    }
                    Action::FetchDetail { source, skill_id } => {
                        spawn_fetch_readme_task(tx.clone(), client.clone(), source, skill_id);
                    }
                    Action::InstallInteractive(skill) => {
                        // Temporarily exit TUI to run interactive command
                        execute!(stdout(), LeaveAlternateScreen)?;
                        disable_raw_mode()?;

                        let output = run_install_interactive(&skill);

                        enable_raw_mode()?;
                        execute!(stdout(), EnterAlternateScreen)?;
                        terminal.clear()?;

                        // Show result in modal
                        app.state.mode = app::state::Mode::Installing;
                        app.state.install_command = skill.install_command();
                        app.state.install_output = output;
                    }
                }
            }
        }

        if event::poll(Duration::from_millis(50))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let actions = app.update(Event::Key(key));
                    for action in actions {
                        match action {
                            Action::FetchView(view) => {
                                spawn_fetch_task(tx.clone(), client.clone(), view);
                            }
                            Action::FetchDetail { source, skill_id } => {
                                spawn_fetch_readme_task(tx.clone(), client.clone(), source, skill_id);
                            }
                            Action::InstallInteractive(skill) => {
                                // Temporarily exit TUI to run interactive command
                                execute!(stdout(), LeaveAlternateScreen)?;
                                disable_raw_mode()?;

                                let output = run_install_interactive(&skill);

                                enable_raw_mode()?;
                                execute!(stdout(), EnterAlternateScreen)?;
                                terminal.clear()?;

                                // Show result in modal
                                app.state.mode = app::state::Mode::Installing;
                                app.state.install_command = skill.install_command();
                                app.state.install_output = output;
                            }
                        }
                    }
                }
            }
        }

        if app.state.should_quit {
            break;
        }
    }

    Ok(())
}

fn run_install_interactive(skill: &data::SkillSummary) -> String {
    use std::process::Command;

    let mut cmd = Command::new("npx");
    cmd.args([
        "skills",
        "add",
        &format!("https://github.com/{}", skill.source),
        "--skill",
        &skill.skill_id,
    ]);

    // Run interactively with user's terminal
    cmd.stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    let status = cmd.status();

    match status {
        Ok(exit) if exit.success() => {
            "Installation completed successfully".to_string()
        }
        Ok(exit) => {
            format!("Installation failed with exit code: {}", exit)
        }
        Err(e) => {
            format!("Failed to run install command: {}", e)
        }
    }
}

fn spawn_fetch_task(tx: mpsc::Sender<Event>, client: SkillsClient, view: ViewKind) {
    tokio::spawn(async move {
        match client.fetch_skills(view).await {
            Ok(skills) => {
                let _ = tx.send(Event::ViewLoaded { view, skills }).await;
            }
            Err(e) => {
                let _ = tx.send(Event::Error(e.to_string())).await;
            }
        }
    });
}

fn spawn_fetch_readme_task(
    tx: mpsc::Sender<Event>,
    client: SkillsClient,
    source: String,
    skill_id: String,
) {
    tokio::spawn(async move {
        match client.fetch_readme(&source, &skill_id).await {
            Ok(markdown) => {
                let _ = tx
                    .send(Event::DetailLoaded {
                        key: skill_id,
                        markdown,
                    })
                    .await;
            }
            Err(e) => {
                let _ = tx.send(Event::Error(e.to_string())).await;
            }
        }
    });
}
