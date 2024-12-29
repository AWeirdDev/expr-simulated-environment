use std::sync::{Arc, RwLock};

use boa_engine::{
    context::Context, js_string, object::ObjectInitializer, property::Attribute, JsError, JsValue,
    NativeFunction, Source,
};
use scraper::{Html, Selector};

mod doc_state;
mod engine;
mod json;

use doc_state::DocState;

fn add_console(context: &mut Context) {
    let data = json!({
        "log" => empty_jsonfn!(),
        "error" => empty_jsonfn!(),
        "table" => empty_jsonfn!(),
        "info" => empty_jsonfn!(),
        "warn" => empty_jsonfn!(),
        "debug" => empty_jsonfn!(),
        "trace" => empty_jsonfn!(),
        "dir" => empty_jsonfn!(),
        "dirxml" => empty_jsonfn!(),
        "group" => empty_jsonfn!(),
        "groupCollapsed" => empty_jsonfn!(),
        "groupEnd" => empty_jsonfn!(),
        "time" => empty_jsonfn!(),
        "timeEnd" => empty_jsonfn!(),
        "count" => empty_jsonfn!(),
        "assert" => empty_jsonfn!(),
        "profile" => empty_jsonfn!(),
        "profileEnd" => empty_jsonfn!(),
        "profileEnd" => empty_jsonfn!(),
        "clear" => empty_jsonfn!(),
    })
    .build(context);

    context
        .register_global_property(js_string!("console"), data, Attribute::all())
        .expect("Failed to register");
}

fn add_document(context: &mut Context, document: &'static Arc<RwLock<Html>>) {
    fn query_selector(
        _this: &JsValue,
        args: &[JsValue],
        ctx: &mut Context,
        html: &Arc<RwLock<Html>>,
    ) -> Result<JsValue, JsError> {
        if let Some(v) = args.get(0) {
            let target = v
                .as_string()
                .expect("Expected string")
                .to_std_string()
                .expect("Expected UTF-16");
            let selector = Selector::parse(target.as_str()).expect("Expected valid selector");

            let reader = html.read().unwrap();
            let res = reader.select(&selector);

            if let Some(ele) = res.into_iter().nth(0) {
                return Ok(json!({
                    "textContent" => jsonstr!(ele.text().collect::<String>().as_str()),
                    "innerHTML" => jsonstr!(ele.html()),
                    "tagName" => jsonstr!(ele.value().name()),
                    "attributes" => json!({}).nested(ctx),
                })
                .object(ctx));
            }
        }

        Ok(json!().object(ctx))
    }

    let object = ObjectInitializer::new(context)
        .function(
            NativeFunction::from_copy_closure_with_captures(
                |a, b, captures, c| query_selector(a, b, c, captures),
                document,
            ),
            js_string!("querySelector"),
            0,
        )
        .build();

    context
        .register_global_property(js_string!("document"), object, Attribute::all())
        .expect("Failed to register");
}

fn make_context(state: DocState) -> Result<Context, JsError> {
    let mut context = Context::default();
    add_console(&mut context);
    add_document(&mut context, state.clone_reference());

    Ok(context)
}

fn main() {
    let html = r#"<html>
<head>
    <script>
        console.log(10 + 10);
    </script>
</head>
<body>
    <h1>Hello, world!</h1>
    <p>This is a paragraph.</p>
</body>
</html>"#;

    let document = Html::parse_document(html);
    let state = DocState::new(document);

    let mut context = make_context(state).unwrap();
    match context.eval(Source::from_bytes(
        "let st = document.querySelector('h1');st.textContent = 'aaa';st.textContent",
    )) {
        Ok(res) => println!("{res:#?}"),
        Err(e) => println!("Error: {}", e),
    };
}
