use colored::*;

/// Logging message types
pub enum Type {
    _Warning,
    _Skipped,
    Error,
    Info,
    Success,
}

/// Outputs logging messages
pub fn push_message(log_type: Type, message: &str) {
    let prefix = match log_type {
        Type::_Warning => format!("{}{}{}", "[".bold(), "WARN".bold().yellow(), "]".bold()),
        Type::_Skipped => format!("{}{}{}", "[".bold(), "SKIPPED".bold().yellow(), "]".bold()),
        Type::Error => format!("\n{}{}{}", "[".bold(), "ERROR".bold().red(), "]".bold()),
        Type::Info => format!("{}{}{}", "[".bold(), "INFO".bold().cyan(), "]".bold()),
        Type::Success => format!("\n{}{}{}", "[".bold(), "SUCCESS".bold().green(), "]".bold())
    };

    eprint!("{}\n", format!("{} {}", prefix, message))
}

pub mod logger {
    use super::push_message;
    use super::Type;

    pub fn info(message: &str) {
        return push_message(Type::Info, message)
    }

    pub fn skip(message: &str) {
        return push_message(Type::_Skipped, message)
    }

    pub fn warn(message: &str) {
        return push_message(Type::_Warning, message)
    }

    pub fn error(message: &str) {
        return push_message(Type::Error, message)
    }

    pub fn sussess(message: &str) {
        return push_message(Type::Success, message)
    }
}