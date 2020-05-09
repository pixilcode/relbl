use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // Files to match
    query: String,
    // Pattern for rename
    replace: String,
    // Where to perform operation
    #[structopt(short = "t", long = "target-dir", parse(from_os_str))]
    target_dir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let query = format_query(&args.query)?;
    let replace = format_replace(&args.replace);
    let files = get_matching_files(args.target_dir.unwrap_or(PathBuf::from(".")), &query)?;
    rename_files(files, &query, &replace)?;
    Ok(())
}

fn format_query(query: &str) -> Result<Regex, regex::Error> {
    Regex::new(&match (query.starts_with('^'), query.ends_with('$')) {
        (false, false) => String::from("^") + query + "$",
        (false, _) => String::from("^") + query,
        (_, false) => String::from(query) + "$",
        (_, _) => String::from(query),
    })
}

fn get_matching_files<P: AsRef<Path>>(p: P, match_str: &Regex) -> io::Result<Vec<fs::DirEntry>> {
    fs::read_dir(p)?
        .filter(
            |res| {
                res.as_ref()
                    .map(
                        |e| e.file_type().map(|f| f.is_file()).unwrap_or(true), // Don't silence errors
                    )
                    .unwrap_or(true)
            }, // Don't silence errors
        )
        .filter(|res| {
            res.as_ref()
                .map(|e| match_str.is_match(&e.file_name().to_string_lossy()))
                .unwrap_or(true)
        })
        .collect::<Result<Vec<_>, io::Error>>()
}

// For later, when certain inner variables are added
fn format_replace(replace: &str) -> String {
    replace.into()
}

fn rename_files(files: Vec<fs::DirEntry>, match_str: &Regex, replace: &str) -> io::Result<()> {
    files
        .into_iter()
        .map(|entry| entry.path())
        .map(|path| {
            let mut new_path = path.clone();
            new_path.set_file_name(
                match_str
                    .replace_all(
                        // Should already be checked to make sure it's a file
                        &path.file_name().map(|s| s.to_string_lossy()).unwrap(),
                        replace,
                    )
                    .into_owned(),
            );
            (path, new_path)
        })
        .map(|(old_path, new_path)| fs::rename(old_path, new_path))
        .collect::<io::Result<()>>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn test_get_files() -> Result<(), Box<std::error::Error>> {
        // Only find files in the current directory
        // that end with ".txt" ("a.txt")
        // Doesn't find "c.md" because it doesn't end with ".txt"
        // Doesn't find "b.txt" because it is in a sub-directory
        // Doesn't find "b" because it is a directory
        let expected = vec!["a.txt"];
        // Create a temporary directory for testing
        let dir = tempdir()?;
        let file_a_path = dir.path().join("a.txt");
        let file_a = fs::File::create(file_a_path)?;
        let file_c_path = dir.path().join("c.md");
        let file_c = fs::File::create(file_c_path)?;
        let dir_b_path = dir.path().join("b");
        fs::create_dir(dir_b_path)?;
        let file_b_path = dir.path().join("b").join("b.txt");
        let file_b = fs::File::create(file_b_path);
        let match_str = format_query(".*.txt").unwrap();

        let actual: Vec<_> = get_matching_files(dir.path(), &match_str)?
            .into_iter()
            .map(|f| f.file_name())
            .collect();
        assert_eq!(expected, actual);
        drop(file_a);
        drop(file_b);
        drop(file_c);
        dir.close()?;
        Ok(())
    }
    #[test]
    fn test_format_query() -> Result<(), Box<dyn std::error::Error>> {
        // All of these user queries should result in the same query
        // Query is "one or more 'a'"
        // '^' represents start of input
        // '$' represents end of input
        let queries = vec![
            format_query("a+")?,
            format_query("^a+")?,
            format_query("a+$")?,
            format_query("^a+$")?,
        ];
        for query in queries {
            assert!(query.is_match("a"));
            assert!(query.is_match("aa"));
            assert!(!query.is_match("baa"));
            assert!(!query.is_match("aab"));
            assert!(!query.is_match("baab"));
        }
        Ok(())
    }
}
