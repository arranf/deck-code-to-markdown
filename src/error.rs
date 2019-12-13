use custom_error::custom_error;

custom_error! {pub AppError // Enum name
    // Specific types
    Io{source: std::io::Error}            = "Error performing IO",
    Serde{source: serde_json::error::Error } = "Error saving or fetching data",
    Reqwest{source: reqwest::Error} = "Error contacting API",
    DecodeError{source: deck_codes::error::DeckCodeError} = "Error reading deck code",
    CacheError{} = "Error accessing cache data",
}
