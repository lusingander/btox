use std::fmt::Write;

use itsuki::zero_indexed_enum;
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use ratatui_macros::vertical;
use serde_json::value::RawValue;

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{Msg, PageMsg, UnicodeMsg},
    pages::{page::Page, util},
    widget::{
        scroll::{ScrollOutput, ScrollOutputState},
        select::Select,
    },
};

pub struct UnicodePage {
    focused: bool,
    cur: CurrentStatus,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    eod_sel: EncodeOrDecodeSelect,
    format_sel: InputFormatSelect,
    escape_sel: EscapeSelect,
    input: String,
    input_state: ScrollOutputState,
    output: String,
    status: String,
}

impl UnicodePage {
    pub fn new(focused: bool) -> UnicodePage {
        UnicodePage {
            focused,
            cur: CurrentStatus::default(),
        }
    }
}

#[derive(Default)]
#[zero_indexed_enum]
enum PageItems {
    #[default]
    EncodeOrDecode,
    InputFormat,
    Escape,
    Input,
    Output,
}

#[derive(Default)]
#[zero_indexed_enum]
enum EncodeOrDecodeSelect {
    #[default]
    Encode,
    Decode,
}

impl EncodeOrDecodeSelect {
    fn_str_map! {
        EncodeOrDecodeSelect::Encode => "Encode",
        EncodeOrDecodeSelect::Decode => "Decode",
    }

    fn_next_prev_mut! {}
}

#[derive(Default)]
#[zero_indexed_enum]
enum InputFormatSelect {
    #[default]
    Text,
    JsonString,
    JsonDocument,
}

impl InputFormatSelect {
    fn_str_map! {
        InputFormatSelect::Text => "Text",
        InputFormatSelect::JsonString => "JSON string",
        InputFormatSelect::JsonDocument => "JSON document",
    }

    fn_next_prev_mut! {}
}

#[derive(Default)]
#[zero_indexed_enum]
enum EscapeSelect {
    #[default]
    NonAscii,
    All,
}

impl EscapeSelect {
    fn_str_map! {
        EscapeSelect::NonAscii => "Non-ASCII",
        EscapeSelect::All => "All characters",
    }

    fn_next_prev_mut! {}
}

