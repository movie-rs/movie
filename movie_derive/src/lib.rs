#![doc(html_root_url = "https://movie.pzmarzly.pl")]

//! `movie_derive` - crate containing procedural macros.

extern crate proc_macro;
use proc_macro::TokenStream;

use std::collections::HashMap;

#[proc_macro]
/// Macro that generates module `ActorName`, which contains structs `Actor` and `Input`.
pub fn actor(input: TokenStream) -> TokenStream {
    actor_internal(input, false)
}
#[proc_macro]
/// This version of `actor!` will `eprintln!` how it sees the input and what code it generated.
pub fn actor_dbg(input: TokenStream) -> TokenStream {
    actor_internal(input, true)
}

// Input: "SimplestActor input : Ping , on_message : Ping => Pong ,"
fn actor_internal(input: TokenStream, debug: bool) -> TokenStream {
    let input = input.to_string();

    if debug {
        eprintln!("Input:");
        eprintln!("{}", input);
    }

    // PART ONE
    // Locate attributes inside input string

    let supported_attributes = vec![
        // attr name, default value
        ("public_visibility", ""),
        ("docs", ""),
        ("input", ""),
        ("input_derive", ""),
        ("data", ""),
        ("on_init", ""),
        ("on_message", ""),
        ("tick_interval", "100"),
        ("on_tick", ""),
        ("on_stop", ""),
        ("spawner", "std::thread::spawn"),
        ("spawner_return_type", "std::thread::JoinHandle<()>"),
        ("custom_code", ""),
    ];

    // locations = [(start, attr name, start of content), ...]
    let mut locations = vec![(0, "name", 0)];
    for attr in &supported_attributes {
        let attr = attr.0;
        // Any of the following cases may happen:
        let search_strings = &[
            format!("\n{} :\n", attr),
            format!(" {} :\n", attr),
            format!("\n{} : ", attr),
            format!(" {} : ", attr),
            format!("\n{}\n:\n", attr),
            format!(" {}\n:\n", attr),
            format!("\n{}\n: ", attr),
            format!(" {}\n: ", attr),
        ];
        for search_str in search_strings {
            let pos = input.find(search_str);
            if let Some(pos) = pos {
                locations.push((pos, attr, pos + search_str.len()));
                break;
            }
        }
    }

    // PART TWO
    // Turn array of locations to array of values

    // attrs = {
    //     "input": "Ping ,"
    //     "name": "SimplestActor",
    //     "on_message": "Ping => Pong ,",
    // }
    let mut attrs: HashMap<&str, String> = HashMap::new();

    locations.sort_unstable();

    // locations = [(start, attr name, start of content), ...]
    for i in 0..locations.len() {
        let value = if i == locations.len() - 1 {
            // We are parsing the last segment
            &input[locations[i].2..]
        } else {
            // Start of the next segment is this one's ends
            &input[locations[i].2..locations[i + 1].0]
        };
        attrs.insert(locations[i].1, value.to_string());
    }

    if debug {
        eprintln!("Parsed attributes:");
        eprintln!("{:?}", &attrs);
    }

    // PART THREE
    // Assign default values for missing optional supported attrs

    for attr in &supported_attributes {
        attrs.entry(attr.0).or_insert(attr.1.to_string());
    }

    // PART FOUR
    // Generate code

    // Prepare strings used later
    let public_visibility = if attrs["public_visibility"].contains("true") {
        "pub".to_string()
    } else {
        "".to_string()
    };
    let input_derive = if attrs["input_derive"].len() > 0 {
        format!("#[derive({})]", attrs["input_derive"])
    } else {
        "".to_string()
    };

    // TODO: Consider rewriting to quote!()
    let output = format!(
        "
        {docs}
        {public_visibility} mod {name} {{
        use super::*;

        {custom_code}

        pub struct Actor {{
            {data}
        }}

        {input_derive}
        pub enum Input {{
            {input}
        }}

        pub type Handle = movie::Handle<{spawner_return_type}, Input>;

        impl Actor {{
            pub fn start(mut self) -> Handle
            {{
                let (tx_ota, rx_ota) = std::sync::mpsc::channel(); // owner-to-actor data
                let (tx_kill, rx_kill) = std::sync::mpsc::channel(); // owner-to-actor stop requests
                let handle = {spawner}(move || {{
                    {on_init} // on_init is not separated as this is the simplest way to
                              // implement thread-local data. This may change in later (breaking)
                              // updates
                    let mut running = true;
                    while running {{
                        while let Ok(message) = rx_ota.try_recv() {{
                            use Input::*;
                            match message {{
                                {on_message}
                            }};
                        }}
                        if let Ok(_) = rx_kill.try_recv() {{
                            running = false;
                            {{
                                {on_stop}
                            }};
                        }}
                        {{
                            {on_tick}
                        }};
                        use std::thread::sleep;
                        use std::time::Duration;
                        sleep(Duration::from_millis({tick_interval}));
                    }}
                }});
                movie::Handle {{
                    join_handle: handle,
                    tx: tx_ota,
                    kill: tx_kill,
                }}
            }}
        }}
        }}",
        // attrs
        name = attrs["name"],
        docs = attrs["docs"],
        input = attrs["input"],
        data = attrs["data"],
        on_init = attrs["on_init"],
        on_message = attrs["on_message"],
        tick_interval = attrs["tick_interval"],
        on_tick = attrs["on_tick"],
        on_stop = attrs["on_stop"],
        spawner = attrs["spawner"],
        spawner_return_type = attrs["spawner_return_type"],
        custom_code = attrs["custom_code"],
        // prepared strings
        public_visibility = public_visibility,
        input_derive = input_derive,
    );
    if debug {
        eprintln!("Generated code:");
        eprintln!("{}", output);
    }
    output.parse().unwrap()
}
