//! Low-level bindings to Notificcation API.

use core::ffi::c_char;
use crate::{opaque, c_string};

pub const RECORD_NOTIFICATION: *const c_char = c_string!("notification");

opaque!(Notification);
opaque!(NotificationSequence);

extern "C" {
    #[link_name="notification_message"]
    pub fn message(notifications_app: *mut Notification, sequence: *const NotificationSequence);
}

pub mod vibro {
    use super::NotificationSequence;

    extern {
        #[link_name="sequence_set_vibro_on"]
        pub static ON: NotificationSequence;
        #[link_name="sequence_reset_vibro"]
        pub static OFF: NotificationSequence;
    }
}

pub mod led {
    pub mod red {
        use super::super::*;

        extern {
            #[link_name="sequence_blink_red_10"]
            pub static BLINK_10: NotificationSequence;
            #[link_name="sequence_blink_red_100"]
            pub static BLINK_100: NotificationSequence;
        }
    }

    pub mod green {
        use super::super::*;

        extern {
            #[link_name="sequence_blink_green_10"]
            pub static BLINK_10: NotificationSequence;
            #[link_name="sequence_blink_green_100"]
            pub static BLINK_100: NotificationSequence;
        }
    }

    pub mod blue {
        use super::super::*;

        extern {
            #[link_name="sequence_blink_blue_10"]
            pub static BLINK_10: NotificationSequence;
            #[link_name="sequence_blink_blue_100"]
            pub static BLINK_100: NotificationSequence;
        }
    }

    pub mod yellow {
        use super::super::*;

        extern {
            #[link_name="sequence_blink_yellow_10"]
            pub static BLINK_10: NotificationSequence;
            #[link_name="sequence_blink_yellow_100"]
            pub static BLINK_100: NotificationSequence;
        }
    }

    pub mod cyan {
        use super::super::*;

        extern {
            #[link_name="sequence_blink_cyan_10"]
            pub static BLINK_10: NotificationSequence;
            #[link_name="sequence_blink_cyan_100"]
            pub static BLINK_100: NotificationSequence;
        }
    }

    pub mod magenta {
        use super::super::*;

        extern {
            #[link_name="sequence_blink_magenta_10"]
            pub static BLINK_10: NotificationSequence;
            #[link_name="sequence_blink_magenta_100"]
            pub static BLINK_100: NotificationSequence;
        }
    }
}