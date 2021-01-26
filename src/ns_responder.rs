// /*
//         NSResponder.h
//         Application Kit
//         Copyright (c) 1994-2019, Apple Inc.
//         All rights reserved.
// */
// #import <Foundation/NSObject.h>
// #import <Foundation/NSArray.h>
// #import <AppKit/NSEvent.h>
// #import <AppKit/NSAccessibilityProtocols.h>
// #import <AppKit/NSPasteboard.h>
// #import <AppKit/AppKitDefines.h>

// NS_ASSUME_NONNULL_BEGIN
// APPKIT_API_UNAVAILABLE_BEGIN_MACCATALYST

// @class NSError, NSEvent, NSMenu, NSUndoManager, NSWindow;

// @interface NSResponder : NSObject <NSCoding>
use crate::prelude::*;

pub enum NSResponderFFI {}

foreign_obj_type! {
    type CType = NSResponderFFI;
    pub struct NSResponder;
    pub struct NSResponderRef;
}

impl NSResponder {
    // - (instancetype)init NS_DESIGNATED_INITIALIZER;
    // - (nullable instancetype)initWithCoder:(NSCoder *)coder NS_DESIGNATED_INITIALIZER;
}

impl NSResponderRef {
    // @property (nullable, assign) NSResponder *nextResponder;
    pub fn next_responder(&self) -> Option<&NSResponderRef> {
        todo!()
    }

    pub fn set_next_responder(&self, responder: &NSResponderRef) {
        todo!()
    }

    // - (BOOL)tryToPerform:(SEL)action with:(nullable id)object;

    // - (BOOL)performKeyEquivalent:(NSEvent *)event;
    pub fn perform_key_equivalent(&self, event: &NSEventRef) -> bool {
        unsafe { msg_send![self, performKeyEquivalent: event] }
    }
    // - (nullable id)validRequestorForSendType:(nullable NSPasteboardType)sendType returnType:(nullable NSPasteboardType)returnType;
    // - (void)mouseDown:(NSEvent *)event;
    pub fn mouse_down(&self, event: &NSEventRef) {
        unsafe { msg_send![self, mouseDown: event] }
    }

    // - (void)mouseUp:(NSEvent *)event;
    pub fn mouse_up(&self, event: &NSEventRef) {
        unsafe { msg_send![self, mouseUp: event] }
    }

    // - (void)rightMouseUp:(NSEvent *)event;
    pub fn right_mouse_up(&self, event: &NSEventRef) {
        unsafe { msg_send![self, rightMouseUp: event] }
    }

    // - (void)otherMouseUp:(NSEvent *)event;
    pub fn other_mouse_up(&self, event: &NSEventRef) {
        unsafe { msg_send![self, otherMouseUp: event] }
    }

    // - (void)mouseMoved:(NSEvent *)event;
    pub fn mouse_moved(&self, event: &NSEventRef) {
        unsafe { msg_send![self, mouseMoved: event] }
    }

    // - (void)mouseDragged:(NSEvent *)event;
    pub fn mouse_dragged(&self, event: &NSEventRef) {
        unsafe { msg_send![self, mouseDragged: event] }
    }

    // - (void)scrollWheel:(NSEvent *)event;
    pub fn scroll_wheel(&self, event: &NSEventRef) {
        unsafe { msg_send![self, scrollWheel: event] }
    }

    // - (void)rightMouseDragged:(NSEvent *)event;
    pub fn right_mouse_dragged(&self, event: &NSEventRef) {
        unsafe { msg_send![self, rightMouseDragged: event] }
    }

    // - (void)otherMouseDragged:(NSEvent *)event;
    pub fn other_mouse_dragged(&self, event: &NSEventRef) {
        unsafe { msg_send![self, otherMouseDragged: event] }
    }

    // - (void)mouseEntered:(NSEvent *)event;
    pub fn mouse_entered(&self, event: &NSEventRef) {
        unsafe { msg_send![self, mouseEntered: event] }
    }

