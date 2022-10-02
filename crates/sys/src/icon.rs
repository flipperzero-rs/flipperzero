//! Low-level bindings to the Icon API.

use crate::opaque;

opaque!(Icon);

extern "C" {
    #[link_name = "icon_get_width"]
    pub fn get_width(icon: *const Icon) -> u8;
    #[link_name = "icon_get_height"]
    pub fn get_height(icon: *const Icon) -> u8;
}

pub mod icons {
    use super::Icon;

    extern "C" {
        #[link_name = "I_Smile_18x18"]
        pub static SMILE: Icon;
    }

    pub mod buttons {
        use super::Icon;

        extern "C" {
            #[link_name = "I_ButtonCenter_7x7"]
            pub static CENTER: Icon;
            #[link_name = "I_back_10px"]
            pub static BACK: Icon;

            #[link_name = "I_ButtonUp_7x4"]
            pub static UP: Icon;
            #[link_name = "I_ButtonDown_7x4"]
            pub static DOWN: Icon;

            #[link_name = "I_ButtonLeft_4x7"]
            pub static LEFT: Icon;
            #[link_name = "I_ButtonRight_4x7"]
            pub static RIGHT: Icon;
        }
    }

    pub mod pin {
        use super::Icon;

        extern "C" {
            #[link_name = "I_Pin_back_arrow_10x8"]
            pub static BACK: Icon;

            #[link_name = "I_Pin_arrow_up_7x9"]
            pub static UP: Icon;
            #[link_name = "I_Pin_arrow_down_7x9"]
            pub static DOWN: Icon;
            #[link_name = "I_Pin_arrow_left_9x7"]
            pub static LEFT: Icon;
            #[link_name = "I_Pin_arrow_right_9x7"]
            pub static RIGHT: Icon;
        }
    }

    pub mod down {
        use super::Icon;

        extern "C" {
            #[link_name = "I_ArrowDownEmpty_14x15"]
            pub static OUTLINE: Icon;
            #[link_name = "I_ArrowDownFilled_14x15"]
            pub static FILLED: Icon;
        }
    }

    pub mod up {
        use super::Icon;

        extern "C" {
            #[link_name = "I_ArrowUpEmpty_14x15"]
            pub static OUTLINE: Icon;
            #[link_name = "I_ArrowUpFilled_14x15"]
            pub static FILLED: Icon;
        }
    }
}
