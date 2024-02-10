/// Replaces Markdown links in the input text with just the text.
///
/// # Arguments
///
/// * `input` - A string slice that contains Markdown text with links.
///
/// # Returns
///
/// A string with Markdown links replaced by their corresponding text.
///
/// # Examples
///
/// ```
/// use remove_markdown_links::remove_markdown_links;
/// let markdown = "This is a [link](https://www.example.com) to Example.".to_string();
/// let result = remove_markdown_links(&markdown);
/// assert_eq!(result, "This is a link to Example.");
/// ```
pub fn remove_markdown_links(input: &str) -> String {
    let re = regex::Regex::new(r"\[([^\[\]]+)\]\(([^)]+)\)").unwrap();
    let replaced = re.replace_all(input, |caps: &regex::Captures| {
        caps.get(1).unwrap().as_str().to_string()
    });
    replaced.to_string()
}
