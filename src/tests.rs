#[cfg(test)]
mod tests {
    use crate::constants::RECURSE_NOT_DIR;
    use crate::*;
    use std::fs::File;
    use std::string::String;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            Ok(vec![String::from("safe, fast, productive.")]),
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            Ok(vec![String::from("Rust:"), String::from("Trust me.")]),
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn whole_file() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            Ok(vec![String::from("Rust:"), String::from("Trust me.")]),
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn recursive_normal() {
        let query = String::from("warts");
        let filename = String::from("testFiles");
        let mut case = false;
        let verbose = false;
        let whole = false;
        let recurse = true;

        assert_eq!(
            Ok(vec![
                String::from("stalwarts"),
                String::from("thwarts"),
                String::from("warts")
            ]),
            search_recursive(&Config {
                query,
                filename,
                case,
                verbose,
                whole,
                recurse,
            })
        );

        let query = String::from("Who");
        let filename = String::from("testFiles");
        case = true;

        assert_eq!(
            Ok(vec![
                String::from("I’m Nobody ! Who are you ?"),
                String::from("2I’m Nobody ! Who are you ?")
            ]),
            search_recursive(&Config {
                query,
                filename,
                case,
                verbose,
                whole,
                recurse,
            })
        );
    }

    #[test]
    fn recursive_notdir() {
        let query = String::from("thing");
        let filename = String::from("testFiles/file.txt");
        let case = false;
        let verbose = false;
        let whole = false;
        let recurse = true;

        assert_eq!(
            Err(RECURSE_NOT_DIR.to_string()),
            search_recursive(&Config {
                query,
                filename,
                case,
                verbose,
                whole,
                recurse,
            })
        )
    }

    #[test]
    fn recursive_failfile() {
        let f = &File::open(Path::new("lockedFile/opened.txt")).unwrap();
        File::lock(f).expect("test file opened.txt not found");
        let query = String::from("thing");
        let filename = String::from("lockedFile");
        let case = false;
        let verbose = false;
        let whole = false;
        let recurse = true;

        assert!(
            search_recursive(&Config {
                query,
                filename,
                case,
                verbose,
                whole,
                recurse,
            })
            .is_err()
        );

        File::unlock(f).unwrap();
    }

    #[test]
    fn str_bool() {
        let tr = "true".to_string();
        let fa = "false".to_string();

        assert_eq!(true, string_to_bool(tr));
        assert_eq!(false, string_to_bool(fa));
        assert_eq!(false, string_to_bool("notABoolean".to_string()));
    }

    #[test]
    fn conf_create() {
        let query = String::from("q");
        let filename = String::from("f");
        let case = string_to_bool("true".to_string());
        let verbose = string_to_bool("false".to_string());
        let whole = string_to_bool("true".to_string());
        let recurse = string_to_bool("false".to_string());

        assert_eq!(
            Ok(Config {
                query,
                filename,
                case,
                verbose,
                whole,
                recurse,
            }),
            Config::new(&[
                String::new(),
                String::from("q"),
                String::from("f"),
                String::from("true"),
                String::from("false"),
                String::from("true"),
                String::from("maybe")
            ])
        );
    }
}
