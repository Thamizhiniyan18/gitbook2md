use regex::Regex;
use std::fs;
use std::path::PathBuf;
use url::Url;

pub fn code(content: String) -> String {
    let regex_pattern = match Regex::new(r#"(?s)\{%\scode\s\w+?="\w+?"\s%}(.+?)\{%\sendcode\s%\}"#)
    {
        Ok(pattern) => pattern,
        Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
    };

    let mut replaced_content = content.clone();

    for capture in regex_pattern.captures_iter(&content) {
        let code = capture.get(1).unwrap().as_str();

        replaced_content = replaced_content.replace(capture.get(0).unwrap().as_str(), &code)
    }

    return replaced_content;
}

pub fn embed_urls(content: String) -> String {
    let regex_pattern = match Regex::new(r#"(?s)\{%\sembed\surl="(.+?)"\s%\}"#) {
        Ok(pattern) => pattern,
        Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
    };

    let mut replaced_content = content.clone();

    for capture in regex_pattern.captures_iter(&content) {
        let url = capture.get(1).unwrap().as_str();

        replaced_content = replaced_content.replace(
            capture.get(0).unwrap().as_str(),
            &format!("[{}]({})", url, url),
        )
    }

    let regex_pattern = match Regex::new(r#"\{%\sendembed\s%\}"#) {
        Ok(pattern) => pattern,
        Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
    };

    replaced_content = regex_pattern.replace_all(&replaced_content, "").to_string();

    return replaced_content;
}

pub fn file_links(content: String, parent_dir: &PathBuf, asset_dir: &PathBuf) -> String {
    let regex_pattern = match Regex::new(r#"(?s)\{%\sfile\ssrc="(.+?)"\s%\}"#) {
        Ok(pattern) => pattern,
        Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
    };

    let mut replaced_content = content.clone();

    for capture in regex_pattern.captures_iter(&content) {
        let src = capture.get(1).unwrap().as_str();

        if Url::parse(src).is_ok() {
            continue;
        };

        let src_absolute = match parent_dir.join(&src).canonicalize() {
            Ok(value) => value,
            Err(error) => {
                println!("Parent Dir: {}", parent_dir.display());
                println!("Src: {}", &src);
                panic!("Error: {}", error)
            }
        };

        let name = src_absolute
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(" ", "_");

        match fs::copy(
            src_absolute,
            format!("{}/{}", asset_dir.to_str().unwrap(), name),
        ) {
            Ok(_) => {}
            Err(why) => panic!("Failed to write file: {}", why),
        }

        replaced_content = replaced_content.replace(
            capture.get(0).unwrap().as_str(),
            &format!("[{}](assets/{})", name, name),
        )
    }

    return replaced_content;
}

pub fn hints(content: String) -> String {
    let regex_pattern =
        match Regex::new(r#"(?s)\{%\shint\sstyle="\w+?"\s%\}(.+?)\{%\sendhint\s%\}"#) {
            Ok(pattern) => pattern,
            Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
        };

    let mut replaced_content = content.clone();

    for capture in regex_pattern.captures_iter(&content) {
        let note = capture
            .get(1)
            .unwrap()
            .as_str()
            .lines()
            .map(|line| format!("> {}", line))
            .collect::<Vec<String>>()
            .join("\n");

        replaced_content = replaced_content.replace(capture.get(0).unwrap().as_str(), &note)
    }

    return replaced_content;
}

pub fn images(content: String, parent_dir: &PathBuf, asset_dir: &PathBuf) -> String {
    let regex_pattern = match Regex::new(
        r#"(?s)<figure><img\ssrc="(.+?)"\salt=".+?><figcaption></figcaption></figure>|<img\ssrc="(.+?)"\salt=".+?>"#,
    ) {
        Ok(pattern) => pattern,
        Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
    };

    let mut replaced_content = content.clone();

    for capture in regex_pattern.captures_iter(&content) {
        let src = capture
            .get(1)
            .or(capture.get(2))
            .map(|m| m.as_str())
            .unwrap_or("");

        if Url::parse(src).is_ok() {
            continue;
        };

        let src_absolute = match parent_dir.join(&src).canonicalize() {
            Ok(value) => value,
            Err(error) => {
                println!("Parent Dir: {}", parent_dir.display());
                println!("Src: {}", &src);
                panic!("Error: {}", error)
            }
        };

        let name = src_absolute
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(" ", "_");

        match fs::copy(
            src_absolute,
            format!("{}/{}", asset_dir.to_str().unwrap(), name),
        ) {
            Ok(_) => {}
            Err(why) => panic!("Failed to write file: {}", why),
        }

        replaced_content = replaced_content.replace(
            capture.get(0).unwrap().as_str(),
            &format!("![image](assets/{})", name),
        )
    }

    return replaced_content;
}

pub fn tabs(content: String) -> String {
    let regex_pattern_tabs = match Regex::new(r#"(?s)\{%\stabs\s%\}(.+?)\{%\sendtabs\s%\}"#) {
        Ok(pattern) => pattern,
        Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
    };

    let mut replaced_content = content.clone();

    for capture in regex_pattern_tabs.captures_iter(&content) {
        let tabs = capture.get(1).unwrap().as_str();

        let regex_pattern_tab =
            match Regex::new(r#"(?s)\{%\stab\stitle="(\w+?)"\s%\}(.+?)\{%\sendtab\s%\}"#) {
                Ok(pattern) => pattern,
                Err(why) => panic!("Couldn't parse the given regex pattern: {}", why),
            };

        for tab in regex_pattern_tab.captures_iter(&tabs) {
            let language = tab.get(1).unwrap().as_str();
            let code = tab.get(2).unwrap().as_str();

            replaced_content = replaced_content.replace(
                tab.get(0).unwrap().as_str(),
                &format!("### Using {}\n{}", language, code),
            );
        }
    }

    replaced_content = replaced_content.replace("{% tabs %}\n", "");
    replaced_content = replaced_content.replace("{% endtabs %}\n", "");

    return replaced_content;
}
