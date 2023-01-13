use crate::app::App;
use crate::types::{Command, Error, Request, Response, Message};
use interchange::Responder;

pub struct Dispatch<'a> {
    responder: Responder<'a, Request, Response>,
}

impl<'a> Dispatch<'a> {
    pub fn new(responder: Responder<'a, Request, Response>) -> Dispatch {
        Dispatch { responder }
    }

    fn find_app<'b, 'c>(
        command: Command,
        apps: &'b mut [&'c mut dyn App],
    ) -> Option<&'b mut &'c mut dyn App> {
        apps.iter_mut()
            .find(|app| app.commands().contains(&command))
    }

    // // Using helper here to take potentially large stack burden off of call chain to application.
    // #[inline(never)]
    // fn reply_with_request_buffer(&mut self){
    //     let (_command, message) = self.responder.take_request().unwrap();
    //     let message = message.clone();
    //     self.responder.respond(&Ok(message)).expect("responder failed");
    // }

    // Using helper here to take potentially large stack burden off of call chain to application.
    #[inline(never)]
    fn reply_with_error(&mut self, error: Error) {
        self.responder.respond(Err(error)).expect("cant respond");
    }

    #[inline(never)]
    fn call_app(&mut self, app: &mut dyn App, command: Command, request: &Message) {
        let mut response_buffer = Message::new();
        if let Err(error) = app.call(command, request, &mut response_buffer) {
            self.reply_with_error(error)
        } else {
            let response = Ok(response_buffer);
            self.responder.respond(response).expect("responder failed");
        }
    }

    #[inline(never)]
    pub fn poll(&mut self, apps: &mut [&mut dyn App]) -> bool {
        let maybe_request = self.responder.take_request();
        if let Some((command, message)) = maybe_request {
            // info_now!("cmd: {}", u8::from(command));
            // info_now!("cmd: {:?}", command);

            if let Some(app) = Self::find_app(command, apps) {
                // match app.call(command, self.responder.response_mut().unwrap()) {
                self.call_app(*app, command, &message);
            } else {
                self.reply_with_error(Error::InvalidCommand);
            }
        }

        self.responder.state() == interchange::State::Responded
    }
}
