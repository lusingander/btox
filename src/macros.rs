#[macro_export]
macro_rules! key_code {
    ( $code:path ) => {
        ratatui::crossterm::event::KeyEvent { code: $code, .. }
    };
}

#[macro_export]
macro_rules! key_code_char {
    ( $c:ident ) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($c),
            ..
        }
    };
    ( $c:expr ) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($c),
            ..
        }
    };
    ( $c:expr, Ctrl ) => {
        ratatui::crossterm::event::KeyEvent {
            code: ratatui::crossterm::event::KeyCode::Char($c),
            modifiers: ratatui::crossterm::event::KeyModifiers::CONTROL,
            ..
        }
    };
}

#[macro_export]
macro_rules! fn_str_map {
    ( $( $item:pat => $str:expr ),+ $(,)? ) => {
        fn str(&self) -> &str {
            match self {
                $( $item => $str ),+
            }
        }
        #[allow(unused)]
        fn strings_vec() -> Vec<String> {
            Self::vars_vec().iter().map(|s| s.str().into()).collect()
        }
    };
}

#[macro_export]
macro_rules! fn_next_prev_mut {
    () => {
        fn next_mut(&mut self) {
            if self.val() < Self::len() - 1 {
                *self = self.next();
            }
        }
        fn prev_mut(&mut self) {
            if self.val() > 0 {
                *self = self.prev();
            }
        }
    };
}
