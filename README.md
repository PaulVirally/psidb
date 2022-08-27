# ψdb
ψdb is a sci(ψ)entific database management system designed to help you remember which datasets correspond to which experiments!

# TODO
* [ ] CLI
   - [x] Add data
   - [x] Add transforms
   - [x] Apply transform
   - [x] Chain transforms
   - [x] Link datasets
   - [x] --help messages
   - [ ] Man pages ([clap_mangen](https://github.com/clap-rs/clap/tree/master/clap_mangen))
* [ ] GUI
   - [ ] Visualize the entries in the database
   - [ ] Add data to the database through the GUI
      * [x] Init database
      * [x] Add data
      * [x] Add transforms
      * [ ] Apply transform
      * [x] Chain transforms
      * [x] Link datasets
      * [ ] Add templates for metadata formatting
   - [ ] Aesthetics
* [ ] Data integrity
    - [ ] Make a backup of the database before writing the new database
    - [ ] Maybe use SQL-style commits/transactions?