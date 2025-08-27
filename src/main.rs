use claude_statusline::{
    input::ClaudeInput,
    directory::format_directory,
    jj_status::get_jj_status,
    output::StatusLine,
};
use std::env;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse JSON input from stdin
    let input = ClaudeInput::from_stdin()?;
    
    // Format directory path
    let home_dir = env::var("HOME").ok();
    let directory = format_directory(&input.workspace.current_dir, home_dir.as_deref());
    
    // Get jj status
    let jj_info = get_jj_status(&input.workspace.current_dir);
    let jj_info_option = if jj_info.change_id.is_some() { Some(jj_info) } else { None };
    
    // Determine output style
    let output_style = if input.output_style.name != "default" && input.output_style.name != "null" {
        Some(input.output_style.name)
    } else {
        None
    };
    
    // Create and format status line
    let status_line = StatusLine {
        directory,
        jj_info: jj_info_option,
        model_name: input.model.display_name,
        output_style,
    };
    
    print!("{}", status_line.format());
    
    Ok(())
}
