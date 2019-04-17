use mongodb::cursor::Cursor;
use tera::Context;
use tera::*;

pub fn render_view(cursor: &mut Cursor) -> String {
    // build templates
    let tera = compile_templates!("templates/**/*");

    info!("DB fetch");

    // create row
    let mut data = vec![];

    while cursor.has_next().unwrap() {
        match cursor.next() {
            Some(Ok(doc)) => data.push(doc),
            Some(Err(_)) => info!("failed to get next"),
            None => info!("empty db reponse"),
        };
    }

    let mut context = Context::new();
    context.insert("data", &data);

    info!("Rendering template");

    tera.render("main.html", &context).unwrap()
}
