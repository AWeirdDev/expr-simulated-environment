use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use boa_engine::{
    context::Context, js_string, object, property::Attribute, JsError, JsValue, NativeFunction,
    Source,
};
use scraper::{Html, Selector};

macro_rules! empty_fn {
    () => {
        NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined()))
    };
}

struct DocState {
    data: Arc<RwLock<Html>>,
}

impl DocState {
    pub fn new(doc: Html) -> Self {
        Self {
            data: Arc::new(RwLock::new(doc)),
        }
    }

    pub fn clone_reference(&self) -> &'static Arc<RwLock<Html>> {
        Box::leak(Box::new(Arc::clone(&self.data)))
    }

    pub fn read_value(&self) -> RwLockReadGuard<'_, Html> {
        let guard = self.data.read().unwrap();
        guard
    }

    pub fn write_value(&self) -> RwLockWriteGuard<'_, Html> {
        let guard = self.data.write().unwrap();
        guard
    }

    pub fn manipulate<T>(&self, f: impl Fn(&mut Html) -> T) -> T {
        f(&mut self.write_value())
    }
}

fn add_console(context: &mut Context) {
    let object = object::ObjectInitializer::new(context)
        .function(empty_fn!(), js_string!("log"), 0)
        .function(empty_fn!(), js_string!("error"), 0)
        .function(empty_fn!(), js_string!("table"), 0)
        .function(empty_fn!(), js_string!("info"), 0)
        .function(empty_fn!(), js_string!("warn"), 0)
        .function(empty_fn!(), js_string!("debug"), 0)
        .function(empty_fn!(), js_string!("trace"), 0)
        .function(empty_fn!(), js_string!("dir"), 0)
        .function(empty_fn!(), js_string!("dirxml"), 0)
        .function(empty_fn!(), js_string!("group"), 0)
        .function(empty_fn!(), js_string!("groupCollapsed"), 0)
        .function(empty_fn!(), js_string!("groupEnd"), 0)
        .function(empty_fn!(), js_string!("time"), 0)
        .function(empty_fn!(), js_string!("timeEnd"), 0)
        .function(empty_fn!(), js_string!("count"), 0)
        .function(empty_fn!(), js_string!("assert"), 0)
        .function(empty_fn!(), js_string!("profile"), 0)
        .function(empty_fn!(), js_string!("profileEnd"), 0)
        .function(empty_fn!(), js_string!("profileEnd"), 0)
        .function(empty_fn!(), js_string!("clear"), 0)
        .build();

    context
        .register_global_property(js_string!("console"), object, Attribute::all())
        .expect("Failed to register");
}

fn add_document(context: &mut Context, document: &'static Arc<RwLock<Html>>) {
    fn query_selector(
        _this: &JsValue,
        args: &[JsValue],
        _context: &mut Context,
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
                return Ok(JsValue::String(js_string!(ele
                    .text()
                    .collect::<Vec<_>>()
                    .join(""))));
            }
        }

        Ok(JsValue::undefined())
    }

    let object = object::ObjectInitializer::new(context)
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
    match context.eval(Source::from_bytes("console.log()")) {
        Ok(res) => println!("{res:#?}"),
        Err(e) => println!("Error: {}", e),
    };
}
