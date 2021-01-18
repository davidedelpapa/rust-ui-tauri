#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

// Define Custom Error
#[derive(Debug, Clone)]
struct CommandError<'a> {
    message: &'a str,
}
impl<'a> CommandError<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}
impl<'a> std::fmt::Display for CommandError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl<'a> std::error::Error for CommandError<'a> {}

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        // definitions for your custom commands from Cmd here
                        GetResponse {
                            payload,
                            callback,
                            error,
                        } => tauri::execute_promise(
                            _webview,
                            move || {
                                if payload.state != 0 {
                                    let response = cmd::Response {
                                        message: "Hello, World!",
                                    };
                                    Ok(response)
                                } else {
                                    Err(CommandError::new("State not OK").into())
                                }
                            },
                            callback,
                            error,
                        ),
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}
