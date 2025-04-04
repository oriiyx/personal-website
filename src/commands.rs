use js_sys::{Array, Date, Promise};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window, Response};

// A struct to maintain state of all available commands
#[wasm_bindgen]
pub struct CommandRegistry {
    commands: HashMap<String, String>,
}

#[wasm_bindgen]
impl CommandRegistry {
    // Constructor
    #[wasm_bindgen(constructor)]
    pub fn new() -> CommandRegistry {
        let mut commands = HashMap::new();

        // Register some default commands with descriptions
        commands.insert("help".to_string(), "Display available commands".to_string());
        commands.insert("aboutme".to_string(), "Display about me info".to_string());
        commands.insert("booktime".to_string(), "Display booktime info".to_string());
        commands.insert("bde".to_string(), "Display bde info".to_string());
        commands.insert("linkedin".to_string(), "Display linkedin info".to_string());
        commands.insert("hostname".to_string(), "Show current hostname".to_string());
        commands.insert("whoami".to_string(), "Show current user".to_string());
        commands.insert("date".to_string(), "Display current date".to_string());
        commands.insert("echo".to_string(), "Repeat the input text".to_string());
        commands.insert("github".to_string(), "Open GitHub profile".to_string());
        commands.insert("sudo".to_string(), "Admin access".to_string());
        commands.insert("theme".to_string(), "Choose your theme".to_string());
        commands.insert("repo".to_string(), "Open this repository".to_string());
        commands.insert("clear".to_string(), "Clears screen".to_string());
        commands.insert("email".to_string(), "Opens a tab with mailto".to_string());
        commands.insert("weather".to_string(), "Shows weather info".to_string());
        commands.insert("banner".to_string(), "Terminal banner".to_string());
        commands.insert("exit".to_string(), "Displays how to exit".to_string());

        CommandRegistry { commands }
    }

    #[wasm_bindgen]
    pub fn help(&self) -> String {
        let mut result = String::from("Available commands:\n\n");

        // Define categories with their commands
        let categories = [
            (
                "General",
                vec![
                    "help", "banner", "clear", "exit", "whoami", "hostname", "date",
                ],
            ),
            (
                "Navigation",
                vec!["github", "repo", "blog", "email", "linkedin"],
            ),
            ("Information", vec!["aboutme", "weather", "echo"]),
            ("Projects", vec!["booktime", "bde"]),
            ("Customization", vec!["theme", "sudo"]),
        ];

        for (category, cmds) in categories.iter() {
            // Add category with bold formatting
            result.push_str(&format!(
                "<span style='font-weight: bold; color: #84c138;'>{}</span>\n",
                category
            ));

            for cmd in cmds {
                if let Some(desc) = self.commands.get(*cmd) {
                    // Command name in bold, description in normal text
                    result.push_str(&format!(
                        "<span style='font-weight: bold;'>{}</span>{}{}\n",
                        cmd,
                        " ".repeat(12.max(cmd.len() + 2) - cmd.len()),
                        desc
                    ));
                }
            }

            result.push_str("\n");
        }

        result
    }

    #[wasm_bindgen]
    pub fn whoami(&self) -> String {
        String::from("guest")
    }
    #[wasm_bindgen]
    pub fn exit(&self) -> String {
        String::from("Close the tab to exit.")
    }

    #[wasm_bindgen]
    pub fn date(&self) -> String {
        let now = Date::new_0();
        let date_string = now.to_string();
        date_string.into()
    }

    #[wasm_bindgen]
    pub fn booktime(&self) -> String {
        if let Some(window) = window() {
            let _ = window.open_with_url("https://booktime.co");
        }

        String::from(
            r#"Booktime is a mobile application for book reading tracking with book club functionality. Check out <a href="https://booktime.co" target="_blank">https://booktime.co</a> for more info!"#,
        )
    }

    #[wasm_bindgen]
    pub fn blog(&self) -> String {
        if let Some(window) = window() {
            let _ = window.open_with_url("https://oriiyx.dev");
        }

        String::from(
            r#"Checkout my blog over at <a href="https://oriiyx.dev" target="_blank">https://oriiyx.dev</a> for more personal thoughts!"#,
        )
    }

    #[wasm_bindgen]
    pub fn bde(&self) -> String {
        if let Some(window) = window() {
            let _ = window.open_with_url("https://github.com/oriiyx/bde");
        }

        String::from("BDE stands for Boring Database Engine - it takes SQLc ideology (which I love) and tries to implement it in PHP world.")
    }

    #[wasm_bindgen]
    pub fn linkedin(&self) -> String {
        if let Some(window) = window() {
            let _ = window.open_with_url("https://www.linkedin.com/in/peter-paravinja/");
        }

        String::from(
            r#"Opening <a href="https://www.linkedin.com/in/peter-paravinja" target="_blank">https://www.linkedin.com/in/peter-paravinja</a>"#,
        )
    }

    #[wasm_bindgen]
    pub fn github(&self) -> String {
        if let Some(window) = window() {
            let _ = window.open_with_url("https://github.com/oriiyx");
        }

        String::from(
            r#"Opening <a href="https://github.com/oriiyx" target="_blank">https://github.com/oriiyx</a>"#,
        )
    }

