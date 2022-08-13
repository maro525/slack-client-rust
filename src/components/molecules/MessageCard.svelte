<script lang="ts">
  import { Tile } from "carbon-components-svelte";
  import type { Message } from "~/model/search-messages";

  export let message: Message;
</script>

<Tile>
  <b>{message.user_name}</b>
  <p>{message.channel_name}</p>
  <pre style="white-space: pre-wrap">{message.text}</pre>
</Tile>
