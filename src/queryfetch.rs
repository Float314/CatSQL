/* 
 * CatSQL - the low cost SQL Terminal!
 * Copyright (C) 2026, Float314. 
 * 
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/
use libsql::{Connection}; 
use tabled::builder::Builder;

/*
 * # fetchquery_print(db, query)
 * 
 * Takes db: Connection and query as input and prints the result, otherwise std::error::Error
 * if incase error. 
 * 
 * Dependencies - 
 * - Libsql
 * - tabled 
 * - colorize (if supported, to colour the Field name... Otherwise, leave it...)
 * 
*/
pub async fn fetchquery_print(db: Connection, query: String) -> Result<(), Box<dyn std::error::Error>> {
    // Execute the query asynchronously
    let mut rows = db.query(&query, ()).await?;
    
    let column_count = rows.column_count();
    
    // Extract column headers
    let mut headers = Vec::with_capacity(column_count as usize);
    for i in 0..column_count {
        let name = rows.column_name(i).unwrap_or(&format!("col_{}", i)).to_string();
        headers.push(name);
    }

    // Add column headers as the first record.
    let mut builder = Builder::default();
    builder.push_record(headers);

    // Iterate through all rows returned by the query
    while let Some(row) = rows.next().await? {
        let mut row_values = Vec::with_capacity(column_count as usize);
        for i in 0..column_count {
            let val_str = match row.get_value(i) {
                Ok(libsql::Value::Null) => "NULL".to_string(),
                Ok(libsql::Value::Integer(n)) => n.to_string(),
                Ok(libsql::Value::Real(f)) => f.to_string(),
                Ok(libsql::Value::Text(s)) => s,
                Ok(libsql::Value::Blob(_)) => "[BLOB]".to_string(),
                Err(_) => "".to_string(),
            };
            row_values.push(val_str);
        }
        builder.push_record(row_values);
    }

    // Build and print the table
    let table = builder.build();
    println!("{}", table);

    Ok(())
}