impl Page for UnicodePage {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = match key {
            key_code_char!('j') | key_code!(KeyCode::Down) => UnicodeMsg::SelectNextItem,
            key_code_char!('k') | key_code!(KeyCode::Up) => UnicodeMsg::SelectPrevItem,
            key_code_char!('l') | key_code!(KeyCode::Right) => UnicodeMsg::CurrentItemSelectNext,
            key_code_char!('h') | key_code!(KeyCode::Left) => UnicodeMsg::CurrentItemSelectPrev,
            key_code_char!('e', Ctrl) => UnicodeMsg::ScrollDown,
            key_code_char!('y', Ctrl) => UnicodeMsg::ScrollUp,
            key_code_char!('y') => UnicodeMsg::Copy,
            key_code_char!('p') => UnicodeMsg::Paste,
            _ => return None,
        };
        Some(Msg::Page(PageMsg::Unicode(msg)))
    }

    fn update(&mut self, msg: PageMsg) -> Option<Msg> {
        if let PageMsg::Unicode(msg) = msg {
            match msg {
                UnicodeMsg::SelectNextItem => self.select_next_item(),
                UnicodeMsg::SelectPrevItem => self.select_prev_item(),
                UnicodeMsg::CurrentItemSelectNext => self.current_item_select_next(),
                UnicodeMsg::CurrentItemSelectPrev => self.current_item_select_prev(),
                UnicodeMsg::ScrollDown => self.scroll_down(),
                UnicodeMsg::ScrollUp => self.scroll_up(),
                UnicodeMsg::Copy => return self.copy_to_clipboard(),
                UnicodeMsg::Paste => return self.paste_from_clipboard(),
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        if area.width < 26 || area.height < 11 {
            f.render_widget(Paragraph::new("Area too small"), area);
            return;
        }
        let chunks = vertical![==2, ==2, ==2, >=0, ==1, >=0].split(area);

        let eod_sel = Select::new(
            EncodeOrDecodeSelect::strings_vec(),
            self.cur.eod_sel.val(),
            self.cur.item == PageItems::EncodeOrDecode,
            self.focused,
        );
        f.render_widget(eod_sel, chunks[0]);

        let format_sel = Select::new(
            InputFormatSelect::strings_vec(),
            self.cur.format_sel.val(),
            self.cur.item == PageItems::InputFormat,
            self.focused,
        );
        f.render_widget(format_sel, chunks[1]);

        let escape_sel = Select::new(
            EscapeSelect::strings_vec(),
            self.cur.escape_sel.val(),
            self.cur.item == PageItems::Escape,
            self.focused && self.cur.eod_sel == EncodeOrDecodeSelect::Encode,
        );
        f.render_widget(escape_sel, chunks[2]);

        let input = ScrollOutput::new(
            self.cur.input.clone(),
            self.focused,
            self.cur.item == PageItems::Input,
        )
        .title("Input");
        f.render_stateful_widget(input, chunks[3], &mut self.cur.input_state);

        if !self.cur.status.is_empty() {
            let status = Paragraph::new(self.cur.status.as_str()).block(
                Block::default()
                    .borders(Borders::empty())
                    .style(Style::default().fg(Color::Red))
                    .padding(Padding::horizontal(1)),
            );
            f.render_widget(status, chunks[4]);
        }

        let output_style = if self.focused {
            if self.cur.item == PageItems::Output {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let output = Paragraph::new(self.cur.output.clone())
            .block(
                Block::bordered()
                    .style(output_style)
                    .title("Output")
                    .padding(Padding::horizontal(1)),
            )
            .wrap(Wrap { trim: false });
        f.render_widget(output, chunks[5]);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn helps(&self) -> Vec<&str> {
        let mut helps = vec!["<j/k> Select item"];
        if matches!(
            self.cur.item,
            PageItems::EncodeOrDecode | PageItems::InputFormat | PageItems::Escape
        ) {
            helps.push("<h/l> Select current item value");
        }
        if self.cur.item == PageItems::Output {
            helps.push("<y> Copy to clipboard");
        }
        if self.cur.item == PageItems::Input {
            helps.push("<C-e/C-y> Scroll down/up");
            helps.push("<p> Paste from clipboard");
        }
        helps
    }
}

impl UnicodePage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
        if self.cur.item == PageItems::Escape && self.cur.eod_sel == EncodeOrDecodeSelect::Decode {
            self.cur.item = self.cur.item.next();
        }
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
        if self.cur.item == PageItems::Escape && self.cur.eod_sel == EncodeOrDecodeSelect::Decode {
            self.cur.item = self.cur.item.prev();
        }
    }

    fn current_item_select_next(&mut self) {
        match self.cur.item {
            PageItems::EncodeOrDecode => self.cur.eod_sel.next_mut(),
            PageItems::InputFormat => self.cur.format_sel.next_mut(),
            PageItems::Escape if self.cur.eod_sel == EncodeOrDecodeSelect::Encode => {
                self.cur.escape_sel.next_mut();
            }
            _ => return,
        }
        self.update_output();
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::EncodeOrDecode => self.cur.eod_sel.prev_mut(),
            PageItems::InputFormat => self.cur.format_sel.prev_mut(),
            PageItems::Escape if self.cur.eod_sel == EncodeOrDecodeSelect::Encode => {
                self.cur.escape_sel.prev_mut();
            }
            _ => return,
        }
        self.update_output();
    }

    fn scroll_down(&mut self) {
        if self.cur.item == PageItems::Input && !self.cur.input.is_empty() {
            self.cur.input_state.scroll_down();
        }
    }

    fn scroll_up(&mut self) {
        if self.cur.item == PageItems::Input && !self.cur.input.is_empty() {
            self.cur.input_state.scroll_up();
        }
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        if self.cur.item == PageItems::Output {
            util::copy_to_clipboard(&self.cur.output)
        } else {
            None
        }
    }

    fn paste_from_clipboard(&mut self) -> Option<Msg> {
        if self.cur.item != PageItems::Input {
            return None;
        }
        if let Some(input) = util::paste_from_clipboard() {
            self.cur.input = input;
            self.cur.input_state = ScrollOutputState::default();
            self.update_output();
            None
        } else {
            Some(Msg::NotifyError("Paste from clipboard failed".into()))
        }
    }

    fn update_output(&mut self) {
        match calculate_unicode(
            &self.cur.input,
            self.cur.eod_sel,
            self.cur.format_sel,
            self.cur.escape_sel,
        ) {
            Ok(output) => {
                self.cur.output = output;
                self.cur.status.clear();
            }
            Err(error) => {
                self.cur.output.clear();
                self.cur.status = error.to_string();
            }
        }
    }
}

fn calculate_unicode(
    input: &str,
    eod_sel: EncodeOrDecodeSelect,
    format_sel: InputFormatSelect,
    escape_sel: EscapeSelect,
) -> serde_json::Result<String> {
    let escapes = match eod_sel {
        EncodeOrDecodeSelect::Encode => Some(escape_sel),
        EncodeOrDecodeSelect::Decode => None,
    };

    match format_sel {
        InputFormatSelect::Text => match eod_sel {
            EncodeOrDecodeSelect::Encode => Ok(escape_string(input, escapes)),
            EncodeOrDecodeSelect::Decode => serde_json::from_str(&format!("\"{input}\"")),
        },
        InputFormatSelect::JsonString => {
            // Validate that the top-level value is a string, then preserve its whitespace.
            serde_json::from_str::<String>(input)?;
            transform_json_strings(input, escapes)
        }
        InputFormatSelect::JsonDocument => {
            // Validate syntax without converting numbers or discarding duplicate keys.
            serde_json::from_str::<&RawValue>(input)?;
            transform_json_strings(input, escapes)
        }
    }
}

// None keeps Unicode readable while retaining the escapes required inside JSON strings.
fn escape_string(input: &str, escapes: Option<EscapeSelect>) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        let unicode_escape = match escapes {
            Some(EscapeSelect::All) => true,
            Some(EscapeSelect::NonAscii) => !ch.is_ascii(),
            None => false,
        };
        if unicode_escape {
            for unit in ch.encode_utf16(&mut [0; 2]) {
                write!(output, "\\u{unit:04x}").unwrap();
            }
            continue;
        }
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{00}'..='\u{1f}' => write!(output, "\\u{:04x}", ch as u32).unwrap(),
            _ => output.push(ch),
        }
    }
    output
}