    // - (void)mouseExited:(NSEvent *)event;
    pub fn mouse_exited(&self, event: &NSEventRef) {
        unsafe { msg_send![self, mouseExited: event] }
    }

    // - (void)keyDown:(NSEvent *)event;
    pub fn key_down(&self, event: &NSEventRef) {
        unsafe { msg_send![self, keyDown: event] }
    }

    // - (void)keyUp:(NSEvent *)event;
    pub fn key_up(&self, event: &NSEventRef) {
        unsafe { msg_send![self, keyUp: event] }
    }

    // - (void)flagsChanged:(NSEvent *)event;
    pub fn flags_changed(&self, event: &NSEventRef) {
        unsafe { msg_send![self, flagsChanged: event] }
    }

    // - (void)tabletPoint:(NSEvent *)event;
    pub fn tablet_point(&self, event: &NSEventRef) {
        unsafe { msg_send![self, tabletPoint: event] }
    }

    // - (void)tabletProximity:(NSEvent *)event;
    pub fn tablet_proximity(&self, event: &NSEventRef) {
        unsafe { msg_send![self, tabletProximity: event] }
    }

    // - (void)cursorUpdate:(NSEvent *)event API_AVAILABLE(macos(10.5));
    pub fn cursor_update(&self, event: &NSEventRef) {
        unsafe { msg_send![self, cursorUpdate: event] }
    }

