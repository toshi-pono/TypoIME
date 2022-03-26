use cocoa::appkit::{NSApp, NSApplication};
use cocoa::base::{id, nil, BOOL};
use cocoa::foundation::{NSAutoreleasePool, NSString};

#[macro_use]
extern crate objc;

mod imk;

fn main() {
    imk::register_controller();

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        let k_connection_name = NSString::alloc(nil).init_str("toshi00TypoIME_1_Connection");
        let nib_name = NSString::alloc(nil).init_str("MainMenu");

        // TODO: cocoa の NSbundle に置き換えたいかも
        let bundle: id = msg_send![class!(NSBundle), mainBundle];
        let identifer: id = msg_send![bundle, bundleIdentifier];

        imk::describe(identifer);
        imk::describe(nib_name);
        imk::describe(k_connection_name);

        // NOTE: OBJC 風の書き方を調べる
        imk::connect_imkserver(k_connection_name, identifer);

        let _: BOOL = msg_send![class!(NSBundle), loadNibNamed:nib_name
                                owner:app];
        app.run()
    }
}
