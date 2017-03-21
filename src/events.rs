extern crate sdl2;

use self::sdl2::EventPump;

macro_rules! define_events {
    (
        keyboard: { $( $k_alias:ident : $k_sdl:ident , )* },
        raw: { $( $r_alias:ident : $r_sdl:ident , )* },
    ) => {

        pub struct EventStatus {
            $( pub $k_alias: bool , )*
            $( pub $r_alias: bool , )*
        }

        impl EventStatus {
            pub fn new() -> EventStatus {
                EventStatus {
                    $( $k_alias: false , )*
                    $( $r_alias: false , )*
                }
            }

            pub fn pump(&mut self, pump: &mut EventPump, clear: bool) {
                if clear {
                    $( self.$k_alias = false; )*
                }

                for event in pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        $( $r_sdl { .. } => self.$r_alias = true , )*

                        KeyDown { keycode, .. } => match keycode {
                            $( Some($k_sdl) => self.$k_alias = true , )*
                            _ => {},
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $( Some($k_sdl) => self.$k_alias = false , )*
                            _ => {},
                        },

                        _ => {},
                    }
                }
            }
        }

    }
}

define_events! {
    keyboard: {
        key_left: Left,
        key_right: Right,
        key_up: Up,
        key_down: Down,
        key_escape: Escape,
        key_space: Space,
        key_left_bracket: LeftBracket,
        key_right_bracket: RightBracket,
        key_backslash: Backslash,
        key_return: Return,
    },
    raw: {
        quit: Quit,
    },
}
