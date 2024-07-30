use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use async_std::{io, task};
use serde::Deserialize;
use std::time::Duration;

#[derive(Default, Deserialize)]
struct Config {
    title: Option<String>,
    description: Option<String>,
}

#[derive(Default)]
struct State {
    input: String,
    config: Config,
}

#[init]
fn init(_config_dir: RString) -> State {
    let mut ron = String::new();

    let stdin = io::stdin();
    loop {
        let to = io::timeout(Duration::from_millis(10), async {
            let mut line = String::new();
            stdin.read_line(&mut line).await?;

            Ok(line)
        });

        let result = task::block_on(to);
        match result {
            Err(_) => break,
            Ok(line) => match line.as_str() {
                "" => break,
                _ => ron.push_str(line.as_str()),
            },
        }
    }

    let mut state = State::default();
    match ron::from_str(ron.as_str()) {
        Ok(r) => {
            state.config = r;
            state
        }
        Err(_) => state,
    }
}

#[handler]
fn handler(_match: Match, state: &State) -> HandleResult {
    HandleResult::Stdout(state.input.clone().into_bytes().into())
}

#[get_matches]
fn get_matches(input: RString, state: &mut State) -> RVec<Match> {
    state.input = input.into();

    vec![Match {
        id: ROption::RNone,
        icon: ROption::RNone,

        title: match &state.config.title {
            Some(r) => r.to_string().into(),
            None => "Enter pin".into(),
        },

        description: match &state.config.description {
            Some(d) => ROption::RSome(d.to_string().into()),
            None => ROption::RNone,
        },

        use_pango: false,
    }]
    .into()
}

#[info]
fn plugin_info() -> PluginInfo {
    PluginInfo {
        name: "Pinentry".into(),
        icon: "format-indent-more".into(),
    }
}
