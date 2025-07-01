import { View, ViewOptions } from '../../../App';
import { ModeSection } from '../mode/ModeSection';
import { ToolSelectionStrategySection } from '../tool_selection_strategy/ToolSelectionStrategySection';
import { ResponseStylesSection } from '../response_styles/ResponseStylesSection';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../ui/card';

interface ChatSettingsSectionProps {
  setView: (view: View, viewOptions?: ViewOptions) => void;
}

export default function ChatSettingsSection({ setView }: ChatSettingsSectionProps) {
  return (
    <div className="space-y-4 pr-4 pb-8 mt-1">
      <Card className="pb-2 rounded-lg">
        <CardHeader className="pb-0">
          <CardTitle className="">Mode</CardTitle>
          <CardDescription>Configure how Goose interacts with tools and extensions</CardDescription>
        </CardHeader>
        <CardContent className="px-2">
          <ModeSection setView={setView} />
        </CardContent>
      </Card>

      <Card className="pb-2 rounded-lg">
        <CardHeader className="pb-0">
          <CardTitle className="">Response Styles</CardTitle>
          <CardDescription>Choose how Goose should format and style its responses</CardDescription>
        </CardHeader>
        <CardContent className="px-2">
          <ResponseStylesSection />
        </CardContent>
      </Card>

      <Card className="pb-2 rounded-lg">
        <CardHeader className="pb-0">
          <CardTitle className="">Tool Selection Strategy (preview)</CardTitle>
          <CardDescription>
            Configure how Goose selects tools for your requests. Recommended when many extensions
            are enabled. Available only with Claude models served on Databricks for now.
          </CardDescription>
        </CardHeader>
        <CardContent className="px-2">
          <ToolSelectionStrategySection setView={setView} />
        </CardContent>
      </Card>
    </div>
  );
}