    // /* The following *WithEvent methods are available on 10.5.2 or later, and will be sent only on hardware capable of generating the corresponding NSEvent types
    // */
    // - (void)magnifyWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.5));
    pub fn magnify_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, magnifyWithEvent: event] }
    }
    // - (void)rotateWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.5));
    pub fn rotate_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, rotateWithEvent: event] }
    }
    // - (void)swipeWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.5));
    pub fn swipe_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, swipeWithEvent: event] }
    }
    // - (void)beginGestureWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.5));
    pub fn begin_gesture_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, beginGestureWithEvent: event] }
    }
    // - (void)endGestureWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.5));
    pub fn end_gesture_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, endGestureWithEvent: event] }
    }

    // - (void)smartMagnifyWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.8));
    pub fn smart_magnify_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, smartMagnifyWithEvent: event] }
    }

    // /* Issued in response to a double-tap on the side of the Apple Pencil */
    // - (void)changeModeWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.15));
    pub fn change_mode_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, changeModeWithEvent: event] }
    }

    // /* A new set of touches has been recognized. To get the set of touches that began for this view (or descendants of this view): [event touchesMatchingPhase:NSTouchPhaseBegan inView:self]; Note: this is not always the point of contact with the touch device. A touch that transitions from resting to active may be part of a Began set.
    // */
    // - (void)touchesBeganWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.6));
    pub fn touches_began_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, touchesBeganWithEvent: event] }
    }

    // /* One or more touches has moved. To get the set of touches that moved for this view (or descendants of this view): [event touchesMatchingPhase:NSTouchPhaseMoved inView:self];
    // */
    // - (void)touchesMovedWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.6));
    pub fn touches_moved_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, touchesMovedWithEvent: event] }
    }

    // /* A set of touches have been removed. To get the set of touches that ended for this view (or descendants of this view): [event touchesMatchingPhase:NSTouchPhaseEnded inView:self]; Note: this is not always the point of removal with the touch device. A touch that transitions from active to resting may be part of an Ended set.
    // */
    // - (void)touchesEndedWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.6));
    pub fn touches_ended_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, touchesEndedWithEvent: event] }
    }

    // /* The System has cancelled the tracking of touches for any reason.
    // */
    // - (void)touchesCancelledWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.6));
    pub fn touches_cancelled_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, touchesCancelledWithEvent: event] }
    }

    // /* Perform a Quick Look on the content at location in the event. If there are no Quick Look items at the location, call super. Also, see quickLookPreviewItems: further below. */
    // - (void)quickLookWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.8));
    pub fn quick_look_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, quickLookWithEvent: event] }
    }

    // - (void)pressureChangeWithEvent:(NSEvent *)event API_AVAILABLE(macos(10.10.3));
    pub fn pressure_change_with_event(&self, event: &NSEventRef) {
        unsafe { msg_send![self, pressureChangeWithEvent: event] }
    }

    // - (void)noResponderFor:(SEL)eventSelector;
    // @property (readonly) BOOL acceptsFirstResponder;
    // - (BOOL)becomeFirstResponder;
    // - (BOOL)resignFirstResponder;

    // - (void)interpretKeyEvents:(NSArray<NSEvent *> *)eventArray;
    // - (void)flushBufferedKeyEvents;

    // @property (nullable, strong) NSMenu *menu;

    // - (void)showContextHelp:(nullable id)sender;

    // - (void)helpRequested:(NSEvent *)eventPtr;

    // - (BOOL)shouldBeTreatedAsInkEvent:(NSEvent *)event;

    // /* Some views process gesture scroll events to perform elastic scrolling. In some cases, you may want to track gesture scroll events like a swipe. (see -trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler: in NSEvent.h) Implement this method and return YES in your swipe controller and views that perform elastic scrolling will forward gesture scroll events up the responder chain on the following condition: the content to be scrolled is already at the edge of the scrolled direction at the beginning of the scroll gesture. Otherwise, the view will perform elastic scrolling. Default implementation returns NO.
    // */
    // - (BOOL)wantsScrollEventsForSwipeTrackingOnAxis:(NSEventGestureAxis)axis API_AVAILABLE(macos(10.7));
    pub fn wants_scroll_events_for_swipe_tracking_on_axis(&self, event: &NSEventRef) {
        unsafe { msg_send![self, wantsScrollEventsForSwipeTrackingOnAxis: event] }
    }

    // /* Some views process gesture scroll events to perform elastic scrolling. In some cases, the scroll events should be forwarded up the responder chain for further processing (for example an enclosing scroll view). Implement this method returning yes and views that perform elastic scrolling will forward gesture scroll events up the responder chain on the following condition: the content to be scrolled is already at the edge of the scrolled direction at the beginning of the scroll gesture. Otherwise, the view will perform elastic scrolling. Note: If you intend to call -trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler: implement -wantsToTrackScrollEventsAsSwipeForAxis, as defined above instead. Default implementation returns NO.
    // */
    // - (BOOL)wantsForwardedScrollEventsForAxis:(NSEventGestureAxis)axis API_AVAILABLE(macos(10.7));
    pub fn wants_forwarded_scroll_events_for_axis(&self, event: &NSEventRef) {
        unsafe { msg_send![self, wantsForwardedScrollEventsForAxis: event] }
    }

    // /* This method is used in the process of finding a target for an action method. If this NSResponder instance does not itself respondsToSelector:action, then supplementalTargetForAction:sender: is called. This method should return an object which responds to the action; if this responder does not have a supplemental object that does that, the implementation of this method should call super's supplementalTargetForAction:sender:. NSResponder's implementation returns nil.
    // */
    // - (nullable id)supplementalTargetForAction:(SEL)action sender:(nullable id)sender API_AVAILABLE(macos(10.7));

    // @end

    // /* This protocol contains a large number of methods intended for use as key binding commands.  NSResponder does not implement any of them.  NSTextView implements a certain subset of them (see the NSTextView.h header).  Your responder subclasses can implement any that make sense.  You can make up your own as well, but you should use these if the concepts map.  If your view is key and uses key binding and the user types a key sequence which is bound to a command which is not implemented in your class, it is OK, nothing will happen by default.
    // */
    // @protocol NSStandardKeyBindingResponding <NSObject>
    // @optional

    // */*********************** Key binding entry-points *************************/
    // - (void)insertText:(id)insertString;
    //     // When key events have been passed off to the key binding mechanism through interpretKeyEvents:, they end up back in the view through either this method or the below doCommand... methods.  insertText: is used to pass through text that was not a command.

    // - (void)doCommandBySelector:(SEL)selector;
    //     // Performs the given selector if possible.

    // */*********************** Standard bindable commands *************************/
    //     /* Selection movement and scrolling */
    // - (void)moveForward:(nullable id)sender;
    // - (void)moveRight:(nullable id)sender;
    // - (void)moveBackward:(nullable id)sender;
    // - (void)moveLeft:(nullable id)sender;
    // - (void)moveUp:(nullable id)sender;
    // - (void)moveDown:(nullable id)sender;
    // - (void)moveWordForward:(nullable id)sender;
    // - (void)moveWordBackward:(nullable id)sender;
    // - (void)moveToBeginningOfLine:(nullable id)sender;
    // - (void)moveToEndOfLine:(nullable id)sender;
    // - (void)moveToBeginningOfParagraph:(nullable id)sender;
    // - (void)moveToEndOfParagraph:(nullable id)sender;
    // - (void)moveToEndOfDocument:(nullable id)sender;
    // - (void)moveToBeginningOfDocument:(nullable id)sender;
    // - (void)pageDown:(nullable id)sender;
    // - (void)pageUp:(nullable id)sender;
    // - (void)centerSelectionInVisibleArea:(nullable id)sender;

    // - (void)moveBackwardAndModifySelection:(nullable id)sender;
    // - (void)moveForwardAndModifySelection:(nullable id)sender;
    // - (void)moveWordForwardAndModifySelection:(nullable id)sender;
    // - (void)moveWordBackwardAndModifySelection:(nullable id)sender;
    // - (void)moveUpAndModifySelection:(nullable id)sender;
    // - (void)moveDownAndModifySelection:(nullable id)sender;

    // - (void)moveToBeginningOfLineAndModifySelection:(nullable id)sender;
    // - (void)moveToEndOfLineAndModifySelection:(nullable id)sender;
    // - (void)moveToBeginningOfParagraphAndModifySelection:(nullable id)sender;
    // - (void)moveToEndOfParagraphAndModifySelection:(nullable id)sender;
    // - (void)moveToEndOfDocumentAndModifySelection:(nullable id)sender;
    // - (void)moveToBeginningOfDocumentAndModifySelection:(nullable id)sender;
    // - (void)pageDownAndModifySelection:(nullable id)sender;
    // - (void)pageUpAndModifySelection:(nullable id)sender;
    // - (void)moveParagraphForwardAndModifySelection:(nullable id)sender;
    // - (void)moveParagraphBackwardAndModifySelection:(nullable id)sender;

    // - (void)moveWordRight:(nullable id)sender;
    // - (void)moveWordLeft:(nullable id)sender;
    // - (void)moveRightAndModifySelection:(nullable id)sender;
    // - (void)moveLeftAndModifySelection:(nullable id)sender;
    // - (void)moveWordRightAndModifySelection:(nullable id)sender;
    // - (void)moveWordLeftAndModifySelection:(nullable id)sender;

    // - (void)moveToLeftEndOfLine:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn move_to_left_end_of_line(&self, event: &NSEventRef) {
        unsafe { msg_send![self, moveToLeftEndOfLine: event] }
    }
    // - (void)moveToRightEndOfLine:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn move_to_right_end_of_line(&self, event: &NSEventRef) {
        unsafe { msg_send![self, moveToRightEndOfLine: event] }
    }
    // - (void)moveToLeftEndOfLineAndModifySelection:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn move_to_left_end_of_line_and_modify_selection(&self, event: &NSEventRef) {
        unsafe { msg_send![self, moveToLeftEndOfLineAndModifySelection: event] }
    }
    // - (void)moveToRightEndOfLineAndModifySelection:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn move_to_right_end_of_line_and_modify_selection(&self, event: &NSEventRef) {
        unsafe { msg_send![self, moveToRightEndOfLineAndModifySelection: event] }
    }

    // - (void)scrollPageUp:(nullable id)sender;
    // - (void)scrollPageDown:(nullable id)sender;
    // - (void)scrollLineUp:(nullable id)sender;
    // - (void)scrollLineDown:(nullable id)sender;

    // - (void)scrollToBeginningOfDocument:(nullable id)sender;
    // - (void)scrollToEndOfDocument:(nullable id)sender;

    //     /* Graphical Element transposition */
    // - (void)transpose:(nullable id)sender;
    // - (void)transposeWords:(nullable id)sender;

    //     /* Selections */
    // - (void)selectAll:(nullable id)sender;
    // - (void)selectParagraph:(nullable id)sender;
    // - (void)selectLine:(nullable id)sender;
    // - (void)selectWord:(nullable id)sender;

    //     /* Insertions and Indentations */
    // - (void)indent:(nullable id)sender;
    // - (void)insertTab:(nullable id)sender;
    // - (void)insertBacktab:(nullable id)sender;
    // - (void)insertNewline:(nullable id)sender;
    // - (void)insertParagraphSeparator:(nullable id)sender;
    // - (void)insertNewlineIgnoringFieldEditor:(nullable id)sender;
    // - (void)insertTabIgnoringFieldEditor:(nullable id)sender;
    // - (void)insertLineBreak:(nullable id)sender;
    // - (void)insertContainerBreak:(nullable id)sender;
    // - (void)insertSingleQuoteIgnoringSubstitution:(nullable id)sender API_AVAILABLE(macos(10.5));
    pub fn insert_single_quote_ignoring_substitution(&self, event: &NSEventRef) {
        unsafe { msg_send![self, insertSingleQuoteIgnoringSubstitution: event] }
    }
    // - (void)insertDoubleQuoteIgnoringSubstitution:(nullable id)sender API_AVAILABLE(macos(10.5));
    pub fn insert_double_quote_ignoring_substitution(&self, event: &NSEventRef) {
        unsafe { msg_send![self, insertDoubleQuoteIgnoringSubstitution: event] }
    }

    //     /* Case changes */
    // - (void)changeCaseOfLetter:(nullable id)sender;
    // - (void)uppercaseWord:(nullable id)sender;
    // - (void)lowercaseWord:(nullable id)sender;
    // - (void)capitalizeWord:(nullable id)sender;

    //     /* Deletions */
    // - (void)deleteForward:(nullable id)sender;
    // - (void)deleteBackward:(nullable id)sender;
    // - (void)deleteBackwardByDecomposingPreviousCharacter:(nullable id)sender;
    // - (void)deleteWordForward:(nullable id)sender;
    // - (void)deleteWordBackward:(nullable id)sender;
    // - (void)deleteToBeginningOfLine:(nullable id)sender;
    // - (void)deleteToEndOfLine:(nullable id)sender;
    // - (void)deleteToBeginningOfParagraph:(nullable id)sender;
    // - (void)deleteToEndOfParagraph:(nullable id)sender;

    // - (void)yank:(nullable id)sender;

    //     /* Completion */
    // - (void)complete:(nullable id)sender;

    //     /* Mark/Point manipulation */
    // - (void)setMark:(nullable id)sender;
    // - (void)deleteToMark:(nullable id)sender;
    // - (void)selectToMark:(nullable id)sender;
    // - (void)swapWithMark:(nullable id)sender;

    //     /* Cancellation */
    // - (void)cancelOperation:(nullable id)sender;

    //     /* Writing Direction */
    // - (void)makeBaseWritingDirectionNatural:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn make_base_writing_direction_natural(&self, event: &NSEventRef) {
        unsafe { msg_send![self, makeBaseWritingDirectionNatural: event] }
    }
    // - (void)makeBaseWritingDirectionLeftToRight:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn make_base_writing_direction_left_to_right(&self, event: &NSEventRef) {
        unsafe { msg_send![self, makeBaseWritingDirectionLeftToRight: event] }
    }
    // - (void)makeBaseWritingDirectionRightToLeft:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn make_base_writing_direction_right_to_left(&self, event: &NSEventRef) {
        unsafe { msg_send![self, makeBaseWritingDirectionRightToLeft: event] }
    }

    // - (void)makeTextWritingDirectionNatural:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn make_text_writing_direction_natural(&self, event: &NSEventRef) {
        unsafe { msg_send![self, makeTextWritingDirectionNatural: event] }
    }
    // - (void)makeTextWritingDirectionLeftToRight:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn make_text_writing_direction_left_to_right(&self, event: &NSEventRef) {
        unsafe { msg_send![self, makeTextWritingDirectionLeftToRight: event] }
    }
    // - (void)makeTextWritingDirectionRightToLeft:(nullable id)sender API_AVAILABLE(macos(10.6));
    pub fn make_text_writing_direction_right_to_left(&self, event: &NSEventRef) {
        unsafe { msg_send![self, makeTextWritingDirectionRightToLeft: event] }
    }

    //    /* Quick Look */
    // /* Perform a Quick Look on the text cursor position, selection, or whatever is appropriate for your view. If there are no Quick Look items, then call [[self nextResponder] tryToPerform:_cmd with:sender]; to pass the request up the responder chain. Eventually AppKit will attempt to perform a dictionary look up. Also see quickLookWithEvent: above.
    // */
    // - (void)quickLookPreviewItems:(nullable id)sender API_AVAILABLE(macos(10.8));
    pub fn quick_look_preview_items(&self, event: &NSEventRef) {
        unsafe { msg_send![self, quickLookPreviewItems: event] }
    }

    // @end

    // @interface NSResponder (NSStandardKeyBindingMethods) <NSStandardKeyBindingResponding>
    // @end

    // @interface NSResponder(NSUndoSupport)
    // @property (nullable, readonly, strong) NSUndoManager *undoManager;
    // @end

    // @interface NSResponder (NSControlEditingSupport)

    // /* This is a responder chain method to allow controls to determine when they should become first responder or not. Some controls, such as NSTextField, should only become first responder when the enclosing NSTableView/NSBrowser indicates that the view can begin editing. It is up to the particular control that wants to be validated to call this method in its -mouseDown: (or other time) to determine if it should attempt to become the first responder or not. The default implementation returns YES when there is no -nextResponder, otherwise, it is forwarded up the responder chain. NSTableView/NSBrowser implements this to only allow first responder status if the responder is a view in a selected row. It also delays the first responder assignment if a doubleAction needs to (possibly) be sent. 'event' may be nil if there is no applicable event.
    // **/
    // - (BOOL)validateProposedFirstResponder:(NSResponder *)responder forEvent:(nullable NSEvent *)event API_AVAILABLE(macos(10.7));
    pub fn validate_proposed_first_responder(&self, event: &NSEventRef) {
        unsafe { msg_send![self, validateProposedFirstResponder: event] }
    }

    // @end

    // @interface NSResponder(NSErrorPresentation)

    // /* Present an error alert to the user, as a document-modal panel. When the user has dismissed the alert and any recovery possible for the error and chosen by the user has been attempted, send the selected message to the specified delegate. The method selected by didPresentSelector must have the same signature as:

    //     - (void)didPresentErrorWithRecovery:(BOOL)didRecover contextInfo:(void *)contextInfo;

    // The default implementation of this method always invokes [self willPresentError:error] to give subclassers an opportunity to customize error presentation. It then forwards the message, passing the customized error, to the next responder or, if there is no next responder, NSApp. NSApplication's override of this method invokes [[NSAlert alertWithError:errorToPresent] beginSheetModalForWindow:window modalDelegate:self didEndSelector:selectorForAPrivateMethod contextInfo:privateContextInfo]. When the user has dismissed the alert, the error's recovery attempter is sent an -attemptRecoveryFromError:optionIndex:delegate:didRecoverSelector:contextInfo: message, if the error had recovery options and a recovery delegate.

    // Errors for which ([[error domain] isEqualToString:NSCocoaErrorDomain] && [error code]==NSUserCancelledError) are a special case,  because they do not actually represent errors and should not be presented as such to the user. NSApplication's override of this method does not present an alert to the user for these kinds of errors. Instead it merely invokes the delegate specifying didRecover==NO.

    // Between the responder chain in a typical application and various overrides of this method in AppKit classes, objects are given the opportunity to present errors in orders like these:

    //     For windows owned by documents:
    //     view -> superviews -> window -> window controller -> document -> document controller -> application

    //     For windows that have window controllers but aren't associated with documents:
    //     view -> superviews -> window -> window controller -> application

    //     For windows that have no window controller at all:
    //     view -> superviews -> window -> application

    // You can invoke this method to present error alert sheets. For example, Cocoa's own -[NSDocument saveToURL:ofType:forSaveOperation:delegate:didSaveSelector:contextInfo:] invokes this method when it's just invoked -saveToURL:ofType:forSaveOperation:completionHandler: and that method has signalled an error.

    // You probably shouldn't override this method, because you have no way of reliably predicting whether this method vs. -presentError will be invoked for any particular error. You should instead override the -willPresentError: method described below.
    // */
    // - (void)presentError:(NSError *)error modalForWindow:(NSWindow *)window delegate:(nullable id)delegate didPresentSelector:(nullable SEL)didPresentSelector contextInfo:(nullable void *)contextInfo;

    // /* Present an error alert to the user, as an application-modal panel, and return YES if error recovery was done, NO otherwise. This method behaves much like the previous one except it does not return until the user has dismissed the alert and, if the error had recovery options and a recovery delegate, the error's recovery delegate has been sent an -attemptRecoveryFromError:optionIndex: message.

    // You can invoke this method to present error alert dialog boxes. For example, Cocoa's own [NSDocumentController openDocument:] invokes this method when document opening fails.

    // You probably shouldn't override this method, because you have no way of reliably predicting whether this method vs. -presentError:modalForWindow:delegate:didPresentSelector:contextInfo: will be invoked for any particular error. You should instead override the -willPresentError: method described below.
    // */
    // - (BOOL)presentError:(NSError *)error;

    // /* Given that the receiver is about to present an error (perhaps by just forwarding it to the next responder), return the error that should actually be presented. The default implementation of this method merely returns the passed-in error.

    // You can override this method to customize the presentation of errors by examining the passed-in error and if, for example, its localized description or recovery information is unsuitably generic, returning a more specific one. When you override this method always check the NSError's domain and code to discriminate between errors whose presentation you want to customize and those you don't. For those you don't just return [super willPresentError:error]. Don't make decisions based on the NSError's localized description, recovery suggestion, or recovery options because it's usually not a good idea to try to parse localized text.
    // */
    // - (NSError *)willPresentError:(NSError *)error;

    // @end

    // @interface NSResponder(NSTextFinderSupport)

    // - (void)performTextFinderAction:(nullable id)sender API_AVAILABLE(macos(10.7));
    pub fn perform_text_finder_action(&self, event: &NSEventRef) {
        unsafe { msg_send![self, performTextFinderAction: event] }
    }

    // @end

    // @interface NSResponder(NSWindowTabbing)

    // /* For automatic window tabbing: This method can be implemented in the responder chain. It is automatically called for tabbed windows when the plus button is clicked, and the next window that is created and shown will be placed in a tab. This can be implemented in an NSDocumentController subclass, or somewhere in the responder chain starting at NSWindow (such as NSWindow, the window delegate, the windowController, the NSApp delegate, etc. A plus button on tabbed windows will only be shown if this method exists in the responder chain.
    // **/
    // - (IBAction)newWindowForTab:(nullable id)sender API_AVAILABLE(macos(10.12));
    pub fn new_window_for_tab(&self, event: &NSEventRef) {
        unsafe { msg_send![self, newWindowForTab: event] }
    }

    // @end

    // @interface NSResponder(NSDeprecated)

    // - (BOOL)performMnemonic:(NSString *)string API_DEPRECATED("This has always returned NO and had no effect on macOS", macos(10.0,10.8));

    // @end

    // API_UNAVAILABLE_END
    // NS_ASSUME_NONNULL_END
}
