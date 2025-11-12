use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

struct WaspExtension;

impl zed::Extension for WaspExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Check if user has configured a custom wasp executable path
        let wasp_path = if let Ok(lsp_settings) = LspSettings::for_worktree("waspls", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    path
                } else {
                    "wasp".to_string()
                }
            } else {
                "wasp".to_string()
            }
        } else {
            "wasp".to_string()
        };

        // Try to find wasp in PATH
        let command = worktree
            .which(&wasp_path)
            .unwrap_or_else(|| wasp_path.clone());

        // Get shell environment for the language server
        let (platform, _arch) = zed::current_platform();
        let env = match platform {
            zed::Os::Mac | zed::Os::Linux => worktree.shell_env(),
            zed::Os::Windows => vec![],
        };

        Ok(zed::Command {
            command,
            args: vec!["waspls".to_string(), "--stdio".to_string()],
            env,
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("waspls", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(WaspExtension);
