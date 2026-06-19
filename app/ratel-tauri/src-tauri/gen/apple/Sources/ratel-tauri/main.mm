#include "bindings/bindings.h"

#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>

#import "RatelPush.h"

int main(int argc, char * argv[]) {
	// `ffi::start_app()` runs UIApplicationMain, which creates Tauri's own
	// AppDelegate — we don't control it, so register a one-shot observer for
	// launch completion BEFORE it starts. The block runs on the main queue
	// right after the delegate's didFinishLaunching, when [FIRApp configure]
	// can safely swizzle the delegate for APNs token forwarding.
	@autoreleasepool {
		[[NSNotificationCenter defaultCenter]
			addObserverForName:UIApplicationDidFinishLaunchingNotification
			            object:nil
			             queue:[NSOperationQueue mainQueue]
			        usingBlock:^(NSNotification * _Nonnull note) {
				[RatelPush start];
			}];
	}
	ffi::start_app();
	return 0;
}
