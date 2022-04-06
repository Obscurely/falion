use urlencoding;

pub struct Util {}

impl Util {
    pub fn get_url_compatible_string(text: String) -> String {
        let comp_string = urlencoding::encode(&text).to_string();

        comp_string
    }

    pub fn beautify_text_in_html(text: &str) -> String {
        let mut text = urlencoding::decode(text).unwrap().to_string();
        text = text
            .replace("<blockquote>", "")
            .replace("</blockquote>", "")
            .replace("<p>", "")
            .replace("</p>", "")
            .replace("<code>", "<<--\n")
            .replace("</code>", "\n-->>")
            .replace("<pre>", "")
            .replace("</pre>", "")
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("<h1>", "\n\t")
            .replace("</h1>", "\n")
            .replace("<h2>", "\n\t")
            .replace("</h2>", "\n")
            .replace("<h3>", "\n\t")
            .replace("</h3>", "\n")
            .replace("<h4>", "\n\t")
            .replace("</h4>", "\n")
            .replace("<hr>", "==\n")
            .replace("</hr>", "\n==")
            .replace("<em>", "")
            .replace("</em>", "")
            .replace("<hr />", "\n");

        text
    }
}
