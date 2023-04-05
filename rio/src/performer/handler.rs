use crate::crosswords::pos::CharsetIndex;
use crate::crosswords::pos::Column;
use crate::crosswords::pos::StandardCharset;
use crate::crosswords::Mode;
use std::time::{Duration, Instant};

use crate::crosswords::attr::Attr;

use crate::performer::control::C0;
use colors::{AnsiColor, NamedColor};
use std::fmt::Write;

// https://vt100.net/emu/dec_ansi_parser
use vte::{Params, ParamsIter};

/// Maximum time before a synchronized update is aborted.
const SYNC_UPDATE_TIMEOUT: Duration = Duration::from_millis(150);

/// Number of bytes in the synchronized update DCS sequence before the passthrough parameters.
const SYNC_ESCAPE_START_LEN: usize = 5;

/// Start of the DCS sequence for beginning synchronized updates.
const SYNC_START_ESCAPE_START: [u8; SYNC_ESCAPE_START_LEN] =
    [b'\x1b', b'P', b'=', b'1', b's'];

/// Start of the DCS sequence for terminating synchronized updates.
const SYNC_END_ESCAPE_START: [u8; SYNC_ESCAPE_START_LEN] =
    [b'\x1b', b'P', b'=', b'2', b's'];

pub trait Handler {
    /// OSC to set window title.
    fn set_title(&mut self, _: Option<String>) {}

    /// Set the cursor style.
    // fn set_cursor_style(&mut self, _: Option<CursorStyle>) {}

    /// Set the cursor shape.
    // fn set_cursor_shape(&mut self, _shape: CursorShape) {}

    /// A character to be displayed.
    fn input(&mut self, _c: char) {}

    /// Set cursor to position.
    // fn goto(&mut self, _: Line, _: Column) {}

    /// Set cursor to specific row.
    // fn goto_line(&mut self, _: Line) {}

    /// Set cursor to specific column.
    // fn goto_col(&mut self, _: Column) {}

    /// Insert blank characters in current line starting from cursor.
    fn insert_blank(&mut self, _: usize) {}

    /// Move cursor up `rows`.
    fn move_up(&mut self, _: usize) {}

    /// Move cursor down `rows`.
    fn move_down(&mut self, _: usize) {}

    /// Identify the terminal (should write back to the pty stream).
    fn identify_terminal(&mut self, _intermediate: Option<char>) {}

    /// Report device status.
    fn device_status(&mut self, _: usize) {}

    /// Move cursor forward `cols`.
    // fn move_forward(&mut self, _: Column) {}

    /// Move cursor backward `cols`.
    // fn move_backward(&mut self, _: Column) {}

    /// Move cursor down `rows` and set to column 1.
    fn move_down_and_cr(&mut self, _: usize) {}

    /// Move cursor up `rows` and set to column 1.
    fn move_up_and_cr(&mut self, _: usize) {}

    /// Put `count` tabs.
    fn put_tab(&mut self, _count: u16) {}

    /// Backspace `count` characters.
    fn backspace(&mut self) {}

    /// Carriage return.
    fn carriage_return(&mut self) {}

    /// Linefeed.
    fn linefeed(&mut self) {}

    /// Ring the bell.
    ///
    /// Hopefully this is never implemented.
    fn bell(&mut self) {}

    /// Substitute char under cursor.
    fn substitute(&mut self) {}

    /// Newline.
    fn newline(&mut self) {}

    /// Set current position as a tabstop.
    fn set_horizontal_tabstop(&mut self) {}

    /// Scroll up `rows` rows.
    // fn scroll_up(&mut self, _: usize) {}

    /// Scroll down `rows` rows.
    fn scroll_down(&mut self, _: usize) {}

    /// Insert `count` blank lines.
    fn insert_blank_lines(&mut self, _: usize) {}

    /// Delete `count` lines.
    fn delete_lines(&mut self, _: usize) {}

    /// Erase `count` chars in current line following cursor.
    ///
    /// Erase means resetting to the default state (default colors, no content,
    /// no mode flags).
    fn erase_chars(&mut self, _: Column) {}

