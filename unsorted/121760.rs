mod framing {
    mod public_message_in {
        mod public_message {

            pub struct ConfirmedTranscriptHashInput;
        }

        mod public_message_in {

            #[derive(Debug)]
            pub struct ConfirmedTranscriptHashInput;
        }

        pub use self::public_message::*;
        pub use self::public_message_in::*;
    }

    pub use self::public_message_in::*;
}

use crate::framing::ConfirmedTranscriptHashInput::public_message_in;

fn main() {}
