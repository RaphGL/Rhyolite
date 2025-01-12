use html2md::parse_html; //html2md module to convert html to markdown
//use html2md::parse_html; //markdown_engine module to convert markdown to html
use pulldown_cmark::{html, Options, Parser}; //pulldown_cmark module to parse markdown
// use mdka; //mdka module to convert markdown to html
use regex::Regex; //regex module to use Regex type

pub fn html_to_markdown(html: &str) -> String {
    let re = Regex::new(r"<mark>(.*?)</mark>").unwrap();
    let new_html = re.replace_all(html, "==$1==").to_string();
    let markdown = parse_html(&new_html);
    markdown.replace(r"\==", "==")
}

pub fn markdown_to_html(markdown: &str) -> String {
    let re = Regex::new(r"==(.+?)==").unwrap();
    let processed_markdown = re.replace_all(markdown, r"<mark>$1</mark>").to_string();
    
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&processed_markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