    /// Delete `count` chars.
    ///
    /// Deleting a character is like the delete key on the keyboard - everything
    /// to the right of the deleted things is shifted left.
    fn delete_chars(&mut self, _: usize) {}

    /// Move backward `count` tabs.
    fn move_backward_tabs(&mut self, _count: u16) {}

    /// Move forward `count` tabs.
    fn move_forward_tabs(&mut self, _count: u16) {}

    /// Save current cursor position.
    fn save_cursor_position(&mut self) {}

    /// Restore cursor position.
    fn restore_cursor_position(&mut self) {}

    /// Clear current line.
    fn clear_line(&mut self, _mode: u16) {}

    /// Clear screen.
    // fn clear_screen(&mut self, _mode: ClearMode) {}

    /// Clear tab stops.
    // fn clear_tabs(&mut self, _mode: TabulationClearMode) {}

    /// Reset terminal state.
    fn reset_state(&mut self) {}

    /// Reverse Index.
    ///
    /// Move the active position to the same horizontal position on the
    /// preceding line. If the active position is at the top margin, a scroll
    /// down is performed.
    fn reverse_index(&mut self) {}

    /// Set a terminal attribute.
    fn terminal_attribute(&mut self, _attr: Attr) {}

    /// Set mode.
    fn set_mode(&mut self, _mode: Mode) {}

    /// Unset mode.
    fn unset_mode(&mut self, _: Mode) {}

    /// DECSTBM - Set the terminal scrolling region.
    fn set_scrolling_region(&mut self, _top: usize, _bottom: Option<usize>) {}

    /// DECKPAM - Set keypad to applications mode (ESCape instead of digits).
    fn set_keypad_application_mode(&mut self) {}

    /// DECKPNM - Set keypad to numeric mode (digits instead of ESCape seq).
    fn unset_keypad_application_mode(&mut self) {}

    /// Set one of the graphic character sets, G0 to G3, as the active charset.
    ///
    /// 'Invoke' one of G0 to G3 in the GL area. Also referred to as shift in,
    /// shift out and locking shift depending on the set being activated.
    fn set_active_charset(&mut self, _: CharsetIndex) {}

    /// Assign a graphic character set to G0, G1, G2 or G3.
    ///
    /// 'Designate' a graphic character set as one of G0 to G3, so that it can
    /// later be 'invoked' by `set_active_charset`.
    fn configure_charset(&mut self, _: CharsetIndex, _: StandardCharset) {}

    /// Set an indexed color value.
    // fn set_color(&mut self, _: usize, _: Rgb) {}

    /// Respond to a color query escape sequence.
    fn dynamic_color_sequence(&mut self, _: String, _: usize, _: &str) {}

    /// Reset an indexed color to original value.
    fn reset_color(&mut self, _: usize) {}

    /// Store data into clipboard.
    fn clipboard_store(&mut self, _: u8, _: &[u8]) {}

    /// Load data from clipboard.
    fn clipboard_load(&mut self, _: u8, _: &str) {}

    /// Run the decaln routine.
    fn decaln(&mut self) {}

    /// Push a title onto the stack.
    fn push_title(&mut self) {}

    /// Pop the last title from the stack.
    fn pop_title(&mut self) {}

    /// Report text area size in pixels.
    fn text_area_size_pixels(&mut self) {}

    /// Report text area size in characters.
    fn text_area_size_chars(&mut self) {}
}

#[derive(Debug, Default)]
struct ProcessorState {
    /// Last processed character for repetition.
    preceding_char: Option<char>,

    /// DCS sequence waiting for termination.
    dcs: Option<Dcs>,

    /// State for synchronized terminal updates.
    sync_state: SyncState,
}

/// Maximum number of bytes read in one synchronized update (2MiB).
const SYNC_BUFFER_SIZE: usize = 0x20_0000;

#[derive(Debug)]
struct SyncState {
    /// Expiration time of the synchronized update.
    timeout: Option<Instant>,

