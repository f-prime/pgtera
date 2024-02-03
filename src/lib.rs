use pgrx::{extension_sql, pg_extern, Spi};
use serde::{Deserialize, Serialize};
use serde_json::*;
use tera::Tera;

pgrx::pg_module_magic!();

extension_sql!(
    r#"

    create schema if not exists pgtera;
    create table if not exists pgtera.render_path (
        id serial primary key,
        path text unique not null
    );

"#,
    name = "create_pgtera_render_path_table"
);

#[derive(Serialize, Deserialize, Debug)]
struct CtxInput {
    name: String,
    value: Value,
}

fn get_render_path() -> String {
    if let Ok(Some(r)) =
        Spi::get_one::<String>("select path from pgtera.render_path order by id desc")
    {
        r
    } else {
        panic!("No render path is set.");
    }
}

#[pg_extern]
fn pgtera_set_render_path(file_path: &'static str) {
    let query = format!(
        "insert into pgtera.render_path (path) values ($FP${}$FP$)",
        file_path
    );
    if let Ok(_) = Spi::run(&query) {
        println!("Successfully initialized pgtera.");
    } else {
        panic!("Could not initialize pgtera.");
    }
}

#[pg_extern]
fn pgtera_render(template_name: &'static str, ctx: String) -> String {
    let ctx_values: Vec<CtxInput> =
        serde_json::from_str(ctx.as_str()).expect("Context type is invalid.");

    let rp = get_render_path();
    let tera = Tera::new(rp.as_str()).expect("There was an issue initializing Tera.");

    let mut context = tera::Context::new();

    ctx_values.iter().for_each(|x| {
        context.insert(&x.name, &json!(x.value));
    });

    let rendered = tera
        .render(template_name, &context)
        .expect("Error while rendering template.");

    rendered
}
