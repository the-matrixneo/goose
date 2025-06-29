import { View, ViewOptions } from '../../../App';
import { ModeSection } from '../mode/ModeSection';
import { ToolSelectionStrategySection } from '../tool_selection_strategy/ToolSelectionStrategySection';
import { ResponseStylesSection } from '../response_styles/ResponseStylesSection';

interface ChatSettingsSectionProps {
  setView: (view: View, viewOptions?: ViewOptions) => void;
}

export default function ChatSettingsSection({ setView }: ChatSettingsSectionProps) {
  return (
    <div className="space-y-8">
      <section>
        <h1 className="text-xl text-text-default mb-4">Chat Experience</h1>
        <p className="text-sm text-text-muted mb-8">
          Configure how Goose interacts with you and responds to your queries
        </p>
      </section>

      <ModeSection setView={setView} />

      <ResponseStylesSection />

      <ToolSelectionStrategySection setView={setView} />
    </div>
  );
}