    #[wasm_bindgen]
    pub fn repo(&self, repo_url: &str) -> String {
        if let Some(window) = window() {
            if let Ok(_) = window.open_with_url(repo_url) {
                return String::from("Opening repository...");
            }
        }
        String::from("Failed to open repository.")
    }

    #[wasm_bindgen]
    pub fn email(&self, email: &str) -> String {
        let mailto_url = format!("mailto:{}", email);
        console::log_1(&JsValue::from_str(&mailto_url));

        if let Some(window) = window() {
            if let Ok(_) = window.open_with_url(&mailto_url) {
                return String::from("Opening mailto link...");
            }
        }

        String::from("Failed to open email client.")
    }

    #[wasm_bindgen]
    pub fn sudo(&self, args: Array) -> String {
        let mut r = String::new();

        if args.length() > 0 {
            for arg in args {
                if let Some(arg) = arg.as_string() {
                    r = format!("Permission denied running: {}", arg)
                } else {
                    r = format!("Permission denied running: {:?}", arg.as_string())
                }
            }
        } else {
            r = String::from("Permission denied.");
        }

        r
    }

    #[wasm_bindgen]
    pub fn echo(&self, args: Array) -> String {
        let mut r = String::new();

        for arg in args {
            if let Some(arg) = arg.as_string() {
                r.push_str(arg.as_str());
                r.push_str(" ");
            }
        }

        r
    }

    #[wasm_bindgen]
    pub fn weather(&self, args: Array) -> Promise {
        if args.length() == 0 {
            return Promise::resolve(&JsValue::from_str(
                "Usage: weather [city]. Example: weather Ljubljana",
            ));
        }

        let city_js_string = args.join("+");
        let city = city_js_string.as_string().unwrap();

        let url = format!("https://wttr.in/{}?ATm", city);

        let window = window().unwrap();

        let promise = window.fetch_with_str(&url);

        wasm_bindgen_futures::future_to_promise(async move {
            let resp_value = JsFuture::from(promise).await?;
            let response: Response = resp_value.dyn_into().unwrap();
            let text_promise = response.text();
            let text_js_value = JsFuture::from(text_promise.unwrap()).await?;
            Ok(text_js_value)
        })
    }

    #[wasm_bindgen]
    pub fn banner(&self, version: &str) -> String {
        format!(
            r#"
██████╗ ███████╗████████╗███████╗██████╗     ██████╗
██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗    ██╔══██╗
██████╔╝█████╗     ██║   █████╗  ██████╔╝    ██████╔╝
██╔═══╝ ██╔══╝     ██║   ██╔══╝  ██╔══██╗    ██╔═══╝
██║     ███████╗   ██║   ███████╗██║  ██║    ██║██╗
╚═╝     ╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝    ╚═╝╚═╝  v{}

Peter Paravinja | Full-Stack Developer | Golang | TypeScript | PHP | Rust | Python
Working on web applications, mobile applications and open-source tools.
Type 'help' to see list of available commands.
"#,
            version
        )
    }

    #[wasm_bindgen]
    pub fn aboutme(&self) -> String {
        r#"
<span style='font-weight: bold; color: #84c138;'>Hey there! I'm Peter Paravinja</span>, a full-stack developer with 6 years of experience in building
complex web applications and a bit of mobile development.

<span style='font-weight: bold;'>CURRENT WORK</span>:
- Working at Netis on cryptographic applications, passkey technology while tackling infrastructure problems
- Developing SQLC port for PHP and MySQL (rewritten in Rust)
- Creating interactive CLI-based tools and applications

<span style='font-weight: bold;'>SKILLS</span>:
- Languages: Golang, JavaScript/TypeScript, Rust, PHP, Python
- Full development cycle: from design to deployment and marketing
- Solo developed Booktime.co, a book reading tracking app with book club functionality

<span style='font-weight: bold;'>PROJECTS</span>:
- Booktime.co: A social book-tracking platform with book club features
  (React Native, Remix, Golang, infrastructure, design, marketing)
- Use 'booktime' command for more info!
- Use 'bde' command for more info!

<span style='font-weight: bold;'>CONNECT</span>:
- Website: <a href="https://oriiyx.dev/" target="_blank">https://oriiyx.dev/</a>
- LinkedIn: <a href="https://www.linkedin.com/in/peter-paravinja/" target="_blank">https://www.linkedin.com/in/peter-paravinja/</a>
- GitHub: <a href="https://github.com/oriiyx" target="_blank">https://github.com/oriiyx</a>

I'm always looking for new challenges and opportunities to improve my skills.
Feel free to reach out via LinkedIn or GitHub if you'd like to connect!
"#
            .to_string()
    }

    #[wasm_bindgen]
    pub fn hostname(&self) -> String {
        if let Some(window) = window() {
            let loc = window.location();
            return loc.hostname().unwrap().to_string();
        }

        String::from("Failed to get location.")
    }
}