    /// Sync DCS waiting for termination sequence.
    pending_dcs: Option<Dcs>,

    /// Bytes read during the synchronized update.
    buffer: Vec<u8>,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            buffer: Vec::with_capacity(SYNC_BUFFER_SIZE),
            pending_dcs: None,
            timeout: None,
        }
    }
}

/// Pending DCS sequence.
#[derive(Debug)]
enum Dcs {
    /// Begin of the synchronized update.
    SyncStart,

    /// End of the synchronized update.
    SyncEnd,
}

#[derive(Default)]
pub struct ParserProcessor {
    state: ProcessorState,
    parser: vte::Parser,
}

impl ParserProcessor {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a new byte from the PTY.
    #[inline]
    pub fn advance<H>(&mut self, handler: &mut H, byte: u8)
    where
        H: Handler,
    {
        if self.state.sync_state.timeout.is_none() {
            let mut performer = Performer::new(&mut self.state, handler);
            self.parser.advance(&mut performer, byte);
        } else {
            self.advance_sync(handler, byte);
        }
    }

    /// End a synchronized update.
    pub fn stop_sync<H>(&mut self, handler: &mut H)
    where
        H: Handler,
    {
        // Process all synchronized bytes.
        for i in 0..self.state.sync_state.buffer.len() {
            let byte = self.state.sync_state.buffer[i];
            let mut performer = Performer::new(&mut self.state, handler);
            self.parser.advance(&mut performer, byte);
        }

        // Resetting state after processing makes sure we don't interpret buffered sync escapes.
        self.state.sync_state.buffer.clear();
        self.state.sync_state.timeout = None;
    }

    /// Synchronized update expiration time.
    #[inline]
    pub fn sync_timeout(&self) -> Option<&Instant> {
        self.state.sync_state.timeout.as_ref()
    }

    /// Number of bytes in the synchronization buffer.
    #[inline]
    pub fn sync_bytes_count(&self) -> usize {
        self.state.sync_state.buffer.len()
    }

    /// Process a new byte during a synchronized update.
    #[cold]
    fn advance_sync<H>(&mut self, handler: &mut H, byte: u8)
    where
        H: Handler,
    {
        self.state.sync_state.buffer.push(byte);

        // Handle sync DCS escape sequences.
        match self.state.sync_state.pending_dcs {
            Some(_) => self.advance_sync_dcs_end(handler, byte),
            None => self.advance_sync_dcs_start(),
        }
    }

    /// Find the start of sync DCS sequences.
    fn advance_sync_dcs_start(&mut self) {
        // Get the last few bytes for comparison.
        let len = self.state.sync_state.buffer.len();
        let offset = len.saturating_sub(SYNC_ESCAPE_START_LEN);
        let end = &self.state.sync_state.buffer[offset..];

        // Check for extension/termination of the synchronized update.
        if end == SYNC_START_ESCAPE_START {
            self.state.sync_state.pending_dcs = Some(Dcs::SyncStart);
        } else if end == SYNC_END_ESCAPE_START || len >= SYNC_BUFFER_SIZE - 1 {
            self.state.sync_state.pending_dcs = Some(Dcs::SyncEnd);
        }
    }

    /// Parse the DCS termination sequence for synchronized updates.
    fn advance_sync_dcs_end<H>(&mut self, handler: &mut H, byte: u8)
    where
        H: Handler,
    {
        match byte {
            // Ignore DCS passthrough characters.
            0x00..=0x17 | 0x19 | 0x1c..=0x7f | 0xa0..=0xff => (),
            // Cancel the DCS sequence.
            0x18 | 0x1a | 0x80..=0x9f => self.state.sync_state.pending_dcs = None,
            // Dispatch on ESC.
            0x1b => match self.state.sync_state.pending_dcs.take() {
                Some(Dcs::SyncStart) => {
                    self.state.sync_state.timeout =
                        Some(Instant::now() + SYNC_UPDATE_TIMEOUT);
                }
                Some(Dcs::SyncEnd) => self.stop_sync(handler),
                None => (),
            },
        }
    }
}

