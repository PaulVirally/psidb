use regex::Regex;
use chrono::Utc;

pub fn is_permutation_small<T>(lhs: &[T], rhs: &[T]) -> bool
where
    T: PartialEq,
{
    if lhs.len() != rhs.len() {
        return false;
    }

    // Horrible O(n^2) solution, but may be faster than using a HashMap for small arrays
    // TODO: Measure performance and see if it's worth it to use a HashMap
    for x in lhs {
        let mut found = false;
        for y in rhs {
            if x == y {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

pub fn parse_kv_opt_string(s: Option<&str>, num_default_entries: Option<usize>) -> Vec<Option<String>> {
    // a=1;b=2;c=3;;d=4 -> ["a=1", "b=2", "c=3", "", "d=4"]
    // "" -> [] or ["", "", "", ...] `depending on num_default_entries`
    let num_entries = num_default_entries.unwrap_or_default(); // Defaults to 0
    let default = ";".repeat(std::cmp::max(num_entries, 1) - 1); // Defaults to ";"
    s
        .unwrap_or_else(|| &default) // Avoid underflow issues
        .split(';')
        .map(|spl| {
            if spl.is_empty() {
                return None
            }
            Some(spl.to_owned())
        })
        .collect()
}

pub fn parse_args(s: &str, id: u64) -> String {
    // TODO: This should probably be part of the Database struct so that we can easily grab
    // something from the metadata for an arg like {psidb::md::my_metadata_key} -> my_metadata_value
    let date_re = Regex::new(r"\{psidb::date\}").unwrap();
    let time_re = Regex::new(r"\{psidb::time\}").unwrap();
    let id_re = Regex::new(r"\{psidb::id\}").unwrap();

    let s1 = date_re.replace_all(s, &Utc::now().format("%Y-%m-%d").to_string());
    let s2 = time_re.replace_all(&s1, &Utc::now().format("%H:%M:%S%:z").to_string());
    let s3 = id_re.replace_all(&s2, &id.to_string());
    s3.into_owned()
}

pub fn verify_file_path<T>(path_str: T) -> Result<(), Box<dyn std::error::Error>>
where T: AsRef<str> + AsRef<std::ffi::OsStr> + std::fmt::Display {
    let path = std::path::Path::new(&path_str);

    // Make sure the path exists
    if !path.exists() {
        println!("Error: {} does not exist", path_str);
        return Err(format!("{} does not exist", path_str).into());
    }

    // Make sure the path is a file
    if !path.is_file() {
        println!("Error: {} is not a file", path_str);
        return Err(format!("{} is not a file", path_str).into());
    }

    Ok(())
}

pub fn safe_git_checkout_commit(repo: &mut git2::Repository, commit: git2::Oid) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let head_name = repo.head()?.name().unwrap().to_string();

    // Stash the current state of the repo
    let stash_res = repo.stash_save(&git2::Signature::now("psidb", "psidb@psidb.com")?, "psidb::stash", None);
    let did_stash = stash_res.is_ok();

    // Set the repo to the HEAD commit
    if repo.set_head_detached(commit).is_err() {
        if did_stash {
            repo.stash_pop(0, None)?;
        }
        return Err(format!("Failed to set HEAD to {}", commit.to_string()).into());
    }

    Ok((did_stash, head_name))
}

pub fn safe_git_checkout_commit_restore(repo: &mut git2::Repository, did_stash: bool, head_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Set the HEAD to the previous state
    repo.set_head(head_name)?;

    // Pop the stash
    if did_stash {
        repo.stash_pop(0, None)?;
    }

    Ok(())
}