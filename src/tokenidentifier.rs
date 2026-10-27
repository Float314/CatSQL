/* Token identifier
 * Basically what this does is that it takes a string and 
 * converts it into tokens for AST to generate it later. 
 * 
 * For example - 
 * 
 * SELECT SUM(MARKS) FROM STUDENTS WHERE MARKS > 10;
 * 
 * It will split into a struct of tokens based on the query. 
 * For Select it is going to be - 
 * 
 * struct select {
 *      Rows: Vec<String> // i might need to reconsider this 
 *      Tablename: String
 *      Conditions: String
 * }
 * 
 * Ofc, this is going to change based on how i plan to interpret it. 
*/

use shlex; 

/**
 * Basically so, every module would give a pub struct for tokenizer lilke
 * for create -> create.rs and all at shit. I still dont know how rust 
 * works but it is what it is. Wiriting all that in C++ Would be painful
 * even though im a C++ Glazer. 
 * 
 * thisll be my shot
 */

#[derive(Debug, PartialEq, Clone)]
pub struct TokenSelect {
    pub tablename: String,
    pub selection: Option<String>
}

#[derive(Debug, Clone)]
pub enum SelectItem {
    Wildcard, // * , select everything like Select * 
    Coloumn(String) // self explanatory
}

