use urlencoding;

pub struct Util {}

impl Util {
    pub fn get_url_compatible_string(text: String) -> String {
        let comp_string = urlencoding::encode(&text).to_string();

        comp_string
    }

    pub fn beautify_text_in_html(text: &str) -> String {
        let mut text = urlencoding::decode(text).unwrap().to_string();
        text = text.replace("<blockquote>", "").replace("</blockquote>", "").replace("<p>", "").replace("</p>", "").replace("<code>", "<<--\n").replace("</code>", "\n-->>").replace("<pre>", "").replace("</pre>", "").replace("<strong>", "").replace("</strong>", "");

        text
    }
}
