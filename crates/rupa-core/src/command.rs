use rupa_base::Error;

/// The behavioral contract for a Rupa Console Command.
/// This allows users to extend the CLI with project-specific logic.
pub trait ConsoleCommand: Send + Sync {
    /// The unique name of the command (e.g., "make:component").
    fn name(&self) -> &str;

    /// A brief description of what the command does.
    fn description(&self) -> &str;

    /// The execution logic for the command.
    fn execute(&self, args: Vec<String>) -> Result<(), Error>;
}