// The caller validates the document before scanning, so quotes delimit JSON string tokens.
fn transform_json_strings(
    input: &str,
    escapes: Option<EscapeSelect>,
) -> serde_json::Result<String> {
    let mut output = String::new();
    let mut start = None;
    let mut escaped = false;
    let mut copied = 0;

    for (index, ch) in input.char_indices() {
        if let Some(string_start) = start {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                let text: String = serde_json::from_str(&input[string_start..=index])?;
                output.push_str(&input[copied..string_start]);
                output.push('"');
                output.push_str(&escape_string(&text, escapes));
                output.push('"');
                copied = index + 1;
                start = None;
            }
        } else if ch == '"' {
            start = Some(index);
        }
    }
    output.push_str(&input[copied..]);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("abc 123 /", "abc 123 /")]
    #[case("Aあ😀", r"A\u3042\ud83d\ude00")]
    #[case("\u{ffff}\u{10000}\u{10ffff}", r"\uffff\ud800\udc00\udbff\udfff")]
    #[case("\"\\\n\r\t\u{08}\u{0c}\0\u{1f}", r#"\"\\\n\r\t\b\f\u0000\u001f"#)]
    #[case(r"\u3042", r"\\u3042")]
    fn test_encode_text(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            calculate_unicode(
                input,
                EncodeOrDecodeSelect::Encode,
                InputFormatSelect::Text,
                EscapeSelect::NonAscii,
            )
            .unwrap(),
            expected
        );
    }

    #[test]
    fn test_encode_all_characters() {
        let output = calculate_unicode(
            "A あ😀\"\\\n/",
            EncodeOrDecodeSelect::Encode,
            InputFormatSelect::Text,
            EscapeSelect::All,
        )
        .unwrap();
        assert_eq!(
            output,
            r"\u0041\u0020\u3042\ud83d\ude00\u0022\u005c\u000a\u002f"
        );
    }

    #[rstest]
    #[case("", "")]
    #[case("Aあ", "Aあ")]
    #[case(r"A\u3042\ud83d\ude00", "Aあ😀")]
    #[case(r"\uD83D\uDE00", "😀")]
    #[case(r"\uDBFF\uDFFF", "\u{10ffff}")]
    #[case(r#"\"\\\/\b\f\n\r\t\u0000"#, "\"\\/\u{08}\u{0c}\n\r\t\0")]
    #[case(r"\\u3042", r"\u3042")]
    #[case(r"\u005cu3042", r"\u3042")]
    fn test_decode_text(#[case] input: &str, #[case] expected: &str) {
        for escapes in [EscapeSelect::NonAscii, EscapeSelect::All] {
            assert_eq!(
                calculate_unicode(
                    input,
                    EncodeOrDecodeSelect::Decode,
                    InputFormatSelect::Text,
                    escapes,
                )
                .unwrap(),
                expected
            );
        }
    }

    #[rstest]
    #[case(r"\u123")]
    #[case(r"\uGGGG")]
    #[case(r"\u{3042}")]
    #[case(r"\U00003042")]
    #[case(r"\x41")]
    #[case("\\")]
    #[case("\"")]
    #[case("a\nb")]
    #[case("\0")]
    #[case(r"\uD800")]
    #[case(r"\uDC00")]
    #[case(r"\uD800\u0041")]
    #[case(r"\uD800\uD800")]
    fn test_decode_text_rejects_invalid_escapes(#[case] input: &str) {
        assert!(calculate_unicode(
            input,
            EncodeOrDecodeSelect::Decode,
            InputFormatSelect::Text,
            EscapeSelect::NonAscii,
        )
        .is_err());
    }

    #[test]
    fn test_json_string_keeps_quotes_and_whitespace() {
        let input = " \n\"Aあ😀\"\t";
        for (escapes, expected) in [
            (EscapeSelect::NonAscii, " \n\"A\\u3042\\ud83d\\ude00\"\t"),
            (EscapeSelect::All, " \n\"\\u0041\\u3042\\ud83d\\ude00\"\t"),
        ] {
            let encoded = calculate_unicode(
                input,
                EncodeOrDecodeSelect::Encode,
                InputFormatSelect::JsonString,
                escapes,
            )
            .unwrap();
            assert_eq!(encoded, expected);
            assert_eq!(
                calculate_unicode(
                    &encoded,
                    EncodeOrDecodeSelect::Decode,
                    InputFormatSelect::JsonString,
                    escapes,
                )
                .unwrap(),
                input
            );
        }
    }

    #[test]
    fn test_json_decode_keeps_required_escapes() {
        let input = r#""\u0022\u005c\u000a\u0000\uD83D\uDE00""#;
        let expected = r#""\"\\\n\u0000😀""#;
        for format in [
            InputFormatSelect::JsonString,
            InputFormatSelect::JsonDocument,
        ] {
            let output = calculate_unicode(
                input,
                EncodeOrDecodeSelect::Decode,
                format,
                EscapeSelect::All,
            )
            .unwrap();
            assert_eq!(output, expected);
            assert_eq!(
                serde_json::from_str::<String>(&output).unwrap(),
                serde_json::from_str::<String>(input).unwrap()
            );
        }
    }

    #[test]
    fn test_json_document_preserves_non_string_tokens() {
        let input = concat!(
            " \n{\r\n  \"名\": \"Aあ😀\",\n",
            "  \"list\": [true, null, {\"名\": \"B\"}],\n",
            "  \"n\": 90071992547409931234567890, \"n\": -0.00e+123,\n",
            "  \"huge\": 1e999\n}\t"
        );
        let expected = concat!(
            " \n{\r\n  \"\\u540d\": \"A\\u3042\\ud83d\\ude00\",\n",
            "  \"list\": [true, null, {\"\\u540d\": \"B\"}],\n",
            "  \"n\": 90071992547409931234567890, \"n\": -0.00e+123,\n",
            "  \"huge\": 1e999\n}\t"
        );
        let encoded = calculate_unicode(
            input,
            EncodeOrDecodeSelect::Encode,
            InputFormatSelect::JsonDocument,
            EscapeSelect::NonAscii,
        )
        .unwrap();
        assert_eq!(encoded, expected);
        assert_eq!(
            calculate_unicode(
                &encoded,
                EncodeOrDecodeSelect::Decode,
                InputFormatSelect::JsonDocument,
                EscapeSelect::NonAscii,
            )
            .unwrap(),
            input
        );
    }

    #[test]
    fn test_encode_all_document_keys_and_values() {
        assert_eq!(
            calculate_unicode(
                r#"{"a":"A", "b":["あ", 42, true, null]}"#,
                EncodeOrDecodeSelect::Encode,
                InputFormatSelect::JsonDocument,
                EscapeSelect::All,
            )
            .unwrap(),
            r#"{"\u0061":"\u0041", "\u0062":["\u3042", 42, true, null]}"#
        );
    }

    #[test]
    fn test_document_string_scanning_handles_escaped_quotes_and_backslashes() {
        let input = r#"{"quote\"あ":"a\\\"😀", "literal":"\\u3042", "slash":"/"}"#;
        for escapes in [EscapeSelect::NonAscii, EscapeSelect::All] {
            let encoded = calculate_unicode(
                input,
                EncodeOrDecodeSelect::Encode,
                InputFormatSelect::JsonDocument,
                escapes,
            )
            .unwrap();
            let decoded = calculate_unicode(
                &encoded,
                EncodeOrDecodeSelect::Decode,
                InputFormatSelect::JsonDocument,
                escapes,
            )
            .unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[rstest]
    #[case("")]
    #[case("あ")]
    #[case("null")]
    #[case("42")]
    #[case("[]")]
    #[case("{}")]
    #[case(r#""a" "b""#)]
    fn test_json_string_requires_one_quoted_string(#[case] input: &str) {
        for operation in [EncodeOrDecodeSelect::Encode, EncodeOrDecodeSelect::Decode] {
            assert!(calculate_unicode(
                input,
                operation,
                InputFormatSelect::JsonString,
                EscapeSelect::NonAscii,
            )
            .is_err());
        }
    }

    #[rstest]
    #[case("")]
    #[case(r#"{"a":}"#)]
    #[case(r#"{"a":"\x41"}"#)]
    #[case(r#"{"\uD800":"value"}"#)]
    #[case(r#"{"a":"\uDC00"}"#)]
    #[case(r#"["\uD800\u0041"]"#)]
    #[case(r#"{"a":"unterminated}"#)]
    #[case("[1,]")]
    #[case("1 2")]
    #[case("NaN")]
    #[case("// comment\n{}")]
    fn test_json_document_rejects_invalid_input(#[case] input: &str) {
        for operation in [EncodeOrDecodeSelect::Encode, EncodeOrDecodeSelect::Decode] {
            assert!(calculate_unicode(
                input,
                operation,
                InputFormatSelect::JsonDocument,
                EscapeSelect::NonAscii,
            )
            .is_err());
        }
    }

    #[rstest]
    #[case("null")]
    #[case(" true ")]
    #[case("-0.00e+123")]
    #[case("[]")]
    #[case("{}")]
    fn test_json_document_accepts_non_string_values(#[case] input: &str) {
        for operation in [EncodeOrDecodeSelect::Encode, EncodeOrDecodeSelect::Decode] {
            assert_eq!(
                calculate_unicode(
                    input,
                    operation,
                    InputFormatSelect::JsonDocument,
                    EscapeSelect::All,
                )
                .unwrap(),
                input
            );
        }
    }

    #[test]
    fn test_text_round_trip() {
        let input: String = ('\0'..='\u{1f}')
            .chain(
                "Aあ😀\"\\/\u{7f}\u{80}\u{d7ff}\u{e000}\u{ffff}\u{10000}\u{10ffff}e\u{301}".chars(),
            )
            .collect();
        for escapes in [EscapeSelect::NonAscii, EscapeSelect::All] {
            let encoded = calculate_unicode(
                &input,
                EncodeOrDecodeSelect::Encode,
                InputFormatSelect::Text,
                escapes,
            )
            .unwrap();
            assert_eq!(
                calculate_unicode(
                    &encoded,
                    EncodeOrDecodeSelect::Decode,
                    InputFormatSelect::Text,
                    escapes,
                )
                .unwrap(),
                input
            );
        }
    }

    #[test]
    fn test_settings_recalculate_output_and_clear_errors() {
        let mut page = UnicodePage::new(true);
        page.cur.input = "Aあ".into();
        page.update_output();
        assert_eq!(page.cur.output, r"A\u3042");

        page.update(PageMsg::Unicode(UnicodeMsg::SelectNextItem));
        page.update(PageMsg::Unicode(UnicodeMsg::SelectNextItem));
        page.update(PageMsg::Unicode(UnicodeMsg::CurrentItemSelectNext));
        assert_eq!(page.cur.output, r"\u0041\u3042");

        page.update(PageMsg::Unicode(UnicodeMsg::SelectPrevItem));
        page.update(PageMsg::Unicode(UnicodeMsg::CurrentItemSelectNext));
        assert!(!page.cur.status.is_empty());
        assert!(page.cur.output.is_empty());

        page.update(PageMsg::Unicode(UnicodeMsg::CurrentItemSelectPrev));
        assert!(page.cur.status.is_empty());
        assert_eq!(page.cur.output, r"\u0041\u3042");

        page.cur.input = page.cur.output.clone();
        page.cur.item = PageItems::EncodeOrDecode;
        page.update(PageMsg::Unicode(UnicodeMsg::CurrentItemSelectNext));
        assert_eq!(page.cur.output, "Aあ");
    }

    #[test]
    fn test_decode_skips_encode_only_setting() {
        let mut page = UnicodePage::new(true);
        page.cur.eod_sel = EncodeOrDecodeSelect::Decode;
        page.cur.item = PageItems::InputFormat;
        page.select_next_item();
        assert!(page.cur.item == PageItems::Input);
        page.select_prev_item();
        assert!(page.cur.item == PageItems::InputFormat);

        page.cur.eod_sel = EncodeOrDecodeSelect::Encode;
        page.select_next_item();
        assert!(page.cur.item == PageItems::Escape);
    }

    #[test]
    fn test_render_at_different_sizes() {
        use ratatui::{backend::TestBackend, Terminal};

        let mut page = UnicodePage::new(true);
        page.cur.input = "Aあ😀".into();
        page.update_output();
        for (width, height) in [(80, 24), (40, 15), (26, 11), (20, 5), (0, 0)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            terminal
                .draw(|frame| page.render(frame, frame.area()))
                .unwrap();
        }
    }
}
