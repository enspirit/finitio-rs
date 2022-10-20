use finitio::fio;
use finitio::schema;
use finitio::js;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd_parse()
}

const TEST_SCHEMA: &str = r#"
@import finitio/data

Null = Nil
Any = .
Number = .Number
Integer = Number
String = .String

Seq = [Integer]
Set = {Integer}

Complex = {{[Set]}}

Union = Number | Integer | Nil

Struct = <Number | Integer, Nil>

PosInteger = Integer(i | i > 0)

Tuple = {
    name          :   String,
    optional_age  :?  Number
}
Relation = {{ name: String, age: Number }}

Password = String( s | s.length >= 8 )
User = {
    name: String,
    password: Password,
    confirm: Password
}( t | t.password == t.confirm )
"#;

fn cmd_parse() -> Result<(), Box<dyn std::error::Error>> {
    // Parse FIO file
    let mut fios: Vec<fio::Schema> = Vec::new();
    let fio = fio::parse_schema(TEST_SCHEMA).map_err(|e| format!("{}", e))?;
    fios.push(fio);

    for fio in fios.iter() {
        let res = schema::Schema::from_fio(fio)?;
        // println!("{:?}", res);
        js::Schema::from_schema(&res);
    }

    Ok(())
}
