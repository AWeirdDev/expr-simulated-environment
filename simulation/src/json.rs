use boa_engine::{
    js_string, object::ObjectInitializer, property::Attribute, Context, JsObject, JsValue,
    NativeFunction,
};

pub enum JsonValue {
    Property(JsValue),
    Function(NativeFunction),
}

pub struct Json {
    pub map: std::collections::HashMap<String, JsonValue>,
}

impl Json {
    pub fn build(&self, context: &mut Context) -> JsObject {
        let mut init = ObjectInitializer::new(context);
        for (key, value) in &self.map {
            match value {
                JsonValue::Property(v) => {
                    init.property(js_string!(key.to_string()), v.clone(), Attribute::all())
                }
                JsonValue::Function(of) => {
                    init.function(of.clone(), js_string!(key.to_string()), 0)
                }
            };
        }

        init.build()
    }

    pub fn object(&self, context: &mut Context) -> JsValue {
        JsValue::Object(self.build(context))
    }

    pub fn nested(&self, context: &mut Context) -> JsonValue {
        JsonValue::Property(JsValue::Object(self.build(context)))
    }
}

/// Creates a JSON object.
///
/// To **create an empty `JSON`**:
///
/// ```rust
/// let json: Json = json!(); // note that there is no "{}"
/// ```
///
/// To **create a `JSON` with properties or functions**:
///
/// ```rust
/// let json: Json = json!({
///     "key" => jsonstr!("value"),  // strings
///     "function" => empty_fn!(),  // empty function
///     "function2" => jsonfn!(fn (this, args, ctx) => {
///         Ok(json!({ "key" => jsonstr!("value") }).object(ctx))
///     }),
/// });
/// ```
#[macro_export]
macro_rules! json {
    ($({})?) => {
        simulation::json::Json { map: std::collections::HashMap::new() }
    };

    ({
        $($key:expr => $value:expr),* $(,)?
    }) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value);
            )*
            simulation::json::Json { map }
        }
    };
}

#[macro_export]
macro_rules! jsonstr {
    ($d:literal) => {
        simulation::json::JsonValue::Property(JsValue::String(js_string!($d)))
    };

    ($d:expr) => {
        simulation::json::JsonValue::Property(JsValue::String(js_string!($d)))
    };

    ($d:ident) => {
        simulation::json::JsonValue::Property(JsValue::String(js_string!($d)))
    };
}

#[macro_export]
macro_rules! jsonfn {
    (fn ($($args:pat),*) => $body:block) => {
        simulation::json::JsonValue::Function(NativeFunction::from_fn_ptr(|a, b, c| {
            let ($($args),*) = (a, b, c);
            $body
        }))
    };
}

#[macro_export]
macro_rules! empty_jsonfn {
    () => {
        simulation::json::JsonValue::Function(empty_fn!())
    };
}

#[macro_export]
macro_rules! empty_fn {
    () => {
        NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined()))
    };
}
