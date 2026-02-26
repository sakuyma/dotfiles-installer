use super::interactive::InteractivePrompt;
use super::Args;

pub struct PromptManager {
    pub interactive: bool,
    pub assume_yes: bool,
    pub quiet: bool,
    prompt: InteractivePrompt,
    
}

impl PromptManager {
    pub fn new(args: &Args) -> Self {
        Self {
            interactive: args.interactive,
            assume_yes: args.assume_yes,
            quiet: args.quiet,
            prompt: InteractivePrompt::new(),
        }
    }

    pub fn confirm(&self, message: &str, default: bool) -> bool {
        // In quiet mode, always return default without asking
        if self.quiet {
            return default;
        }

        if self.assume_yes {
            return true;
        }
        if !self.interactive {
            return default;
        }
        self.prompt.confirm(message, default)
    }

    pub fn confirm_step(&self, step: &str, description: &str) -> bool {
        if self.assume_yes {
            return true;
        }
        if !self.interactive {
            return true;
        }
        let message = format!("{}: {}\nProceed with this step?", step, description);
        self.prompt.confirm(&message, true)
    }

    pub fn select(&self, prompt: &str, options: Vec<&str>) -> Option<String> {
        if !self.interactive {
            return options.first().map(|&s| s.to_string());
        }
        self.prompt.select(prompt, options)
    }

    pub fn select_multiple(&self, prompt: &str, options: Vec<&str>) -> Vec<String> {
        if !self.interactive {
            return options.into_iter().map(|s| s.to_string()).collect();
        }
        self.prompt.select_multiple(prompt, options)
    }
}