struct Performer<'a, H: Handler> {
    state: &'a mut ProcessorState,
    handler: &'a mut H,
}

impl<'a, H: Handler + 'a> Performer<'a, H> {
    /// Create a performer.
    #[inline]
    pub fn new<'b>(
        state: &'b mut ProcessorState,
        handler: &'b mut H,
    ) -> Performer<'b, H> {
        Performer { state, handler }
    }
}

impl<U: Handler> vte::Perform for Performer<'_, U> {
    fn print(&mut self, c: char) {
        // println!("[print] {c:?}");
        self.handler.input(c);
        self.state.preceding_char = Some(c);
    }

    fn execute(&mut self, byte: u8) {
        println!("[execute] {byte:04x}");

        match byte {
            C0::HT => self.handler.put_tab(1),
            C0::BS => self.handler.backspace(),
            C0::CR => self.handler.carriage_return(),
            C0::LF | C0::VT | C0::FF => self.handler.linefeed(),
            C0::BEL => self.handler.bell(),
            C0::SUB => self.handler.substitute(),
            // C0::SI => self.handler.set_active_charset(CharsetIndex::G0),
            // C0::SO => self.handler.set_active_charset(CharsetIndex::G1),
            _ => println!("[unhandled] execute byte={byte:02x}"),
        }
    }

    fn hook(
        &mut self,
        params: &Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        if ignore || intermediates.len() > 1 {
            println!("unhandled");
            return;
        }

        let mut params_iter = params.iter();
        let handler = &mut self.handler;

        let mut next_param_or = |default: u16| match params_iter.next() {
            Some(&[param, ..]) if param != 0 => param,
            _ => default,
        };

        match (action, intermediates) {
            ('b', []) => {
                if let Some(c) = self.state.preceding_char {
                    for _ in 0..next_param_or(1) {
                        handler.input(c);
                    }
                } else {
                    println!("tried to repeat with no preceding char");
                }
            }
            ('s', [b'=']) => {
                // Start a synchronized update. The end is handled with a separate parser.
                if params.iter().next().map_or(false, |param| param[0] == 1) {
                    // self.state.dcs = Some(Dcs::SyncStart);
                }
            }
            _ => println!(
                "[unhandled hook] params={:?}, ints: {:?}, ignore: {:?}, action: {:?}",
                params, intermediates, ignore, action
            ),
        }
        // println!(
        //     "[hook] params={params:?}, intermediates={intermediates:?}, ignore={ignore:?}, char={c:?}"
        // );
    }

    fn put(&mut self, _byte: u8) {
        // println!("[put] {byte:02x}");
    }

    #[inline]
    fn unhook(&mut self) {
        match self.state.dcs {
            Some(Dcs::SyncStart) => {
                self.state.sync_state.timeout =
                    Some(Instant::now() + SYNC_UPDATE_TIMEOUT);
            }
            Some(Dcs::SyncEnd) => (),
            _ => println!("[unhandled unhook]"),
        }
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        println!("[osc_dispatch] params={params:?} bell_terminated={bell_terminated}");

        let _terminator = if bell_terminated { "\x07" } else { "\x1b\\" };

        fn unhandled(params: &[&[u8]]) {
            let mut buf = String::new();
            for items in params {
                buf.push('[');
                for item in *items {
                    let _ = write!(buf, "{:?}", *item as char);
                }
                buf.push_str("],");
            }
            println!("[unhandled osc_dispatch]: [{}] at line {}", &buf, line!());
        }

        if params.is_empty() || params[0].is_empty() {
            return;
        }

        match params[0] {
            // Set window title.
            b"0" | b"2" => {
                if params.len() >= 2 {
                    let title = params[1..]
                        .iter()
                        .flat_map(|x| std::str::from_utf8(x))
                        .collect::<Vec<&str>>()
                        .join(";")
                        .trim()
                        .to_owned();
                    self.handler.set_title(Some(title));
                    return;
                }
                unhandled(params);
            }

            // Set color index.
            b"4" => {
                if params.len() <= 1 || params.len() % 2 == 0 {
                    unhandled(params);
                    // return;
                }

                // for chunk in params[1..].chunks(2) {
                // let index = match parse_number(chunk[0]) {
                //     Some(index) => index,
                //     None => {
                //         unhandled(params);
                //         continue;
                //     },
                // };

                // if let Some(c) = xparse_color(chunk[1]) {
                //     self.handler.set_color(index as usize, c);
                // } else if chunk[1] == b"?" {
                //     let prefix = format!("4;{index}");
                //     self.handler.dynamic_color_sequence(prefix, index as usize, terminator);
                // } else {
                //     unhandled(params);
                // }
                // }
            }

            b"10" | b"11" | b"12" => {
                if params.len() >= 2 {
                    // if let Some(mut dynamic_code) = parse_number(params[0]) {
                    //     for param in &params[1..] {
                    //         // 10 is the first dynamic color, also the foreground.
                    //         let offset = dynamic_code as usize - 10;
                    //         let index = NamedColor::Foreground as usize + offset;

                    //         // End of setting dynamic colors.
                    //         if index > NamedColor::Cursor as usize {
                    //             unhandled(params);
                    //             break;
                    //         }

                    //         if let Some(color) = xparse_color(param) {
                    //             self.handler.set_color(index, color);
                    //         } else if param == b"?" {
                    //             self.handler.dynamic_color_sequence(
                    //                 dynamic_code.to_string(),
                    //                 index,
                    //                 terminator,
                    //             );
                    //         } else {
                    //             unhandled(params);
                    //         }
                    //         dynamic_code += 1;
                    //     }
                    //     return;
                    // }
                }
                unhandled(params);
            }

            b"110" => {}

            b"111" => {}

            b"112" => {}

            _ => unhandled(params),
        }
    }

    // Control Sequence Introducer
    // CSI is the two-character sequence ESCape left-bracket or the 8-bit
    // C1 code of 233 octal, 9B hex.  CSI introduces a Control Sequence, which
    // continues until an alphabetic character is received.
    fn csi_dispatch(
        &mut self,
        params: &Params,
        intermediates: &[u8],
        should_ignore: bool,
        action: char,
    ) {
        macro_rules! csi_unhandled {
            () => {{
                println!(
                    "[csi_dispatch] params={params:#?}, intermediates={intermediates:?}, should_ignore={should_ignore:?}, action={action:?}"
                );
            }};
        }

        if should_ignore || intermediates.len() > 1 {
            return;
        }

        let mut params_iter = params.iter();
        let handler = &mut self.handler;

        let mut next_param_or = |default: u16| match params_iter.next() {
            Some(&[param, ..]) if param != 0 => param,
            _ => default,
        };

        match (action, intermediates) {
            ('K', []) => handler.clear_line(next_param_or(0)),
            ('J', []) => {}
            ('t', []) => match next_param_or(1) as usize {
                14 => handler.text_area_size_pixels(),
                18 => handler.text_area_size_chars(),
                // 22 => handler.push_title(),
                // 23 => handler.pop_title(),
                _ => println!("aaa"),
            },

            ('m', []) => {
                if params.is_empty() {
                    handler.terminal_attribute(Attr::Reset);
                } else {
                    for attr in attrs_from_sgr_parameters(&mut params_iter) {
                        match attr {
                            Some(attr) => handler.terminal_attribute(attr),
                            None => csi_unhandled!(),
                        }
                    }
                }
            }
            _ => {}
        };
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        println!(
            "[esc_dispatch] intermediates={intermediates:?}, ignore={ignore:?}, byte={byte:02x}"
        );
    }
}

