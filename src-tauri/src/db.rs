use serde::{Deserialize, Serialize};
use sqlite::{Connection, State};
use std::{collections::HashMap, error::Error};
pub struct SqliteDB {
    connection: Connection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLecture {
    pub name: String,
    pub lid: String,
    pub updated: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub lid: String,
    pub secid: u32,
    pub lineno: u32,
    pub data: String,
}

pub trait LectureStore: Sized {
    fn new() -> Result<Self, Box<dyn Error>>;
    fn get_lectures(&self) -> Vec<CreateLecture>;
    fn insert_lecture(&self, creation_data: CreateLecture) -> Result<(), Box<dyn Error>>;
    fn insert_line(&self, line: Line) -> Result<(), Box<dyn Error>>;
    fn get_lecture(&self, lid: &str) -> Result<String, Box<dyn Error>>;
    fn delete_line(&self, line: Line) -> Result<(), Box<dyn Error>>;
    fn get_lecture_struct(&self, lid: &str) -> Result<LectureData, Box<dyn std::error::Error>>;
}
impl LectureStore for SqliteDB {
    fn new() -> Result<Self, Box<dyn Error>> {
        let connection = sqlite::open("lectures.db")?;

        let query = "
        CREATE TABLE IF NOT EXISTS lectures (
        name VARCHAR(128) NOT NULL,
        lid VARCHAR(16) PRIMARY KEY,
        updated_at INT
        );
        CREATE TABLE IF NOT EXISTS lines (
            lid VARCHAR(16) NOT NULL,
            secid INT NOT NULL,
            line_no INT NOT NULL,
            content TEXT,
            PRIMARY KEY (lid, secid, line_no)
        );
        ";
        connection.execute(query).unwrap();
        Ok(Self { connection })
    }
    fn insert_line(&self, line: Line) -> Result<(), Box<dyn Error>> {
        // Step 1: Shift existing lines below this one down
        let shift_query = format!(
            "UPDATE lines
             SET line_no = line_no + 1
             WHERE lid = '{}' AND secid = {} AND line_no >= {};",
            line.lid.replace('\'', "''"),
            line.secid,
            line.lineno
        );
        self.connection.execute(&shift_query)?;

        // Step 2: Insert the new line
        let insert_query = format!(
            "INSERT INTO lines (lid, secid, line_no, content)
             VALUES ('{}', {}, {}, '{}');",
            line.lid.replace('\'', "''"),
            line.secid,
            line.lineno,
            line.data.replace('\'', "''")
        );
        self.connection.execute(&insert_query)?;
        Ok(())
    }

    fn delete_line(&self, line: Line) -> Result<(), Box<dyn Error>> {
        // Step 1: Delete the line
        let delete_query = format!(
            "DELETE FROM lines
             WHERE lid = '{}' AND secid = {} AND line_no = {};",
            line.lid.replace('\'', "''"),
            line.secid,
            line.lineno
        );
        self.connection.execute(&delete_query)?;

        // Step 2: Shift lines up
        let shift_query = format!(
            "UPDATE lines
             SET line_no = line_no - 1
             WHERE lid = '{}' AND secid = {} AND line_no > {};",
            line.lid.replace('\'', "''"),
            line.secid,
            line.lineno
        );
        self.connection.execute(&shift_query)?;
        Ok(())
    }

    fn get_lectures(&self) -> Vec<CreateLecture> {
        let query = "SELECT name, lid, updated_at FROM lectures;";
        let mut lecture_list: Vec<CreateLecture> = Vec::new();

        self.connection
            .iterate(query, |pairs| {
                let mut name = String::new();
                let mut lid = String::new();
                let mut updated = 0;
                for &(col, val) in pairs.iter() {
                    match col {
                        "name" => name = val.unwrap_or("").to_string(),
                        "lid" => lid = val.unwrap_or("").to_string(),
                        "updated_at" => {
                            updated = val.unwrap_or("0").parse::<u64>().unwrap_or_else(|_| 0);
                        }
                        _ => {}
                    }
                }

                lecture_list.push(CreateLecture { name, lid, updated });
                true
            })
            .unwrap();

        lecture_list
    }

    fn insert_lecture(&self, creation_data: CreateLecture) -> Result<(), Box<dyn Error>> {
        let query = format!(
            "INSERT INTO lectures (name, lid, updated_at) VALUES ('{}', '{}', '{}');",
            creation_data.name.replace('\'', "''"),
            creation_data.lid.replace('\'', "''"),
            creation_data.updated
        );
        self.connection.execute(&query)?;
        Ok(())
    }
    fn get_lecture(&self, lid: &str) -> Result<String, Box<dyn Error>> {
        let mut statement = self
            .connection
            .prepare("SELECT content FROM lines WHERE lid = ? ORDER BY secid, line_no;")?;
        statement.bind((1, lid))?;
        let mut result = String::new();

        while let sqlite::State::Row = statement.next()? {
            let content: String = statement.read(0)?;
            result.push_str(&content);
            result.push('\n');
        }
        Ok(result)
    }
    fn get_lecture_struct(&self, lid: &str) -> Result<LectureData, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT secid, line_no, content FROM lines WHERE lid = ? ORDER BY secid, line_no",
        )?;
        stmt.bind((1, lid))?;

        let mut sections_map: HashMap<i64, Vec<(i64, String)>> = HashMap::new();

        while let State::Row = stmt.next()? {
            let secid: i64 = stmt.read(0)?;
            let line_no: i64 = stmt.read(1)?;
            let content: String = stmt.read(2)?;

            sections_map
                .entry(secid)
                .or_default()
                .push((line_no, content));
        }

        let mut sections: Vec<Section> = Vec::new();

        for (_secid, mut lines) in sections_map {
            // Sort lines just in case
            lines.sort_by_key(|(line_no, _)| *line_no);

            // Get section name and the rest of the lines
            let (name, body_lines) = if let Some((first_no, first_content)) = lines.get(0) {
                if *first_no == -1 {
                    let name = first_content.clone();
                    let rest = lines.iter().skip(1).map(|(_, l)| l.clone()).collect();
                    (name, rest)
                } else {
                    // fallback if no -1 line
                    (
                        "Untitled Section".to_string(),
                        lines.into_iter().map(|(_, l)| l).collect(),
                    )
                }
            } else {
                ("Untitled Section".to_string(), Vec::new())
            };

            sections.push(Section {
                name,
                lines: body_lines,
            });
        }

        Ok(LectureData { sections })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub lines: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LectureData {
    pub sections: Vec<Section>,
}
