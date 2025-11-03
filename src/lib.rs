use crate::{exports::witmproxy::plugin::witm_plugin::{CapabilityProvider, Guest, HandleRequestResult, HandleResponseResult, Request, Response, PluginManifest}};

wit_bindgen::generate!({
    world: "witmproxy:plugin/plugin",
    generate_all
});

struct Plugin;

impl Guest for Plugin {
    fn manifest() -> PluginManifest {
        PluginManifest {
            name: "{{plugin-name}}".to_string(),
            namespace: "{{author}}".to_string(),
            author: "{{author}}".to_string(),
            version: "0.0.0".to_string(),
            description: "{{description}}".to_string(),
            metadata: vec![],
            capabilities: vec![
                "request".to_string(),
                "response".to_string(),
            ],
            cel: "true".to_string(),
            license: "MIT".to_string(),
            url: "https://example.com".to_string(),
            publickey: "todo".to_string(),
        }
    }

    fn handle_request(req: Request, cap: CapabilityProvider) -> HandleRequestResult {
        HandleRequestResult::Next(req)
    }

    fn handle_response(res: Response, cap: CapabilityProvider) -> HandleResponseResult {
        HandleResponseResult::Next(res)
    }
}

export!(Plugin);