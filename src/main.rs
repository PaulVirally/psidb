mod database;
use std::collections::HashMap;
use ron::ser::{PrettyConfig, to_string_pretty};
use database::{Database, entry::directory::Directory};

fn get_serde_config() -> ron::ser::PrettyConfig {
    PrettyConfig::new()
        .depth_limit(5)
        .indentor("\t".to_owned())
        .struct_names(true)
}

fn main() {
    let dir = Directory {
        fpaths: vec!["a1".to_owned(), "a2".to_owned(), "a3".to_owned()],
        sub_dirs: vec![
            Directory {fpaths: vec!["b1".to_owned(), "b2".to_owned()],
            sub_dirs: vec![
                Directory {fpaths: vec!["ba1".to_owned(), "ba2".to_owned()], sub_dirs: vec![]}
            ]},
            Directory {fpaths: vec![],
            sub_dirs: vec![
                Directory {fpaths: vec!["ca1".to_owned()],
                sub_dirs: vec![
                    Directory {fpaths: vec!["caa1".to_owned()],
                    sub_dirs: vec![]}]}
            ]}
        ]
    };
    let md = Some(HashMap::from([("a".to_owned(), "0.123".to_owned()), ("b".to_owned(), "0.456".to_owned())]));

    let script_paths = vec!["test_dir/scripts/script1".to_owned(), "test_dir/scripts/script2".to_owned(), "test_dir/scripts/script3".to_owned(), "test_dir/scripts/script4".to_owned()];
    let script_args = vec![Some(vec!["-a 1".to_owned(), "-b 2".to_owned()]), Some(vec!["-a=1 -b=2".to_owned()]), Some(vec!["-a".to_owned(), "1".to_owned(), "-b".to_owned(), "2".to_owned()]), None];
    let script_git_hashes = vec![Some("0000111122223333444455556666777788889999".to_owned()), None, Some("0123456789abcdef0123456789abcdef01234567".to_owned()), None];

    let mut db = Database::new();
    db.add_data(dir, md).unwrap();
    db.add_transformation(script_paths, script_args, script_git_hashes, None).unwrap();

    println!("{}", to_string_pretty(&db, get_serde_config()).unwrap());
}
