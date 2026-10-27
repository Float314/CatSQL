/*
 * Copyright (C) 2026, Float314. Licensed under The GNU LGPL v3. 
 * 
 * Permission to use and modify this software is permitted under 
 * some conditions under the GNU LGPL v3. A Copy of license is
 * provided in the source and binary. If not, please refer to 
 * <https://www.gnu.org/licenses/lgpl-3.0.en.html>
*/

/* Create command in rust0 */
use libsql; 

// ------------- Tokenizer section ----------------

/**
 * What type of Create command are we going to create
 * 
 * Eg - Create Table... <- Table enum selected
 *      Create Database .... <- Database enum selected. 
 */
#[derive(Debug, PartialEq)]
pub enum CreateCommandType {
    Database, 
    Table
}

/*
 * Field property (coloumn property) 
 * Primary Key -> PrimaryKey
 * Not Blank -> NOT NULL 
 * Unique ->  Unique (Can be null iirc sql does this (The backend is SQL i dont have yto worry bout that))
*/
#[derive(Debug, PartialEq)]
pub enum FieldProperty { 
    PrimaryKey,
    NotNull,
    Unique
}

/* I Think i need no explanation for this */
#[derive(Debug, PartialEq)]
pub enum ColType {
    Integer,
    Varchar, 
    Text, 
    Floating, 
    Boolean,
    DateTime // unix time
}

/* Struct for coloumn info. Also self explanatory :/  */
#[derive(Debug, PartialEq)]
pub struct ColInfo {
    Name: String, 
    Type: ColType,
    TypeLen: i32,
    Property: Vec<FieldProperty>
}

/**
 * What arguments are we going to pass to the Create command
 * 
 * If na -> just an enum ig (used for database)
 * 
 * If yes -> then we have a vector of strings that we will pass to the command
 * 
 * damn i love documenting my code 
 */
#[derive(Debug, PartialEq)]
pub enum CreateCommandArguments {
    Na,
    Ye(Vec<ColInfo>)
}

/**
 * Structure of Create command: 
 * 
 * Type: CreateCommandType -> Enum
 * Name: String -> the name of table
 * Args: CreateCommandArguments -> Enum {No, ye(Vec<String>)}
 */
#[derive(Debug, PartialEq)]
pub struct CommandCreate {
    Type: CreateCommandType, 
    Name: String,
    Args: CreateCommandArguments
}