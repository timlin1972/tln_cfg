use anstream::println;
use owo_colors::OwoColorize as _;

use common::{cfg, plugin};

const MODULE: &str = "cfg";

#[derive(Debug)]
pub struct Plugin {}

impl Plugin {
    pub fn new(_tx: &crossbeam_channel::Sender<String>) -> Plugin {
        println!("[{}] Loading...", MODULE.blue());

        Plugin {}
    }
}

impl plugin::Plugin for Plugin {
    fn name(&self) -> &str {
        MODULE
    }

    fn action(&mut self, action: &str, data: &str, _data2: &str) -> String {
        match action {
            "set" => {
                let cmd_update: serde_json::Value = serde_json::from_str(data).unwrap();

                if let Some(name) = cmd_update.get("name") {
                    cfg::set_name(name.as_str().unwrap()).unwrap();
                }
            }
            "add" => {
                let cmd_update: serde_json::Value = serde_json::from_str(data).unwrap();

                if let Some(alias) = cmd_update.get("alias") {
                    if let Some(cmd) = cmd_update.get("cmd") {
                        let alias = cfg::Alias {
                            cmd: cmd.as_str().unwrap().to_owned(),
                            alias: alias.as_str().unwrap().to_owned(),
                        };
                        cfg::add_alias(alias).unwrap();
                    }
                }
            }
            _ => (),
        }

        "action".to_owned()
    }

    fn show(&mut self) -> String {
        println!("[{}]", MODULE.blue());

        let mut show = String::new();

        show += "action plugin cfg set '{\"name\": \"new_name\"}'\n";
        show += "\tSet name\n\n";

        show += "action plugin cfg add '{\"alias\": \"!t\", \"cmd\": \"show plugins\"}'\n";
        show += "\tAlias '!t' to 'show plugins'\n\n";

        show += "action plugin cfg add '{\"alias\": \"!startshell\", \"cmd\": \"send plugin command shell start\"}'\n";
        show += "\tAlias '!startshell' to 'send plugin command start shell'\n\n";

        show += "action plugin cfg add '{\"alias\": \"!stopshell\", \"cmd\": \"send plugin command shell stop\"}'\n";
        show += "\tAlias '!stopshell' to 'send plugin command stop shell'\n\n";

        show += "status plugin cfg\n";
        show += "\tShow the cfg plugin status. For listing 'alias' especially.\n\n";

        println!("{show}");

        show
    }

    fn status(&mut self) -> String {
        println!("[{}]", MODULE.blue());

        let cfg = cfg::get_cfg();

        let mut status = String::new();

        status += &"Name:\n".blue().to_string();
        status += &format!("\t{}\n", cfg.name);

        status += &"Startup:\n".blue().to_string();
        cfg.startup
            .iter()
            .for_each(|item| status += &format!("\t{item}\n"));

        status += &"Polling:\n".blue().to_string();
        cfg.polling
            .iter()
            .for_each(|item| status += &format!("\t{item}\n"));

        status += &"Aliases:\n".blue().to_string();
        cfg.aliases
            .iter()
            .for_each(|item| status += &format!("\t{}: {}\n", item.alias, item.cmd));

        println!("{status}");

        status
    }

    fn unload(&mut self) -> String {
        println!("[{}] Unload", MODULE.blue());

        "unload".to_owned()
    }
}

#[no_mangle]
pub extern "C" fn create_plugin(
    tx: &crossbeam_channel::Sender<String>,
) -> *mut plugin::PluginWrapper {
    let plugin = Box::new(Plugin::new(tx));
    Box::into_raw(Box::new(plugin::PluginWrapper::new(plugin)))
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn unload_plugin(wrapper: *mut plugin::PluginWrapper) {
    if !wrapper.is_null() {
        unsafe {
            let _ = Box::from_raw(wrapper);
        }
    }
}
