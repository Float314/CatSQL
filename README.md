# CatSQL - The low cost SQL Command line 
> Because cats are low maintainance animals :D 

CatSQL is a low-effort SQL Command line which implements 
- SQL Parsing (Thrugh sqlparser-rs)
- Preety Result and error appearance (through colorize)
- sqlite backend (libsql for rust) which is preety damn fast
- its damn cheap
- Single binary o' a few megabytes

# Memory Comparison With MySQL 

CatSQL has an SQLite Backend, making it easy for mem management! 
Key Comaprion i found was these - 
1. Idle Memory Usage: According to reddit, idle time for MySQL 
   Without InnoDB is around 70-100 Mb. CatSQL is much niche cause 
   it only uses about 5 Mb of RAM. (Mb - Megabits). Htop confirmed
   this. 
2. 1000 records in a database show about 300 Mb of RAM Usage, 
   while CatSQL only showed around 15-17 Mb Of RAM (Without 
   InnoDB). Htop also confirmed this
3. Binary Size for `Release` (i.e. `cargo build --release`) size
   is about 12 Mb. MySQL Shell alone is about 200 Mb. This makes 
   it easy if you are setting up a small computer labaratory
4. Almost no external .dll files*. LibSQL is built in (infact i 
   didnt install libsql nor SQLite in my lappy yet.). However, 
   Operating Systems essentials may require stuff like `kernel32.dll`
   which the rust ecosystem cant take care of ig... Comparatively 
   MySQL Could come with other external dependencies and dll files.
5. **Why CatSQL and why not SQLite?** SQLite is also an amazing
   path for a learning point perspective **but** SQLite's Dialect
   is a lil' bit different from MySQL Dialect and ANSI Dialect. 
   CatSQL Closes this gap by providing MySQL, ANSI, PostgreSQL and 
   its own sqlparser dialect all bundled in a binary of a few 
   megabytes. 

> [!WARNING]
> These tests are honest comparisons with Windows. Platforms 
> may vary! Please cross-check these comparisons if possible. 

# To-Do 

- [x] Implement Config files
- [ ] Organise functions (accuracy)
- [x] release v0.2
- [x] reconfigure cargo.lock
- [x] Implement `.ast` command

# License
GNU LGPL v3.