#[inline]
fn attrs_from_sgr_parameters(params: &mut ParamsIter<'_>) -> Vec<Option<Attr>> {
    let mut attrs = Vec::with_capacity(params.size_hint().0);

    #[allow(clippy::while_let_on_iterator)]
    while let Some(param) = params.next() {
        let attr = match param {
            [0] => Some(Attr::Reset),
            [1] => Some(Attr::Bold),
            [2] => Some(Attr::Dim),
            [3] => Some(Attr::Italic),
            [4, 0] => Some(Attr::CancelUnderline),
            [4, 2] => Some(Attr::DoubleUnderline),
            [4, 3] => Some(Attr::Undercurl),
            [4, 4] => Some(Attr::DottedUnderline),
            [4, 5] => Some(Attr::DashedUnderline),
            [4, ..] => Some(Attr::Underline),
            [5] => Some(Attr::BlinkSlow),
            [6] => Some(Attr::BlinkFast),
            [7] => Some(Attr::Reverse),
            [8] => Some(Attr::Hidden),
            [9] => Some(Attr::Strike),
            [21] => Some(Attr::CancelBold),
            [22] => Some(Attr::CancelBoldDim),
            [23] => Some(Attr::CancelItalic),
            [24] => Some(Attr::CancelUnderline),
            [25] => Some(Attr::CancelBlink),
            [27] => Some(Attr::CancelReverse),
            [28] => Some(Attr::CancelHidden),
            [29] => Some(Attr::CancelStrike),
            [30] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Black))),
            [31] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Red))),
            [32] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Green))),
            [33] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Yellow))),
            [34] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Blue))),
            [35] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Magenta))),
            [36] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Cyan))),
            [37] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::White))),
            // [38] => {
            //     // let mut iter = params.map(|param| param[0]);
            //     // parse_sgr_color(&mut iter).map(Attr::Foreground)
            // }
            // [38, params @ ..] => {
            //     // handle_colon_rgb(params).map(Attr::Foreground)
            // }
            [39] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::Foreground))),
            [40] => Some(Attr::Background(AnsiColor::Named(NamedColor::Black))),
            [41] => Some(Attr::Background(AnsiColor::Named(NamedColor::Red))),
            [42] => Some(Attr::Background(AnsiColor::Named(NamedColor::Green))),
            [43] => Some(Attr::Background(AnsiColor::Named(NamedColor::Yellow))),
            [44] => Some(Attr::Background(AnsiColor::Named(NamedColor::Blue))),
            [45] => Some(Attr::Background(AnsiColor::Named(NamedColor::Magenta))),
            [46] => Some(Attr::Background(AnsiColor::Named(NamedColor::Cyan))),
            [47] => Some(Attr::Background(AnsiColor::Named(NamedColor::White))),
            // [48] => {
            //     let mut iter = params.map(|param| param[0]);
            //     parse_sgr_color(&mut iter).map(Attr::Background)
            // },
            // [48, params @ ..] => handle_colon_rgb(params).map(Attr::Background),
            [49] => Some(Attr::Background(AnsiColor::Named(NamedColor::Background))),
            // [58] => {
            //     let mut iter = params.map(|param| param[0]);
            //     parse_sgr_color(&mut iter).map(|color| Attr::UnderlineColor(Some(color)))
            // },
            // [58, params @ ..] => {
            //     handle_colon_rgb(params).map(|color| Attr::UnderlineColor(Some(color)))
            // },
            [59] => Some(Attr::UnderlineColor(None)),
            [90] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightBlack))),
            [91] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightRed))),
            [92] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightGreen))),
            [93] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightYellow))),
            [94] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightBlue))),
            [95] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightMagenta))),
            [96] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightCyan))),
            [97] => Some(Attr::Foreground(AnsiColor::Named(NamedColor::LightWhite))),
            [100] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightBlack))),
            [101] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightRed))),
            [102] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightGreen))),
            [103] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightYellow))),
            [104] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightBlue))),
            [105] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightMagenta))),
            [106] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightCyan))),
            [107] => Some(Attr::Background(AnsiColor::Named(NamedColor::LightWhite))),
            _ => None,
        };
        attrs.push(attr);
    }

    attrs
}