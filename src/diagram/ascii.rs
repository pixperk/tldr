use anyhow::Result;

#[derive(Debug)]
pub struct AsciiDiagram {
    pub title: String,
    pub content: String,
}

impl AsciiDiagram {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    /// Extract ASCII diagram from LLM response
    pub fn extract_from_response(response: &str) -> Result<String> {
        // Look for ```ascii or ```text blocks first
        for marker in &["```ascii", "```text", "```"] {
            if let Some(start) = response.find(marker) {
                let start_content = start + marker.len();
                if let Some(end) = response[start_content..].find("```") {
                    let diagram_content = &response[start_content..start_content + end];
                    // Skip language identifier if present
                    let content = if let Some(newline) = diagram_content.find('\n') {
                        &diagram_content[newline + 1..]
                    } else {
                        diagram_content
                    };
                    return Ok(content.trim().to_string());
                }
            }
        }

        // If no code blocks found, return the whole response cleaned up
        Ok(response.trim().to_string())
    }

    /// Format the diagram for inclusion in README
    pub fn format_for_readme(&self, section_title: &str) -> String {
        format!(
            r#"## 🏛️ {}

```
{}
```

*Diagram: {}*
"#,
            section_title, self.content, self.title
        )
    }

    /// Create a universal fallback diagram based on detected project type
    pub fn create_fallback_diagram(project_type: &str, components: &[String]) -> Self {
        let content = match project_type.to_lowercase().as_str() {
            "webapp" | "web" => Self::create_web_app_diagram(components),
            "api" | "backend" | "service" => Self::create_api_diagram(components),
            "cli" | "tool" | "command" => Self::create_cli_diagram(components),
            "library" | "package" | "module" => Self::create_library_diagram(components),
            "mobile" | "app" => Self::create_mobile_diagram(components),
            _ => Self::create_generic_diagram(components),
        };

        Self::new(format!("{} Architecture", project_type), content)
    }

