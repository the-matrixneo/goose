---
sidebar_position: 2
title: Updating Goose
sidebar_label: Updating Goose
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import DesktopUpdateInstructions from '@site/src/components/DesktopUpdateInstructions';
import CLIUpdateInstructions from '@site/src/components/CLIUpdateInstructions';
import WSLUpdateInstructions from '@site/src/components/WSLUpdateInstructions';

The Goose CLI and desktop apps are under active and continuous development. To get the newest features and fixes, you should periodically update your Goose client using the following instructions.

<Tabs>
  <TabItem value="mac" label="macOS" default>
    <Tabs groupId="interface">
      <TabItem value="ui" label="Goose Desktop" default>
        <DesktopUpdateInstructions os="mac" />
      </TabItem>
      <TabItem value="cli" label="Goose CLI">
        <CLIUpdateInstructions />
      </TabItem>
    </Tabs>
  </TabItem>

  <TabItem value="linux" label="Linux">
    <Tabs groupId="interface">
      <TabItem value="ui" label="Goose Desktop" default>
        <DesktopUpdateInstructions os="linux" />
      </TabItem>
      <TabItem value="cli" label="Goose CLI">
        <CLIUpdateInstructions />
      </TabItem>
    </Tabs>
  </TabItem>

  <TabItem value="windows" label="Windows">
    <Tabs groupId="interface">
      <TabItem value="ui" label="Goose Desktop" default>
        <DesktopUpdateInstructions os="windows" />
      </TabItem>
      <TabItem value="cli" label="Goose CLI">
        <CLIUpdateInstructions />
        <WSLUpdateInstructions />
      </TabItem>
    </Tabs>
  </TabItem>
</Tabs>