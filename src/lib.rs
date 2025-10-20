use zed_extension_api as zed;

struct WaspExtension;

impl zed::Extension for WaspExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(WaspExtension);