    fn create_web_app_diagram(components: &[String]) -> String {
        format!(r#"┌─────────────────────────────────────────────────────────────────────────────┐
│                          🌐 Web Application Architecture                        │
└─────────────────────────────────────────────────────────────────────────────┘

🖥️  FRONTEND                  ⚙️  BACKEND                   💾 DATA LAYER
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  🎨 UI Layer    │◀────────▶│  🔧 API Server  │◀─────────▶│  📊 Database    │
│  📱 Components  │         │  🛡️  Auth       │          │  🗃️  Storage    │
│  🎯 State Mgmt  │         │  📋 Business    │          │  🔍 Queries     │
│  🌐 HTTP Client │         │  📊 Data Layer  │          │  🔄 Migrations  │
└─────────────────┘         └─────────────────┘          └─────────────────┘
         │                           │                            │
         ▼                           ▼                            ▼
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📦 Build Tools │         │  🐳 Deployment  │          │  ☁️  Cloud      │
│  🎨 Styling     │         │  📊 Monitoring  │          │  🔒 Security    │
└─────────────────┘         └─────────────────┘          └─────────────────┘

Components: {}"#, components.join(", "))
    }

    fn create_api_diagram(components: &[String]) -> String {
        format!(r#"┌─────────────────────────────────────────────────────────────────────────────┐
│                            🔧 API Service Architecture                          │
└─────────────────────────────────────────────────────────────────────────────┘

📡 CLIENT LAYER              🔧 SERVICE LAYER              💾 DATA LAYER
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📱 Mobile Apps │──┐      │  🌐 API Gateway │          │  📊 Primary DB  │
│  🌐 Web Clients │  │   ┌─▶│  🛡️  Auth Guard │◀─────────▶│  🗃️  Cache      │
│  🤖 API Clients │  └───┤  │  📋 Controllers │          │  📁 File Store  │
│  📊 Dashboards  │      └─▶│  ⚙️  Services   │          │  🔍 Search DB   │
└─────────────────┘         └─────────────────┘          └─────────────────┘
         │                           │                            │
         ▼                           ▼                            ▼
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📚 API Docs    │         │  📊 Monitoring  │          │  🔄 Replication │
│  🧪 Testing     │         │  📈 Analytics   │          │  💿 Backup      │
└─────────────────┘         └─────────────────┘          └─────────────────┘

Components: {}"#, components.join(", "))
    }

    fn create_cli_diagram(components: &[String]) -> String {
        format!(r#"┌─────────────────────────────────────────────────────────────────────────────┐
│                           🖥️  CLI Tool Architecture                            │
└─────────────────────────────────────────────────────────────────────────────┘

📥 INPUT LAYER               🔄 PROCESSING ENGINE           📤 OUTPUT LAYER
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  👤 User Input  │──────────▶│  📋 Command      │──────────▶│  📄 Files       │
│  📁 Config      │         │     Parser       │          │  📊 Reports     │
│  🔑 Credentials │         │  ⚙️  Core Engine │          │  📺 Console     │
│  📂 File Args   │         │  🔍 Processor    │          │  📈 Graphics    │
└─────────────────┘         └─────────────────┘          └─────────────────┘
         │                           │                            │
         ▼                           ▼                            ▼
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  ✅ Validation  │         │  🌐 External    │          │  ✨ Formatting  │
│  📋 Help System │         │     APIs        │          │  🎨 Styling     │
└─────────────────┘         └─────────────────┘          └─────────────────┘

Components: {}"#, components.join(", "))
    }

    fn create_library_diagram(components: &[String]) -> String {
        format!(r#"┌─────────────────────────────────────────────────────────────────────────────┐
│                          📦 Library/Package Architecture                        │
└─────────────────────────────────────────────────────────────────────────────┘

🌐 PUBLIC API                🔧 INTERNAL MODULES           📚 DEPENDENCIES
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📋 Main Export │         │  ⚙️  Core Logic │          │  📦 External    │
│  🎯 Functions   │◀────────▶│  🔧 Utilities   │◀─────────▶│     Packages    │
│  📊 Classes     │         │  🎨 Helpers     │          │  🛠️  Dev Tools   │
│  🔗 Interfaces │         │  🧪 Internal    │          │  📋 Polyfills   │
└─────────────────┘         └─────────────────┘          └─────────────────┘
         │                           │                            │
         ▼                           ▼                            ▼
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📚 Docs        │         │  🧪 Tests       │          │  🏗️  Build      │
│  💡 Examples    │         │  📊 Coverage    │          │  📦 Package     │
└─────────────────┘         └─────────────────┘          └─────────────────┘

Components: {}"#, components.join(", "))
    }

    fn create_mobile_diagram(components: &[String]) -> String {
        format!(r#"┌─────────────────────────────────────────────────────────────────────────────┐
│                           📱 Mobile App Architecture                           │
└─────────────────────────────────────────────────────────────────────────────┘

🎨 UI LAYER                  ⚙️  BUSINESS LAYER            🔌 PLATFORM LAYER
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📱 Screens     │         │  📋 State Mgmt  │          │  📱 Native APIs │
│  🎯 Components  │◀────────▶│  🔧 Services    │◀─────────▶│  📊 Storage     │
│  🎨 Styling     │         │  🌐 Network     │          │  📡 Sensors     │
│  🔄 Navigation  │         │  🧪 Validation  │          │  🔔 Push Notif  │
└─────────────────┘         └─────────────────┘          └─────────────────┘
         │                           │                            │
         ▼                           ▼                            ▼
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  🎯 User Inter. │         │  📊 Analytics   │          │  🏗️  Build      │
│  ♿ Accessibility│         │  🐛 Error Track │          │  📦 Distribution│
└─────────────────┘         └─────────────────┘          └─────────────────┘

Components: {}"#, components.join(", "))
    }

    fn create_generic_diagram(components: &[String]) -> String {
        format!(r#"┌─────────────────────────────────────────────────────────────────────────────┐
│                            🏗️  Project Architecture                            │
└─────────────────────────────────────────────────────────────────────────────┘

📥 INPUT/INTERFACE           🔄 CORE PROCESSING            📤 OUTPUT/RESULTS
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  👤 User Entry  │──────────▶│  ⚙️  Main Logic │──────────▶│  📄 Generated   │
│  📁 Data Input  │         │  🔧 Algorithms  │          │     Content     │
│  ⚙️  Config     │         │  🔍 Processing  │          │  📊 Results     │
│  🔗 External    │         │  📋 Business    │          │  💾 Storage     │
└─────────────────┘         └─────────────────┘          └─────────────────┘
         │                           │                            │
         ▼                           ▼                            ▼
┌─────────────────┐         ┌─────────────────┐          ┌─────────────────┐
│  📋 Validation  │         │  🌐 External    │          │  ✨ Formatting  │
│  🛡️  Security   │         │     Services    │          │  📈 Reporting   │
└─────────────────┘         └─────────────────┘          └─────────────────┘

Detected Components: {}"#, components.join(", "))
    }
}
