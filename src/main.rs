extern crate env_logger;
extern crate handlebars;

use handlebars::{template, Handlebars};
use log::info;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;
use std::{collections::HashMap, env};
use tiny_http::{Header, Response, Server};

//TODO - using register_templates_directory

fn main() -> Result<(), Box<dyn Error>> {
    initiate_logging();

    info!("Welcome to Faculty Site.");

    let handlebars = register_templates()?;

    let app_data = construct_data()?;

    // output html file
    let mut output_file = File::create("./website/index.html")?;
    handlebars.render_to_write("app", &app_data, &mut output_file)?;
    info!("website/index.html generated");

    // ==========================
    // run server to run dev_mode
    let hbs = Arc::new(handlebars);

    let address = std::env::var("ADDRESS").expect("Missed 'ADDRESS' environment variable");
    info!("address: {}", address);
    let server = Server::http(address.as_str()).expect("Failed to start server.");
    info!("Tiny server started");

    for req in server.incoming_requests() {
        let mut output_file = File::create("./website/index_live.html")?;
        hbs.render_to_write("app", &app_data, &mut output_file)?;
        info!("=== LIVE render_to_write generated ===");

        let result = hbs
            .render("app", &app_data)
            .unwrap_or_else(|e| e.to_string());

        //"content-type:text/html; charset=UTF-8"
        let content_type_header = "Content-Type: text/html; charset=UTF-8"
            .parse::<Header>()
            .unwrap();
        req.respond(Response::from_string(result).with_header(content_type_header))
            .unwrap();
        info!("=== LIVE render generated ===");

        let pwd = std::env::var("PWD")?;
        info!("pwd: {}", pwd);
    }

    Ok(())
}

/// Construct data which mapping inner variables inside .hbs files with related .hbs files.
fn construct_data() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut data: HashMap<String, String> = HashMap::new();

    // templates mapping data
    data.insert("app".to_string(), "app.hbs".to_string());
    data.insert("footer".to_string(), "footer.hbs".to_string());
    data.insert("header".to_string(), "header.hbs".to_string());
    data.insert("main".to_string(), "main.hbs".to_string());
    data.insert("leftside".to_string(), "leftside.hbs".to_string());
    data.insert("rightside".to_string(), "rightside.hbs".to_string());
    data.insert(
        "carousel_images".to_string(),
        "carousel_images.hbs".to_string(),
    );
    data.insert("news".to_string(), "news.hbs".to_string());
    data.insert("courses_card".to_string(), "courses_card.hbs".to_string());
    data.insert("events_card".to_string(), "events_card.hbs".to_string());
    data.insert("faculity_card".to_string(), "faculity_card.hbs".to_string());
    data.insert("about".to_string(), "about.hbs".to_string());

    // Logic data
    data.insert("is_admin".to_string(), "true".to_string());

    dbg!(data.clone());

    Ok(data)
}

/// Register Handlebars templates
fn register_templates() -> Result<Handlebars<'static>, Box<dyn Error>> {
    let templates_dir =
        std::env::var("TEMPLATES_DIR").expect("Missed 'TEMPLATES_DIR' environment variable");
    info!("TEMPLATES_DIR dir: {}", templates_dir);

    let base_templates =
        std::env::var("BASE_TEMPLATES").expect("Missed 'BASE_TEMPLATES' environment variable");
    info!("BASE_TEMPLATES dir: {}", base_templates);
    let pages_templates =
        std::env::var("PAGES_TEMPLATES").expect("Missed 'PAGES_TEMPLATES' environment variable");
    info!("PAGES_TEMPLATES dir: {}", pages_templates);
    let parts_templates =
        std::env::var("PARTS_TEMPLATES").expect("Missed 'PARTS_TEMPLATES' environment variable");
    info!("PARTS templates dir: {}", parts_templates);

    let mut handlebars = Handlebars::new();

    // Set dev-mode to live trace handlebars files
    handlebars.set_dev_mode(true);

    // Fill all templates in a HashMap
    let templates_map: HashMap<String, String> = HashMap::from([
        ("app".to_string(), base_templates.clone() + "app.hbs"),
        ("header".to_string(), base_templates.clone() + "header.hbs"),
        ("footer".to_string(), base_templates.clone() + "footer.hbs"),
        ("main".to_string(), base_templates.clone() + "main.hbs"),
        (
            "leftside".to_string(),
            base_templates.clone() + "leftside.hbs",
        ),
        (
            "rightside".to_string(),
            base_templates.clone() + "rightside.hbs",
        ),
        ("admin".to_string(), pages_templates.clone() + "admin.hbs"),
        (
            "carousel_images".to_string(),
            parts_templates.clone() + "carousel_images.hbs",
        ),
        ("news".to_string(), parts_templates.clone() + "news.hbs"),
        (
            "courses_card".to_string(),
            parts_templates.clone() + "courses_card.hbs",
        ),
        (
            "events_card".to_string(),
            parts_templates.clone() + "events_card.hbs",
        ),
        (
            "faculity_card".to_string(),
            parts_templates.clone() + "faculity_card.hbs",
        ),
        ("about".to_string(), parts_templates.clone() + "about.hbs"),
    ]);

    // register templates in handlebars
    templates_map.iter().for_each(|(key, path)| {
        handlebars.register_template_file(key, path).unwrap();
        info!("`{}` template resgistered, path: {}", key, path);
    });

    // // register templates by directory
    // handlebars
    //     .register_templates_directory(".hbs", templates_dir)
    //     .unwrap();
    // let templates = handlebars.get_templates();
    // info!("registered templates count: {}", templates.capacity());
    // templates.iter().for_each(|(_name, _template)| {
    //     //=
    //     info!("=> template name: {}", _name);
    // });

    // dbg!(templates);

    Ok(handlebars)
}

// fn read_handlebars_vars() {
//     std::env::var("PWD").is_err()
// }

fn initiate_logging() {
    // dotenv().ok();

    let env = dotenv::from_filename(".env").expect("'.env' not found.");
    dbg!(env);

    if std::env::var("PWD").is_err() {
        std::env::set_var("PWD", env!("CARGO_MANIFEST_DIR"));
        let pwd = std::env::var("PWD").unwrap();
        dbg!(pwd);
    }

    std::env::set_var("RUST_LOG", "debug, actix_web=debug");
    env_logger::init();
}
