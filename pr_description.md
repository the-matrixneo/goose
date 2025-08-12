# Refactor Context Summarization UI in Desktop Application

## Summary

This PR refactors the context summarization feature in the Goose desktop application by relocating the summarization button from the main chat input area to the context window alert system. The changes improve UI consistency and provide a more intuitive user experience by integrating the summarization action directly with context window status indicators.

## Changes Overview

### 🎯 Key Changes
- **Moved summarization button** from standalone component to integrated alert system
- **Added confirmation dialog** for manual context compaction with clear user messaging
- **Enhanced alert system** to support action buttons with custom icons
- **Improved code organization** by consolidating context management logic

## Technical Implementation

### Modified Components

#### 1. **ChatInput.tsx** (`ui/desktop/src/components/ChatInput.tsx`)
- Removed the standalone `ManualCompactButton` component
- Integrated summarization functionality directly into the alert system
- Added `showSummarizeButton`, `onSummarize`, and `summarizeIcon` props to context window alerts
- Simplified the bottom menu by removing redundant UI elements

#### 2. **AlertBox.tsx** (`ui/desktop/src/components/alerts/AlertBox.tsx`)
- Extended alert component to support action buttons
- Added "Summarize now" button with icon support for context window alerts
- Implemented proper event handling to prevent propagation issues

#### 3. **Alert Types** (`ui/desktop/src/components/alerts/types.ts`)
- Extended `Alert` interface with new optional properties:
  - `showSummarizeButton?: boolean`
  - `onSummarize?: () => void`
  - `summarizeIcon?: React.ReactNode`

#### 4. **ChatContextManager.tsx** (`ui/desktop/src/components/context_management/ChatContextManager.tsx`)
- Added confirmation dialog for manual context compaction
- Implemented dialog state management with `isConfirmationOpen` and `pendingCompactionData`
- Created `handleCompactionConfirm` and `handleCompactionCancel` handlers
- Improved user communication about the compaction process

#### 5. **ManualCompactButton.tsx** (`ui/desktop/src/components/context_management/ManualCompactButton.tsx`)
- Updated comment to reflect the component's deprecated status
- Component remains for backward compatibility but is no longer used

## User Experience Improvements

### Before
- Summarization button was a separate UI element in the chat input area
- Less contextual connection between token usage and summarization action
- No confirmation dialog for the compaction action

### After
- Summarization is now integrated with the context window status indicator
- Clear visual connection between token usage and available actions
- Confirmation dialog provides clear explanation of what will happen
- More consistent UI with fewer standalone buttons

## Impact Analysis

### Areas Affected
- **Chat Interface**: Modified bottom menu layout and alert display
- **Context Management**: Enhanced user flow for manual summarization
- **Alert System**: Extended to support interactive elements

### Backward Compatibility
- No breaking changes to existing functionality
- ManualCompactButton component retained but unused
- All existing context management features remain functional

## Testing Considerations

### Manual Testing Required
1. **Alert Display**: Verify summarization button appears in context window alerts
2. **Button Functionality**: Test "Summarize now" button triggers confirmation dialog
3. **Dialog Flow**: Confirm both "Cancel" and "Compact Conversation" actions work correctly
4. **Token Thresholds**: Test alert behavior at different token usage levels (50%, 75%, 90%)
5. **Icon Rendering**: Verify ScrollText icon displays correctly in alerts

### Edge Cases to Test
- Empty conversation state (no messages)
- Very long conversations approaching token limit
- Rapid clicking of summarization button
- Dialog dismissal via ESC key or backdrop click

## Migration Notes

No database migrations or configuration changes required. This is a pure UI refactoring that maintains all existing functionality while improving the user experience.

## Related Issues

This refactoring addresses user feedback about UI clutter and improves the discoverability of the context summarization feature by placing it in a more contextually relevant location.

## Screenshots/Demo

The summarization action is now integrated directly into the context window status alert, providing a cleaner and more intuitive interface for users to manage their conversation context.
