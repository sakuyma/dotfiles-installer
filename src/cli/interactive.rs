use inquire::{Confirm, MultiSelect, Select, Text};

pub struct InteractivePrompt;

impl InteractivePrompt {
    pub fn new() -> Self {
        Self
    }

    // Confirm action with user
    pub fn confirm(&self, prompt: &str, default: bool) -> bool {
        Confirm::new(prompt)
            .with_default(default)
            .with_help_message("Press y for yes, n for no")
            .prompt()
            .unwrap_or(default)
    }

    // Select single option from list
    pub fn select(&self, prompt: &str, options: Vec<&str>) -> Option<String> {
        Select::new(prompt, options)
            .with_starting_cursor(0)
            .prompt()
            .map(|s| s.to_string())
            .ok()
    }

    // Select multiple options from list
    pub fn select_multiple(&self, prompt: &str, options: Vec<&str>) -> Vec<String> {
        MultiSelect::new(prompt, options)
            .with_help_message("Use space to select, enter to confirm")
            .prompt()
            .map(|selected| selected.into_iter().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    }


    // Ask for text input
    #[allow(dead_code)]
    pub fn ask_text(&self, prompt: &str, default: Option<&str>) -> String {
        let mut input = Text::new(prompt);
        if let Some(default_val) = default {
            input = input.with_default(default_val);
        }
        input.prompt().unwrap_or_default()
    }

    // Ask for yes/no with custom message
    pub fn ask_yes_no(&self, prompt: &str) -> bool {
        self.confirm(prompt, false)
    }
}

// Specialized prompts for dotfiles-installer
#[allow(dead_code)]
pub fn select_package_groups(all_groups: &[String]) -> Vec<String> {
    let prompt = InteractivePrompt::new();
    let options: Vec<&str> = all_groups.iter().map(|s| s.as_str()).collect();

    prompt.select_multiple("Select package groups to install:", options)
}

pub fn confirm_installation_step(step_name: &str, details: &str) -> bool {
    let prompt = InteractivePrompt::new();
    let message = format!("{}: {}\nProceed with this step?", step_name, details);
    prompt.confirm(&message, true)
}

#[allow(dead_code)]
pub fn confirm_destructive_action(action: &str) -> bool {
    let prompt = InteractivePrompt::new();
    let message = format!(
        "WARNING: {} This may modify system files. Continue?",
        action
    );
    prompt.confirm(&message, false)
}
