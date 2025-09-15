use std::fs;
use std::time::UNIX_EPOCH;
use chrono::{
    DateTime,
    Local,
    Utc
};
use tabled::{
    Table,
    Tabled,
    settings::{
        Style,
        object::Columns,
        Alignment
    },

};

#[derive(Tabled)]
#[derive(Debug)]
pub struct FileInfo {
    name: String,
    size: u64,
    created_at: DateTime<Local>,
    date_modified: DateTime<Local>
}

pub fn list_files(path: &str) -> Result<(), Box<dyn std::error::Error>>  {
    let my_files = fs::read_dir(path)?;
    println!("File Name\t\t File Size()\t\t Created At:");
    let mut vec_files: Vec<FileInfo> = Vec::new();


    for i in my_files {

        let myfile: fs::DirEntry = i?;
        let file_metadata = myfile.metadata()?;


        let created_at = file_metadata.created()?;
        let modified_at = file_metadata.modified()?;

        let dt_local: DateTime<Local> = created_at.into();
        let modified_dt: DateTime<Local> = modified_at.into();

        let filename = myfile.file_name().into_string().unwrap();
        let file_kind = file_metadata.is_file();

        let file_dir = if file_kind {
            "file"
        } else {
            "dir"
        };


        let file_info: FileInfo = FileInfo {
            name: filename,
            size: file_metadata.len(),
            created_at: dt_local,
            date_modified: modified_dt
        };

        vec_files.push(file_info);

    }

    let mut my_table = Table::new(vec_files);
    my_table.with(Style::modern());
    my_table.modify(Columns::first(), Alignment::left());

    println!("{}", my_table);
    Ok(())
}

pub fn create_backup_dir(path: &str) {

