use std::fs;
use crate::Error;
use crate::Todo;
use crate::DB_PATH;

pub fn read_db() -> Result<Vec<Todo>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Todo> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn write_db(data: Vec<Todo>) -> Result<Vec<Todo>, Error> {
    fs::write(DB_PATH, &serde_json::to_vec(&data)?)?;
    Ok(data)
}
