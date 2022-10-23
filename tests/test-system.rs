use cucumber::{given, World, then, gherkin::Step};
use finitio::{schema::{any::Any, Type, nil::Nil, FinitioType}, common::FilePosition};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct FinitioWorld {
    r#type: Option<Type>,
    value: Option<serde_json::Value>
}

impl FinitioWorld {
  fn new() -> Self {
      Self {
          r#type: None,
          value: None
      }
  }
}

fn dress_value(world: &FinitioWorld) -> Result<serde_json::Value, snafu::Whatever> {
  let r#type = world.r#type.as_ref().unwrap();
  r#type.dress(world.value.as_ref().unwrap())
}

#[given("the type under test is Any")]
fn testing_any_type(world: &mut FinitioWorld) {
  world.r#type = Some(Type::Any(Any {
    position: FilePosition::inline()
  }))
}

#[given("the type under test is Nil")]
fn testing_nil_type(world: &mut FinitioWorld) {
  world.r#type = Some(Type::Nil(Nil {
    position: FilePosition::inline()
  }))
}

#[given(regex = r##"^I dress JSON's '"(\w+)"'"##)]
fn i_dress_json_string(world: &mut FinitioWorld, state: String) {
    match state.as_str() {
        v => {
          world.value = Some(serde_json::json!(v))
        }
    }
}

#[given("I dress JSON's 'null'")]
fn i_dress_json_nil(world: &mut FinitioWorld) {
    world.value = Some(serde_json::Value::Null)
}

#[given(regex = r"^I dress JSON's '(\d+)'")]
fn i_dress_json_number(world: &mut FinitioWorld, state: String) {
    match state.as_str() {
        v => {
          let num = v.parse::<i32>().unwrap();
          world.value = Some(serde_json::json!(num))
        }
    }
}

#[then(regex = r"result should be a representation for Nil")]
fn result_is_nil(world: &mut FinitioWorld) {
  let value = dress_value(world);

  match value {
    Ok(v) => {
      match v {
        serde_json::Value::Null => {},
        v => panic!("Nil representation expected, got {}", v)
      }
    },
    Err(e) => panic!("Value was not dressed: {}", e)
  }
}

#[then(regex = r"result should be the integer (\d+)")]
fn result_is_integer(world: &mut FinitioWorld, state: String) {
  let value = dress_value(world);

  match value {
    Ok(v) => {
      match v {
        resolver::Value::Number(n) => {
          let expected = state.as_str().parse::<i64>().unwrap();
          assert_eq!(n.as_i64(), Some(expected));
        },
        v => panic!("Expected Number 12, got: {}", v),
      }
    },
    Err(e) => panic!("Value was not dressed: {}", e)
  }
}

#[then(regex = r"it should be a TypeError")]
fn is_type_error(world: &mut FinitioWorld, step: &Step) {
  let value = dress_value(world);

  let error = match value {
    Ok(v) => {
      panic!("Dressing should have failed, got {}", v);
    },
    Err(e) => e
  };

  if let Some(table) = step.table.as_ref() {
    let message = table.rows
      .get(1)
      .expect("Step table should contain at least two rows: the header and an error row")
      .get(0)
      .expect("Error row should contain message");
    assert_eq!(&error.to_string(), message);
  }
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(FinitioWorld::run("tests/features/test-system"));
}
